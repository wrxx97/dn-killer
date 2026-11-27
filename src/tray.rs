use std::sync::Arc;

use anyhow::{Context, Result};
use async_channel::Sender;
use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::{Icon, MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};

pub enum TrayAction {
    Show,
    Quit,
}

pub fn create(actions: Sender<TrayAction>) -> Result<TrayIcon> {
    let rgba = image::load_from_memory(include_bytes!("../assets/32x32.png"))?.into_rgba8();
    let (width, height) = rgba.dimensions();
    let icon = Icon::from_rgba(rgba.into_raw(), width, height)?;
    let menu = Menu::new();
    menu.append(&MenuItem::with_id("show", "显示悬浮窗", true, None))?;
    menu.append(&MenuItem::with_id("quit", "退出 DN Killer", true, None))?;
    let tray = TrayIconBuilder::new()
        .with_icon(icon)
        .with_tooltip("DN Killer")
        .with_menu(Box::new(menu))
        .with_menu_on_left_click(false)
        .build()
        .context("创建系统托盘失败")?;

    let menu_actions = actions.clone();
    MenuEvent::set_event_handler(Some(Arc::new(move |event: MenuEvent| {
        let action = match event.id().as_ref() {
            "show" => TrayAction::Show,
            "quit" => TrayAction::Quit,
            _ => return,
        };
        let _ = menu_actions.try_send(action);
    })));
    TrayIconEvent::set_event_handler(Some(move |event| {
        if matches!(
            event,
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            }
        ) {
            let _ = actions.try_send(TrayAction::Show);
        }
    }));
    Ok(tray)
}
