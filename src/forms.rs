use anyhow::{Context as _, Result};
use gpui::{
    App, AppContext, Div, Entity, Focusable, Keystroke, ParentElement, Styled, Subscription,
    Window, div,
};
use gpui_component::input::{Input, InputState};

use crate::config::TaskDefinition;
use crate::hotkey::{HOTKEY_PLACEHOLDER, Hotkey, Modifiers};

pub struct TaskForm {
    pub id: Option<u64>,
    pub title: Entity<InputState>,
    pub duration: Entity<InputState>,
    pub hotkey: Entity<InputState>,
    _hotkey_capture: Subscription,
}

impl TaskForm {
    pub fn new(task: Option<&TaskDefinition>, window: &mut Window, cx: &mut App) -> Self {
        let title = input(
            task.map_or("", |task| task.title.as_str()),
            "任务名称",
            window,
            cx,
        );
        let duration = input(
            task.map_or(15, |task| task.duration).to_string(),
            "15",
            window,
            cx,
        );
        let hotkey = input(
            key_label(task.and_then(|task| task.hotkey)),
            HOTKEY_PLACEHOLDER,
            window,
            cx,
        );
        let hotkey_capture = capture_hotkey(&hotkey, |_, _| {}, cx);
        title.update(cx, |state, cx| state.focus(window, cx));
        Self {
            id: task.map(|task| task.id),
            title,
            duration,
            hotkey,
            _hotkey_capture: hotkey_capture,
        }
    }

    pub fn definition(&self, id: u64, cx: &App) -> Result<TaskDefinition> {
        let task = TaskDefinition {
            id,
            title: self.title.read(cx).value().trim().into(),
            duration: self
                .duration
                .read(cx)
                .value()
                .trim()
                .parse()
                .context("时长须为整数秒")?,
            hotkey: Hotkey::parse_optional(&self.hotkey.read(cx).value())?,
        };
        task.validate()?;
        Ok(task)
    }
}

pub fn field(label: &'static str, state: &Entity<InputState>) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(label)
        .child(Input::new(state))
}

pub fn capture_hotkey(
    input: &Entity<InputState>,
    on_recorded: impl Fn(&str, &mut App) + 'static,
    cx: &mut App,
) -> Subscription {
    let input = input.clone();
    cx.intercept_keystrokes(move |event, window, cx| {
        if !input.focus_handle(cx).is_focused(window) {
            return;
        }
        let Some(shortcut) = shortcut_from_keystroke(&event.keystroke) else {
            return;
        };
        let label = shortcut.to_string();
        input.update(cx, |state, cx| state.set_value(label.clone(), window, cx));
        on_recorded(&label, cx);
        cx.stop_propagation();
    })
}

pub fn shortcut_from_keystroke(keystroke: &Keystroke) -> Option<Hotkey> {
    Hotkey::from_key_name(
        &keystroke.key,
        Modifiers {
            control: keystroke.modifiers.control,
            alt: keystroke.modifiers.alt,
            shift: keystroke.modifiers.shift,
            win: keystroke.modifiers.platform,
        },
    )
}

pub fn key_label(key: Option<Hotkey>) -> String {
    key.map(|key| key.to_string()).unwrap_or_default()
}

pub fn input(
    value: impl Into<gpui::SharedString>,
    placeholder: &'static str,
    window: &mut Window,
    cx: &mut App,
) -> Entity<InputState> {
    cx.new(|cx| {
        InputState::new(window, cx)
            .default_value(value)
            .placeholder(placeholder)
    })
}
