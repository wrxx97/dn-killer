use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};

use anyhow::{Context as _, Result};
use async_channel::Receiver;
use gpui::{
    Context, Div, IntoElement, Pixels, Render, SharedString, Subscription, Task, Window,
    WindowControlArea, WindowHandle, div, prelude::*, px, relative, rgb, rgba, size,
};
use gpui_component::{Root, Sizable as _, button::Button};

use crate::config::{Config, ConfigStore, Settings};
use crate::countdown::{Countdown, ceil_seconds};
use crate::forms::{TaskForm, shortcut_from_keystroke};
use crate::hotkey::Hotkey;
use crate::icons::{ButtonIcon, button as icon_button};
use crate::keyboard::{KeyEvent, KeyboardListener};
use crate::settings_window::SettingsWindow;
use crate::speech::{self, Reminder};
use crate::task_editor::TaskEditor;
use crate::tray::{self, TrayAction};
use crate::{native_window, process};

const OVERLAY_PADDING: Pixels = px(12.);
const OVERLAY_GAP: Pixels = px(8.);

pub struct Overlay {
    _keyboard: Option<KeyboardListener>,
    _system_shortcut: Subscription,
    _key_events: Task<()>,
    _clock: Task<()>,
    _speech_events: Task<()>,
    _tray_events: Task<()>,
    _tray: Option<tray_icon::TrayIcon>,
    speech: Option<Sender<Reminder>>,
    store: Option<ConfigStore>,
    config: Config,
    countdowns: HashMap<u64, Countdown>,
    settings_window: Option<WindowHandle<Root>>,
    task_editor: Option<WindowHandle<Root>>,
    last_list_fit: Rc<Cell<Option<Pixels>>>,
    last_key: Option<KeyEvent>,
    notice: Option<Notice>,
    speech_error: Option<SharedString>,
    process_pending: bool,
    last_exit: Option<Instant>,
}

struct Notice {
    text: SharedString,
    is_error: bool,
}

impl Overlay {
    pub fn new(
        keyboard: Result<KeyboardListener>,
        events: Receiver<KeyEvent>,
        (store, config, config_error): (Option<ConfigStore>, Config, Option<String>),
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut error = config_error;
        let keyboard = keyboard
            .map_err(|cause| {
                error.get_or_insert(format!("键盘监听启动失败 · {cause:#}"));
            })
            .ok();
        let main_window = window.window_handle().window_id();
        let overlay = cx.weak_entity();
        let system_shortcut = cx.intercept_keystrokes(move |event, window, cx| {
            if window.window_handle().window_id() == main_window
                && shortcut_from_keystroke(&event.keystroke) == Some(Hotkey::ALT_F4)
                && overlay.upgrade().is_some_and(|view| {
                    let view = view.read(cx);
                    view._keyboard.is_some() && view.uses_hotkey(Hotkey::ALT_F4)
                })
            {
                // Raw Input triggers the bound action. Consume the normal close on this window.
                cx.stop_propagation();
            }
        });

        if let Err(cause) = native_window::set_topmost(window, config.settings.topmost) {
            error.get_or_insert(cause.to_string());
        }
        if let Err(cause) = native_window::install_drag_cursor(window) {
            error.get_or_insert(cause.to_string());
        }

        let (tray_actions, actions) = async_channel::unbounded();
        let tray = tray::create(tray_actions)
            .map_err(|cause| {
                error.get_or_insert(format!("系统托盘不可用 · {cause:#}"));
            })
            .ok();
        let tray_events = cx.spawn_in(window, async move |this, cx| {
            while let Ok(action) = actions.recv().await {
                if this
                    .update_in(cx, |view, window, cx| match action {
                        TrayAction::Show => {
                            if let Err(error) = native_window::show(window) {
                                view.error(error, cx);
                            }
                        }
                        TrayAction::Quit => window.remove_window(),
                    })
                    .is_err()
                {
                    break;
                }
            }
        });

        let keyboard_ready = keyboard.is_some();
        let key_events = cx.spawn(async move |this, cx| {
            while let Ok(event) = events.recv().await {
                if this.update(cx, |view, cx| view.on_key(event, cx)).is_err() {
                    return;
                }
            }
            if keyboard_ready {
                let _ = this.update(cx, |view, cx| view.status("后台键盘监听已停止", true, cx));
            }
        });

        let executor = cx.background_executor().clone();
        let clock = cx.spawn(async move |this, cx| {
            loop {
                executor.timer(Duration::from_millis(100)).await;
                if this.update(cx, |view, cx| view.tick(cx)).is_err() {
                    break;
                }
            }
        });

        let (speech_results, results) = async_channel::unbounded();
        let (speech, speech_error) = match speech::start(speech_results) {
            Ok(speech) => (Some(speech), None),
            Err(error) => (None, Some(format!("{error:#}").into())),
        };
        let speech_events = cx.spawn(async move |this, cx| {
            while let Ok(result) = results.recv().await {
                if this
                    .update(cx, |view, cx| {
                        view.speech_error = result.err().map(|error| format!("{error:#}").into());
                        cx.notify();
                    })
                    .is_err()
                {
                    break;
                }
            }
        });

        Self {
            _keyboard: keyboard,
            _system_shortcut: system_shortcut,
            _key_events: key_events,
            _clock: clock,
            _speech_events: speech_events,
            _tray_events: tray_events,
            _tray: tray,
            speech,
            store,
            config,
            countdowns: HashMap::new(),
            settings_window: None,
            task_editor: None,
            last_list_fit: Rc::default(),
            last_key: None,
            notice: error.map(|text| Notice {
                text: text.into(),
                is_error: true,
            }),
            speech_error,
            process_pending: false,
            last_exit: None,
        }
    }

