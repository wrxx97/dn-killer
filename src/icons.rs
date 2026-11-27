use gpui::{ElementId, px};
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::{Icon, Sizable as _};

macro_rules! outline {
    ($shape:literal) => {
        concat!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">"#,
            $shape,
            "</svg>",
        )
        .as_bytes()
    };
}

pub enum ButtonIcon {
    Add,
    Start,
    Restart,
    Edit,
    Delete,
    Settings,
    Pin(bool),
    Minimize,
    Close,
}

impl ButtonIcon {
    fn svg(self) -> &'static [u8] {
        match self {
            Self::Add => outline!(r#"<path d="M12 5v14M5 12h14"/>"#),
            Self::Start => outline!(r#"<path d="m8 5 11 7-11 7z"/>"#),
            Self::Restart => outline!(r#"<path d="M20 4v6h-6m6-1a8 8 0 1 0-.5 7.9"/>"#),
            Self::Edit => outline!(r#"<path d="m4 16-.5 4.5L8 20 20 8l-4-4L4 16Zm10-10 4 4"/>"#),
            Self::Delete => {
                outline!(r#"<path d="M4 6h16M9 6V3.5h6V6M6 6l1 14h10l1-14M10 10v6m4-6v6"/>"#)
            }
            Self::Settings => outline!(
                r#"<path d="M9.5 3.5h5l.6 2.4 2 .95 2.3-.75 2.15 3.7-1.75 1.65v1.1l1.75 1.65-2.15 3.7-2.3-.75-2 .95-.6 2.4h-5l-.6-2.4-2-.95-2.3.75-2.15-3.7 1.75-1.65v-1.1L2.45 9.8 4.6 6.1l2.3.75 2-.95z"/><circle cx="12" cy="12" r="3"/>"#
            ),
            Self::Pin(false) => include_bytes!("../assets/pin-outline.svg"),
            Self::Pin(true) => include_bytes!("../assets/pin-filled.svg"),
            Self::Minimize => outline!(r#"<path d="M6 12h12"/>"#),
            Self::Close => outline!(r#"<path d="m6 6 12 12M18 6 6 18"/>"#),
        }
    }
}

pub fn button(id: impl Into<ElementId>, icon: ButtonIcon, label: &'static str) -> Button {
    Button::new(id)
        .ghost()
        .with_size(px(24.))
        .icon(Icon::default().data(icon.svg()))
        .tooltip(label)
        .accessibility_label(label)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn all_button_icons_render() {
        let renderer = gpui::SvgRenderer::new(Arc::new(gpui_kit_assets::Assets));
        for icon in [
            ButtonIcon::Add,
            ButtonIcon::Start,
            ButtonIcon::Restart,
            ButtonIcon::Edit,
            ButtonIcon::Delete,
            ButtonIcon::Settings,
            ButtonIcon::Pin(false),
            ButtonIcon::Pin(true),
            ButtonIcon::Minimize,
            ButtonIcon::Close,
        ] {
            let image = renderer.render_single_frame(icon.svg(), 1.).unwrap();
            assert!(
                image
                    .as_bytes(0)
                    .unwrap()
                    .chunks_exact(4)
                    .any(|pixel| pixel[3] != 0)
            );
        }
    }
}
