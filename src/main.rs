#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod countdown;
mod forms;
mod hotkey;
mod icons;
mod keyboard;
mod native_window;
mod overlay;
mod popup;
mod process;
mod settings_window;
mod single_instance;
mod speech;
mod task_editor;
mod tray;

use gpui::{
    AppContext, Bounds, Styled, TitlebarOptions, WindowBackgroundAppearance, WindowBounds,
    WindowOptions, px, rgba, size,
};

use crate::keyboard::KeyboardListener;
use crate::overlay::Overlay;

fn main() {
    let _instance = match single_instance::acquire() {
        Ok(Some(instance)) => instance,
        Ok(None) => return,
        Err(error) => {
            eprintln!("获取应用实例锁失败: {error}");
            return;
        }
    };

    gpui_platform::application()
        .with_assets(gpui_kit_assets::Assets)
        .run(|cx| {
            gpui_component::init(cx);
            gpui_component::Theme::change(gpui_component::ThemeMode::Dark, None, cx);

            let (sender, events) = async_channel::unbounded();
            let keyboard = KeyboardListener::start(sender);
            let (store, config, config_error) = overlay::load_config();
            let height = overlay::list_height(config.tasks.len());
            let bounds = Bounds::centered(None, size(px(492.), height), cx);
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("DN Killer · GPUI".into()),
                    appears_transparent: true,
                    ..Default::default()
                }),
                window_background: WindowBackgroundAppearance::Transparent,
                window_min_size: Some(size(px(370.), px(130.))),
                ..Default::default()
            };

            match cx.open_window(options, |window, cx| {
                let overlay = cx.new(|cx| {
                    Overlay::new(keyboard, events, (store, config, config_error), window, cx)
                });
                cx.new(|cx| gpui_component::Root::new(overlay, window, cx).bg(rgba(0x00000000)))
            }) {
                Ok(handle) => {
                    let main_id = gpui::AnyWindowHandle::from(handle).window_id();
                    cx.on_window_closed(move |cx, id| {
                        if id == main_id {
                            cx.quit();
                        }
                    })
                    .detach();
                    cx.activate(true);
                }
                Err(error) => {
                    eprintln!("创建 GPUI 窗口失败: {error:#}");
                    cx.quit();
                }
            }
        });
}
