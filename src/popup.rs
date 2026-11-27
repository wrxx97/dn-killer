use std::cell::Cell;
use std::rc::Rc;

use anyhow::Result;
use gpui::{
    App, AppContext as _, Bounds, Context, Entity, IntoElement, Pixels, Render, SharedString, Size,
    TitlebarOptions, Window, WindowBackgroundAppearance, WindowBounds, WindowControlArea,
    WindowHandle, WindowOptions, div, prelude::*, px, rgb, rgba, size,
};
use gpui_component::Root;

use crate::icons::{ButtonIcon, button as icon_button};
use crate::native_window;

// 80% transparency leaves the background at 20% opacity.
const POPUP_BACKGROUND: u32 = 0x11182433;

pub enum Layout {
    FitContent,
    Resizable { minimum: Size<Pixels> },
}

pub fn open<V: Render + 'static>(
    owner: &Window,
    title: &'static str,
    dimensions: Size<Pixels>,
    layout: Layout,
    cx: &mut App,
    view: impl FnOnce(Option<SharedString>, &mut Window, &mut Context<V>) -> V,
) -> Result<WindowHandle<Root>> {
    let display = owner.display(cx).map(|display| display.id());
    let bounds = Bounds::centered(display, dimensions, cx);
    let minimum = match layout {
        Layout::FitContent => None,
        Layout::Resizable { minimum } => Some(minimum),
    };
    let options = WindowOptions {
        titlebar: Some(TitlebarOptions {
            title: Some(title.into()),
            appears_transparent: true,
            ..Default::default()
        }),
        window_background: WindowBackgroundAppearance::Transparent,
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        display_id: display,
        window_min_size: minimum,
        is_resizable: minimum.is_some(),
        is_minimizable: false,
        ..Default::default()
    };
    cx.open_window(options, |window, cx| {
        let owner = native_window::set_owner(window, owner);
        let cursor = native_window::install_drag_cursor(window);
        let error = owner
            .and(cursor)
            .err()
            .map(|error| error.to_string().into());
        let content = cx.new(|cx| view(error, window, cx));
        let popup = cx.new(|_| Popup {
            title,
            content,
            fit_content: minimum.is_none().then(Rc::default),
        });
        cx.new(|cx| Root::new(popup, window, cx).bg(rgba(0x00000000)))
    })
}

struct Popup<V: 'static> {
    title: &'static str,
    content: Entity<V>,
    fit_content: Option<Rc<Cell<Option<(Pixels, Pixels)>>>>,
}

impl<V: Render + 'static> Render for Popup<V> {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .overflow_hidden()
            .flex()
            .flex_col()
            .when_some(self.fit_content.clone(), |frame, last_request| {
                frame.on_children_prepainted(move |children, window, cx| {
                    let [header, body] = children.as_slice() else {
                        return;
                    };
                    let fitted_height = header.size.height + body.size.height;
                    let viewport = window.viewport_size();
                    // Preserve any inset above the header or below the popup's layout area.
                    let height = fitted_height + header.top() * 2.;
                    if (viewport.height - height).abs() <= px(1.) {
                        last_request.set(None);
                    } else if last_request.replace(Some((viewport.height, height)))
                        != Some((viewport.height, height))
                    {
                        window.defer(cx, move |window, _| {
                            window.resize(size(window.viewport_size().width, height));
                        });
                    }
                })
            })
            .bg(rgba(POPUP_BACKGROUND))
            .text_color(rgb(0xf4f7fc))
            .child(
                div()
                    .h(px(36.))
                    .flex()
                    .flex_shrink_0()
                    .items_center()
                    .px_3()
                    .child(
                        div()
                            .id("popup-drag-area")
                            .h_full()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .items_center()
                            .window_control_area(WindowControlArea::Drag)
                            .child(div().truncate().child(self.title)),
                    )
                    .child(
                        icon_button("popup-close", ButtonIcon::Close, "关闭窗口")
                            .on_click(|_, window, _| window.remove_window()),
                    ),
            )
            .child(
                div()
                    .when(self.fit_content.is_some(), |body| body.flex_shrink_0())
                    .when(self.fit_content.is_none(), |body| body.flex_1().min_h_0())
                    .child(self.content.clone()),
            )
    }
}