    fn on_key(&mut self, event: KeyEvent, cx: &mut Context<Self>) {
        if !event.shortcut.is_function_key() && !self.uses_hotkey(event.shortcut) {
            return;
        }
        let editor_active = [self.settings_window, self.task_editor]
            .into_iter()
            .flatten()
            .any(|handle| {
                handle
                    .update(cx, |_, window, _| window.is_window_active())
                    .unwrap_or(false)
            });
        if editor_active {
            return;
        }
        self.last_key = Some(event);
        if event.pressed {
            if self.config.settings.exit_hotkey == Some(event.shortcut) {
                self.exit_process(event.received_at, cx);
            }
        } else {
            for task in &self.config.tasks {
                if task.hotkey == Some(event.shortcut) {
                    self.countdowns.insert(
                        task.id,
                        Countdown::new(event.received_at, Duration::from_secs(task.duration)),
                    );
                }
            }
        }
        cx.notify();
    }

    fn uses_hotkey(&self, shortcut: Hotkey) -> bool {
        self.config.settings.exit_hotkey == Some(shortcut)
            || self
                .config
                .tasks
                .iter()
                .any(|task| task.hotkey == Some(shortcut))
    }

    fn exit_process(&mut self, now: Instant, cx: &mut Context<Self>) {
        if self.process_pending
            || self
                .last_exit
                .is_some_and(|last| now.saturating_duration_since(last) < Duration::from_secs(2))
        {
            return;
        }
        self.last_exit = Some(now);
        self.process_pending = true;
        let name = self.config.settings.process_name.clone();
        self.status(format!("正在退出 {name}…"), false, cx);
        let result = cx.background_executor().spawn(async move {
            process::terminate(&name).map(|pid| format!("已退出 {name}（PID {pid}）"))
        });
        cx.spawn(async move |this, cx| {
            let result = result.await;
            let _ = this.update(cx, |view, cx| {
                view.process_pending = false;
                match result {
                    Ok(status) => view.status(status, false, cx),
                    Err(error) => view.error(error, cx),
                }
            });
        })
        .detach();
    }

    fn tick(&mut self, cx: &mut Context<Self>) {
        if self.countdowns.is_empty() {
            return;
        }
        let now = Instant::now();
        self.countdowns
            .retain(|_, countdown| countdown.deadline > now);
        if self.config.settings.voice_enabled
            && let Some(speech) = &self.speech
        {
            let settings = &self.config.settings;
            let lead = Duration::from_secs(settings.voice_lead);
            for task in &self.config.tasks {
                let Some(countdown) = self.countdowns.get_mut(&task.id) else {
                    continue;
                };
                let Some(seconds) = countdown.take_reminder(now, lead) else {
                    continue;
                };
                let reminder = Reminder {
                    text: reminder_text(&settings.voice_template, &task.title, seconds),
                    seconds,
                    deadline: countdown.deadline,
                };
                if speech.send(reminder).is_err() {
                    self.speech = None;
                    self.speech_error
                        .get_or_insert_with(|| "语音线程已停止".into());
                    break;
                }
            }
        }
        cx.notify();
    }

