use crate::component;
use gpui_kit::base::v_flex;
use gpui_kit::component::{
    WindowExt,
    button::{Button, ButtonVariants},
    dialog::{DialogEntrance, DialogFooter},
};
use gpui_kit::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window,
};
use std::range::RangeInclusive;

pub struct CreateImageDialog {
    file_name_input: Entity<component::FileNameInput>,
    size_select: Entity<component::SizeSelect>,
    path_picker: Entity<component::PathPicker>
}

impl CreateImageDialog {
    fn new(window: &mut Window, context: &mut Context<Self>) -> Self {
        let file_name_input =
            component::file_name_input(context, window, "Name", ".img", "example");

        let size_select = component::size_select(context, RangeInclusive::from(2..=1024), 2);

        let path_picker = component::path_picker(context, window, "Path");

        Self {
            file_name_input,
            size_select,
            path_picker
        }
    }

    pub fn open(window: &mut Window, context: &mut App) {
        let view = context.new(|context| Self::new(window, context));

        window.open_dialog(context, move |dialog, _, _| {
            dialog
                .title("Create image")
                .child(view.clone())
                .entrance(DialogEntrance::Fade)
        });
    }
}

impl Render for CreateImageDialog {
    fn render(&mut self, _window: &mut Window, _context: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_6()
            .child(self.file_name_input.clone())
            .child(self.size_select.clone())
            .child(self.path_picker.clone())
            .child(
                DialogFooter::new()
                    .child(
                        Button::new("ok")
                            .primary()
                            .label("Submit")
                            .on_click(|_, _, _| todo!()),
                    )
                    .child(
                        Button::new("cancel")
                            .label("Cancel")
                            .on_click(|_, window, context| {
                                window.close_dialog(context);
                            }),
                    ),
            )
    }
}
