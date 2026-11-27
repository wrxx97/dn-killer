use anyhow::Result;
use gpui::{
    App, Context, IntoElement, Render, SharedString, WeakEntity, Window, WindowHandle, div,
    prelude::*, px, rgb, size,
};
use gpui_component::Root;
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::tooltip::Tooltip;

use crate::config::TaskDefinition;
use crate::forms::{TaskForm, field};
use crate::overlay::Overlay;
use crate::popup;

pub struct TaskEditor {
    overlay: WeakEntity<Overlay>,
    form: TaskForm,
    error: Option<SharedString>,
}

impl TaskEditor {
    pub fn open(
        owner: &Window,
        overlay: WeakEntity<Overlay>,
        task: Option<&TaskDefinition>,
        cx: &mut App,
    ) -> Result<WindowHandle<Root>> {
        let title = if task.is_some() {
            "编辑倒计时"
        } else {
            "新建倒计时"
        };
        popup::open(
            owner,
            title,
            size(px(400.), px(352.)),
            popup::Layout::FitContent,
            cx,
            |error, window, cx| Self {
                overlay,
                form: TaskForm::new(task, window, cx),
                error,
            },
        )
    }

    fn finish(&mut self, result: Result<Result<()>>, window: &mut Window, cx: &mut Context<Self>) {
        match result.and_then(std::convert::identity) {
            Ok(()) => window.remove_window(),
            Err(error) => {
                self.error = Some(format!("{error:#}").into());
                cx.notify();
            }
        }
    }
}

impl Render for TaskEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("task-editor")
            .w_full()
            .flex()
            .flex_col()
            .gap_3()
            .px_4()
            .pt_3()
            .pb_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(field("任务名称", &self.form.title))
                    .child(field("时长（秒）", &self.form.duration))
                    .child(field("任务快捷键（松开触发）", &self.form.hotkey))
                    .children(self.error.clone().map(|error| {
                        div()
                            .id("task-editor-error")
                            .text_size(px(13.))
                            .line_height(px(16.))
                            .line_clamp(2)
                            .text_color(rgb(0xffbdb5))
                            .child(error.clone())
                            .tooltip(move |window, cx| {
                                Tooltip::new(error.clone()).build(window, cx)
                            })
                    })),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .justify_end()
                    .flex_shrink_0()
                    .children(self.form.id.map(|id| {
                        Button::new("delete-task")
                            .danger()
                            .label("删除任务")
                            .on_click(cx.listener(move |view, _, window, cx| {
                                let result = view
                                    .overlay
                                    .update(cx, |overlay, cx| overlay.delete_task(id, cx));
                                view.finish(result, window, cx);
                            }))
                    }))
                    .child(
                        Button::new("cancel")
                            .label("取消")
                            .on_click(|_, window, _| window.remove_window()),
                    )
                    .child(
                        Button::new("save-task")
                            .primary()
                            .label("保存")
                            .on_click(cx.listener(|view, _, window, cx| {
                                let result = view
                                    .overlay
                                    .update(cx, |overlay, cx| overlay.save_task(&view.form, cx));
                                view.finish(result, window, cx);
                            })),
                    ),
            )
    }
}