    fn open_task_editor(
        &mut self,
        id: Option<u64>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        if let Some(handle) = self.task_editor
            && let Ok(result) = handle.update(cx, |_, window, _| native_window::show(window))
        {
            return result;
        }
        let task = id
            .map(|id| {
                self.config
                    .tasks
                    .iter()
                    .find(|task| task.id == id)
                    .context("找不到该任务")
            })
            .transpose()?;
        let overlay = cx.weak_entity();
        self.task_editor = Some(TaskEditor::open(window, overlay, task, cx)?);
        Ok(())
    }

    fn open_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Result<()> {
        if let Some(handle) = self.settings_window
            && let Ok(result) = handle.update(cx, |_, window, _| native_window::show(window))
        {
            return result;
        }
        let overlay = cx.weak_entity();
        self.settings_window = Some(SettingsWindow::open(
            window,
            overlay,
            &self.config.settings,
            cx,
        )?);
        Ok(())
    }

    pub fn change_settings(
        &mut self,
        change: impl FnOnce(&mut Settings) -> Result<()>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let mut next = self.config.clone();
        change(&mut next.settings)?;
        if next.settings != self.config.settings {
            self.persist(next)?;
            cx.notify();
        }
        Ok(())
    }

    fn toggle_topmost(&mut self, window: &Window, cx: &mut Context<Self>) -> Result<()> {
        let previous = self.config.settings.topmost;
        native_window::set_topmost(window, !previous)?;
        let result = self.change_settings(
            |settings| {
                settings.topmost = !previous;
                Ok(())
            },
            cx,
        );
        if result.is_err() {
            let _ = native_window::set_topmost(window, previous);
        }
        result
    }

    pub fn save_task(&mut self, form: &TaskForm, cx: &mut Context<Self>) -> Result<()> {
        let mut next = self.config.clone();
        let id = form.id.map_or_else(|| next.next_task_id(), Ok)?;
        let task = form.definition(id, cx)?;
        let reset = if form.id.is_some() {
            let existing = next
                .tasks
                .iter_mut()
                .find(|task| task.id == id)
                .context("找不到该任务")?;
            let reset = existing.duration != task.duration;
            *existing = task;
            reset
        } else {
            next.tasks.push(task);
            false
        };
        self.persist(next)?;
        if reset {
            self.countdowns.remove(&id);
        }
        self.status("已保存", false, cx);
        Ok(())
    }

    pub fn delete_task(&mut self, id: u64, cx: &mut Context<Self>) -> Result<()> {
        let mut next = self.config.clone();
        next.tasks.retain(|task| task.id != id);
        self.persist(next)?;
        self.countdowns.remove(&id);
        self.status("任务已删除", false, cx);
        Ok(())
    }

    fn persist(&mut self, next: Config) -> Result<()> {
        self.store.as_ref().context("配置目录不可用")?.save(&next)?;
        self.config = next;
        Ok(())
    }

    fn status(&mut self, text: impl Into<SharedString>, is_error: bool, cx: &mut Context<Self>) {
        self.notice = Some(Notice {
            text: text.into(),
            is_error,
        });
        cx.notify();
    }

    fn error(&mut self, error: anyhow::Error, cx: &mut Context<Self>) {
        self.status(format!("{error:#}"), true, cx);
    }

