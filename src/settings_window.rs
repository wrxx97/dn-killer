use anyhow::{Context as _, Result, ensure};
use gpui::{
    App, Context, Entity, IntoElement, Render, SharedString, Subscription, WeakEntity, Window,
    WindowHandle, div, prelude::*, px, rgb, size,
};
use gpui_component::Root;
use gpui_component::input::{InputEvent, InputState};
use gpui_component::switch::Switch;

use crate::config::Settings;
use crate::forms::{capture_hotkey, field, input, key_label};
use crate::hotkey::{HOTKEY_PLACEHOLDER, Hotkey};
use crate::overlay::Overlay;
use crate::popup;

#[derive(Clone, Copy, PartialEq, Eq)]
enum SettingInput {
    ProcessName,
    ExitHotkey,
    VoiceLead,
    VoiceTemplate,
    Opacity,
    FontColor,
}

const INPUTS: [SettingInput; 6] = [
    SettingInput::ProcessName,
    SettingInput::ExitHotkey,
    SettingInput::VoiceLead,
    SettingInput::VoiceTemplate,
    SettingInput::Opacity,
    SettingInput::FontColor,
];

impl SettingInput {
    fn label(self) -> &'static str {
        match self {
            Self::ProcessName => "退出进程名",
            Self::ExitHotkey => "退出快捷键",
            Self::VoiceLead => "提前提醒（秒）",
            Self::VoiceTemplate => "提示模板（{title} 任务名称，{left} 剩余秒数）",
            Self::Opacity => "背景不透明度（0–100）",
            Self::FontColor => "文字颜色（#RRGGBB）",
        }
    }

    fn placeholder(self) -> &'static str {
        match self {
            Self::ProcessName => "DragonNest_x64.exe",
            Self::ExitHotkey => HOTKEY_PLACEHOLDER,
            Self::VoiceLead => "10",
            Self::VoiceTemplate => "{title}还有{left}秒",
            Self::Opacity => "50",
            Self::FontColor => "#FFFFFF",
        }
    }

    fn current(self, settings: &Settings) -> String {
        match self {
            Self::ProcessName => settings.process_name.clone(),
            Self::ExitHotkey => key_label(settings.exit_hotkey),
            Self::VoiceLead => settings.voice_lead.to_string(),
            Self::VoiceTemplate => settings.voice_template.clone(),
            Self::Opacity => settings.opacity.to_string(),
            Self::FontColor => format!("#{:06X}", settings.font_color),
        }
    }

    fn update(self, settings: &mut Settings, text: &str) -> Result<()> {
        let text = text.trim();
        match self {
            Self::ProcessName => settings.process_name = text.into(),
            Self::ExitHotkey => settings.exit_hotkey = Hotkey::parse_optional(text)?,
            Self::VoiceLead => settings.voice_lead = text.parse().context("提醒时间须为整数秒")?,
            Self::VoiceTemplate => settings.voice_template = text.into(),
            Self::Opacity => settings.opacity = text.parse().context("不透明度须为 0–100")?,
            Self::FontColor => {
                let hex = text.strip_prefix('#').unwrap_or(text);
                ensure!(hex.len() == 6, "请输入六位十六进制颜色");
                settings.font_color =
                    u32::from_str_radix(hex, 16).context("请输入六位十六进制颜色")?;
            }
        }
        Ok(())
    }
}

struct SettingField {
    input_kind: SettingInput,
    input: Entity<InputState>,
    error: Option<SharedString>,
}

pub struct SettingsWindow {
    overlay: WeakEntity<Overlay>,
    fields: [SettingField; INPUTS.len()],
    _subscriptions: Vec<Subscription>,
    voice_enabled: bool,
    error: Option<SharedString>,
}

impl SettingsWindow {
    pub fn open(
        owner: &Window,
        overlay: WeakEntity<Overlay>,
        settings: &Settings,
        cx: &mut App,
    ) -> Result<WindowHandle<Root>> {
        popup::open(
            owner,
            "DN Killer 设置",
            size(px(432.), px(510.)),
            popup::Layout::Resizable {
                minimum: size(px(340.), px(250.)),
            },
            cx,
            |error, window, cx| Self::new(overlay, settings, error, window, cx),
        )
    }

    fn new(
        overlay: WeakEntity<Overlay>,
        settings: &Settings,
        error: Option<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let fields = INPUTS.map(|input_kind| SettingField {
            input: input(
                input_kind.current(settings),
                input_kind.placeholder(),
                window,
                cx,
            ),
            input_kind,
            error: None,
        });
        let mut subscriptions: Vec<_> = fields
            .iter()
            .map(|field| {
                let input_kind = field.input_kind;
                cx.subscribe(&field.input, move |view, input, event: &InputEvent, cx| {
                    if matches!(event, InputEvent::Change) {
                        let text = input.read(cx).value();
                        view.commit(input_kind, &text, cx);
                    }
                })
            })
            .collect();
        if let Some(field) = fields
            .iter()
            .find(|field| field.input_kind == SettingInput::ExitHotkey)
        {
            let view = cx.weak_entity();
            subscriptions.push(capture_hotkey(
                &field.input,
                move |label, cx| {
                    let _ = view.update(cx, |view, cx| {
                        view.commit(SettingInput::ExitHotkey, label, cx);
                    });
                },
                cx,
            ));
        }
        Self {
            overlay,
            fields,
            _subscriptions: subscriptions,
            voice_enabled: settings.voice_enabled,
            error,
        }
    }

    fn apply(
        &self,
        change: impl FnOnce(&mut Settings) -> Result<()>,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        self.overlay
            .update(cx, |overlay, cx| overlay.change_settings(change, cx))?
    }

    fn commit(&mut self, input_kind: SettingInput, text: &str, cx: &mut Context<Self>) {
        let result = self.apply(|settings| input_kind.update(settings, text), cx);
        if let Some(field) = self
            .fields
            .iter_mut()
            .find(|field| field.input_kind == input_kind)
        {
            field.error = result.err().map(|error| format!("{error:#}").into());
        }
        cx.notify();
    }
}

impl Render for SettingsWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("settings-body")
            .size_full()
            .overflow_y_scroll()
            .child(
                div()
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        Switch::new("voice-enabled")
                            .label("语音提醒")
                            .checked(self.voice_enabled)
                            .on_change(cx.listener(|view, enabled: &bool, _, cx| {
                                let result = view.apply(
                                    |settings| {
                                        settings.voice_enabled = *enabled;
                                        Ok(())
                                    },
                                    cx,
                                );
                                if result.is_ok() {
                                    view.voice_enabled = *enabled;
                                }
                                view.error = result.err().map(|error| format!("{error:#}").into());
                                cx.notify();
                            })),
                    )
                    .children(
                        self.error
                            .as_ref()
                            .map(|error| div().text_color(rgb(0xffbdb5)).child(error.clone())),
                    )
                    .children(self.fields.iter().map(|entry| {
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(field(entry.input_kind.label(), &entry.input))
                            .children(entry.error.as_ref().map(|error| {
                                div()
                                    .text_size(px(12.))
                                    .text_color(rgb(0xffbdb5))
                                    .child(error.clone())
                            }))
                    })),
            )
    }
}
