use crate::{Application, component, event, file_system};
use gpui_kit::base::v_flex;
use gpui_kit::component::notification::NotificationType;
use gpui_kit::component::{
    WindowExt,
    button::{Button, ButtonVariants},
    dialog::{DialogEntrance, DialogFooter},
};
use gpui_kit::{
    AppContext, ClickEvent, Context, Entity, EventEmitter, IntoElement, ParentElement, Render,
    Styled, Window,
};
use std::cell::RefCell;
use std::path::PathBuf;
use std::range::RangeInclusive;
use std::sync::Arc;

pub struct CreateImageDialog {
    file_name_input: Entity<component::FileNameInput>,
    size_select: Entity<component::SizeSelect>,
    path_picker: Entity<component::PathPicker>,
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
            path_picker,
        }
    }

    pub fn open(window: &mut Window, context: &mut Context<Application>) {
        let view = context.new(|context| Self::new(window, context));

        context
            .subscribe(
                &view,
                |application, _view, event: &event::ImageCreatedEvent, _| {
                    if let Some(file_system) = event.0.borrow_mut().take() {
                        application.file_system = Some(Arc::new(file_system));
                    }
                },
            )
            .detach();

        window.open_dialog(context, move |dialog, _, _| {
            dialog
                .title("Create image")
                .child(view.clone())
                .entrance(DialogEntrance::Fade)
        });
    }

    fn submit(&mut self, _event: &ClickEvent, window: &mut Window, context: &mut Context<Self>) {
        let image_name = self.file_name_input.read(context).value(context) + ".img";
        let image_file_path_string = self.path_picker.read(context).value(context);

        let image_path = PathBuf::from(image_file_path_string).join(image_name);

        let image_size = self.size_select.read(context).value() * 1_048_576;

        let image_file_system =
            file_system::create_image(&image_path, image_size as u64, *b"EMULATED_FS");

        match image_file_system {
            Ok(file_system) => {
                let event = event::ImageCreatedEvent(RefCell::new(Some(file_system)));
                context.emit(event);
            }
            Err(error) => {
                window.push_notification(
                    (
                        NotificationType::Error,
                        format!("Failed to create image file. Please try again.\n{error}"),
                    ),
                    context,
                );
            }
        }

        window.close_dialog(context);
    }
}

impl Render for CreateImageDialog {
    fn render(&mut self, _window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
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
                            .on_click(context.listener(Self::submit)),
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

impl EventEmitter<event::ImageCreatedEvent> for CreateImageDialog {}