    fn task_list(&self, cx: &mut Context<Self>) -> Div {
        let now = Instant::now();
        let mut tasks: Vec<_> = self.config.tasks.iter().collect();
        tasks.sort_by_key(|task| {
            self.countdowns
                .get(&task.id)
                .map_or((true, Duration::ZERO), |countdown| {
                    (false, countdown.remaining(now))
                })
        });
        let settings = &self.config.settings;
        let exit_action = (!settings.process_name.is_empty()).then(|| {
            let label = settings
                .exit_hotkey
                .map_or_else(|| "退出进程".into(), |key| format!("{key} 退出"));
            Button::new("exit-process")
                .with_size(px(24.))
                .label(label)
                .on_click(cx.listener(|view, _, _, cx| {
                    view.exit_process(Instant::now(), cx);
                }))
        });
        let target = if settings.process_name.is_empty() {
            "未配置退出进程"
        } else {
            &settings.process_name
        };
        div()
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .mb_1()
                    .child(div().flex_1().min_w_0().truncate().child(target.to_owned()))
                    .children(exit_action)
                    .child(
                        icon_button("new-task", ButtonIcon::Add, "新增任务").on_click(cx.listener(
                            |view, _, window, cx| {
                                if let Err(error) = view.open_task_editor(None, window, cx) {
                                    view.error(error, cx);
                                }
                            },
                        )),
                    ),
            )
            .when(tasks.is_empty(), |container| {
                container.child(div().py_2().child("暂无倒计时任务"))
            })
            .children(tasks.into_iter().map(|task| {
                let id = task.id;
                let active = self.countdowns.get(&id);
                let left = active
                    .map(|countdown| ceil_seconds(countdown.remaining(now)))
                    .unwrap_or(task.duration);
                let progress = active.map_or(0., |countdown| countdown.progress(now));
                let color = if progress > 0.5 {
                    0x85b5ff
                } else if progress > 0.2 {
                    0xffc468
                } else {
                    0xff8a83
                };
                let (play_icon, play_label) = if active.is_some() {
                    (ButtonIcon::Restart, "重启")
                } else {
                    (ButtonIcon::Start, "开始")
                };
                div()
                    .h(px(36.))
                    .flex()
                    .flex_col()
                    .rounded_md()
                    .overflow_hidden()
                    .bg(rgba(0x1118249e))
                    .child(
                        div()
                            .flex_1()
                            .min_h_0()
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .child(
                                div()
                                    .flex_1()
                                    .min_w_0()
                                    .truncate()
                                    .child(task.title.clone()),
                            )
                            .children(task.hotkey.map(|key| {
                                div()
                                    .rounded_sm()
                                    .px_1()
                                    .text_size(px(11.))
                                    .bg(rgba(0x33415b9e))
                                    .child(key.to_string())
                            }))
                            .child(
                                div()
                                    .w(px(62.))
                                    .flex_shrink_0()
                                    .flex()
                                    .justify_end()
                                    .font_family("Consolas")
                                    .text_size(px(13.))
                                    .child(clock_text(left)),
                            )
                            .child(icon_button(("start", id), play_icon, play_label).on_click(
                                cx.listener(move |view, _, _, cx| {
                                    if let Some(task) =
                                        view.config.tasks.iter().find(|task| task.id == id)
                                    {
                                        view.countdowns.insert(
                                            id,
                                            Countdown::new(
                                                Instant::now(),
                                                Duration::from_secs(task.duration),
                                            ),
                                        );
                                        cx.notify();
                                    }
                                }),
                            ))
                            .child(
                                icon_button(("edit", id), ButtonIcon::Edit, "编辑").on_click(
                                    cx.listener(move |view, _, window, cx| {
                                        if let Err(error) =
                                            view.open_task_editor(Some(id), window, cx)
                                        {
                                            view.error(error, cx);
                                        }
                                    }),
                                ),
                            )
                            .child(
                                icon_button(("delete", id), ButtonIcon::Delete, "删除任务")
                                    .on_click(cx.listener(move |view, _, _, cx| {
                                        if let Err(error) = view.delete_task(id, cx) {
                                            view.error(error, cx);
                                        }
                                    })),
                            ),
                    )
                    .child(
                        div()
                            .w_full()
                            .h(px(2.))
                            .bg(rgb(0x33415b))
                            .child(div().h_full().w(relative(progress)).bg(rgb(color))),
                    )
            }))
    }
}

impl Render for Overlay {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let page = self.task_list(cx);
        let pinned = self.config.settings.topmost;
        let pin_label = if pinned { "取消置顶" } else { "置顶" };
        let toolbar = div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .gap_2()
            .child(
                div()
                    .id("drag-area")
                    .flex_1()
                    .py_1()
                    .text_size(px(15.))
                    .child("DN Killer")
                    .window_control_area(WindowControlArea::Drag),
            )
            .child(
                icon_button("settings", ButtonIcon::Settings, "设置").on_click(cx.listener(
                    |view, _, window, cx| {
                        if let Err(error) = view.open_settings(window, cx) {
                            view.error(error, cx);
                        }
                    },
                )),
            )
            .child(
                icon_button("topmost", ButtonIcon::Pin(pinned), pin_label).on_click(cx.listener(
                    |view, _, window, cx| {
                        if let Err(error) = view.toggle_topmost(window, cx) {
                            view.error(error, cx);
                        }
                    },
                )),
            )
            .child(
                icon_button("minimize", ButtonIcon::Minimize, "最小化")
                    .on_click(|_, window, _| window.minimize_window()),
            )
            .child(
                icon_button("close", ButtonIcon::Close, "退出应用")
                    .on_click(|_, window, _| window.remove_window()),
            );
        let key_status = self.last_key.map(|event| {
            let action = if event.pressed { "按下" } else { "松开" };
            let source = if event.background { "后台" } else { "前台" };
            format!("{} {action} · {source}", event.shortcut)
        });
        let alpha = u32::from(self.config.settings.opacity) * 255 / 100;
        let speech_error = self
            .speech_error
            .as_ref()
            .filter(|_| self.config.settings.voice_enabled);
        let list_height = Rc::new(Cell::new(px(0.)));
        let child_height = list_height.clone();
        let last_fit = self.last_list_fit.clone();
        div()
            .on_children_prepainted(move |children, window, cx| {
                if window.is_maximized() {
                    last_fit.set(None);
                    return;
                }
                let [toolbar, _, footer @ ..] = children.as_slice() else {
                    return;
                };
                let limit = window.display(cx).map_or(px(520.), |display| {
                    display.visible_bounds().size.height * 0.75
                });
                let height = fit_height(
                    toolbar.size.height,
                    list_height.get(),
                    footer.first().map(|bounds| bounds.size.height),
                    limit,
                );
                if last_fit.replace(Some(height)) != Some(height)
                    && window.viewport_size().height != height
                {
                    window.defer(cx, move |window, _| {
                        window.resize(size(window.viewport_size().width, height));
                    });
                }
            })
            .size_full()
            .overflow_hidden()
            .flex()
            .flex_col()
            .gap(OVERLAY_GAP)
            .p(OVERLAY_PADDING)
            .bg(rgba(0x11182400 | alpha))
            .text_color(rgb(self.config.settings.font_color))
            .text_size(px(13.))
            .child(toolbar)
            .child(
                div()
                    .on_children_prepainted(move |children, _, _| {
                        if let Some(content) = children.first() {
                            child_height.set(content.size.height);
                        }
                    })
                    .id("task-list")
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .flex_col()
                    .overflow_y_scroll()
                    .child(page.flex_shrink_0()),
            )
            .children(
                (self.notice.is_some() || speech_error.is_some() || key_status.is_some()).then(
                    || {
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .flex_shrink_0()
                            .text_size(px(12.))
                            .children(self.notice.as_ref().map(|notice| {
                                let color = if notice.is_error { 0xffbdb5 } else { 0xc9f4dc };
                                div().text_color(rgb(color)).child(notice.text.clone())
                            }))
                            .children(
                                speech_error.map(|error| {
                                    div().text_color(rgb(0xffbdb5)).child(error.clone())
                                }),
                            )
                            .children(key_status)
                    },
                ),
            )
    }
}

fn fit_height(toolbar: Pixels, list: Pixels, footer: Option<Pixels>, limit: Pixels) -> Pixels {
    let mut height = OVERLAY_PADDING * 2. + OVERLAY_GAP + toolbar + list + px(2.);
    if let Some(footer) = footer {
        height += OVERLAY_GAP + footer;
    }
    height.min(limit).max(px(130.)).ceil()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_fit_includes_full_content_and_wrapped_status_before_limiting_height() {
        assert_eq!(
            fit_height(px(28.), px(104.), Some(px(36.)), px(600.)),
            px(210.)
        );
        assert_eq!(
            fit_height(px(28.), px(104.), Some(px(36.)), px(200.)),
            px(200.)
        );
    }
}

fn clock_text(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds / 60) % 60;
    let seconds = seconds % 60;
    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes:02}:{seconds:02}")
    }
}

pub fn list_height(task_count: usize) -> Pixels {
    px(110. + 40. * task_count.clamp(1, 6) as f32)
}

pub fn load_config() -> (Option<ConfigStore>, Config, Option<String>) {
    match ConfigStore::new() {
        Ok(store) => match store.load() {
            Ok(config) => (Some(store), config, None),
            Err(error) => {
                let mut config = Config::default();
                config.settings.exit_hotkey = None;
                (
                    Some(store),
                    config,
                    Some(format!("配置读取失败，已使用默认值 · {error:#}")),
                )
            }
        },
        Err(error) => (None, Config::default(), Some(format!("{error:#}"))),
    }
}

fn reminder_text(template: &str, title: &str, seconds: u64) -> String {
    let seconds = seconds.to_string();
    let mut rest = template;
    let mut text = String::new();
    while let Some(open) = rest.find('{') {
        text.push_str(&rest[..open]);
        rest = &rest[open..];
        if let Some(suffix) = rest.strip_prefix("{title}") {
            text.push_str(title);
            rest = suffix;
        } else if let Some(suffix) = rest.strip_prefix("{left}") {
            text.push_str(&seconds);
            rest = suffix;
        } else {
            text.push('{');
            rest = &rest[1..];
        }
    }
    text.push_str(rest);
    text
}
