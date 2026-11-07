use crate::dialog::dialog_form;
use crate::{Application, component, event, file_system};
use fatfs::FileSystem;
use fscommon::BufStream;
use gpui_kit::component::WindowExt;
use gpui_kit::component::dialog::DialogEntrance;
use gpui_kit::component::notification::NotificationType;
use gpui_kit::{
    AppContext, ClickEvent, Context, Entity, EventEmitter, IntoElement, ParentElement, Render,
    Window,
};
use std::cell::RefCell;
use std::fs::File;
use std::path::PathBuf;
use std::range::RangeInclusive;
use std::sync::Arc;

pub trait ImageDialog: Render + EventEmitter<event::ImageOpenEvent> + Sized + 'static {
    const TITLE: &'static str;

    fn new(window: &mut Window, context: &mut Context<Self>) -> Self;

    fn open(window: &mut Window, context: &mut Context<Application>) {
        let view = context.new(|context| Self::new(window, context));

        context
            .subscribe(
                &view,
                |application, _view, event: &event::ImageOpenEvent, context| {
                    if let Some(file_system) = event.0.borrow_mut().take() {
                        application.set_file_system(context, Arc::new(file_system));
                    }
                },
            )
            .detach();

        window.open_dialog(context, move |dialog, _, _| {
            dialog
                .title(Self::TITLE)
                .child(view.clone())
                .entrance(DialogEntrance::Fade)
        });
    }

    fn handle_result(
        result: Result<FileSystem<BufStream<File>>, impl std::fmt::Display>,
        error_prefix: &str,
        window: &mut Window,
        context: &mut Context<Self>,
    ) {
        match result {
            Ok(file_system) => {
                context.emit(event::ImageOpenEvent(RefCell::new(Some(file_system))));
            }
            Err(error) => {
                window.push_notification(
                    (NotificationType::Error, format!("{error_prefix}\n{error}")),
                    context,
                );
            }
        }
        window.close_dialog(context);
    }
}

pub struct OpenImageDialog {
    file_picker: Entity<component::PathPicker>,
}

impl ImageDialog for OpenImageDialog {
    const TITLE: &'static str = "Open image";

    fn new(window: &mut Window, context: &mut Context<Self>) -> Self {
        Self {
            file_picker: component::file_picker(window, context, "Image path"),
        }
    }
}

impl OpenImageDialog {
    fn submit(&mut self, _event: &ClickEvent, window: &mut Window, context: &mut Context<Self>) {
        let image_file_path_string = self.file_picker.read(context).value(context);
        let image_file_path = PathBuf::from(image_file_path_string);

        let file_system = file_system::open_image(&image_file_path);

        Self::handle_result(
            file_system,
            "Failed to open image file. Please try again.",
            window,
            context,
        );
    }
}

impl Render for OpenImageDialog {
    fn render(&mut self, _window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        dialog_form(context, [self.file_picker.clone()], Self::submit)
    }
}

impl EventEmitter<event::ImageOpenEvent> for OpenImageDialog {}

pub struct CreateImageDialog {
    file_name_input: Entity<component::FileNameInput>,
    size_select: Entity<component::SizeSelect>,
    folder_picker: Entity<component::PathPicker>,
}

impl ImageDialog for CreateImageDialog {
    const TITLE: &'static str = "Create image";

    fn new(window: &mut Window, context: &mut Context<Self>) -> Self {
        Self {
            file_name_input: component::file_name_input(window, context, "Name", ".img", "example"),
            size_select: component::size_select(context, RangeInclusive::from(2..=1024), 2),
            folder_picker: component::folder_picker(window, context, "Folder path"),
        }
    }
}

impl CreateImageDialog {
    fn submit(&mut self, _event: &ClickEvent, window: &mut Window, context: &mut Context<Self>) {
        let image_name = self.file_name_input.read(context).value(context) + ".img";
        let image_folder = self.folder_picker.read(context).value(context);

        let image_path = PathBuf::from(image_folder).join(image_name);

        let image_size = self.size_select.read(context).value() as u64 * 1_048_576;

        let file_system = file_system::create_image(&image_path, image_size, *b"EMULATED_FS");

        Self::handle_result(
            file_system,
            "Failed to create image file. Please try again.",
            window,
            context,
        );
    }
}

impl Render for CreateImageDialog {
    fn render(&mut self, _window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        dialog_form(
            context,
            [
                self.file_name_input.clone().into_any_element(),
                self.size_select.clone().into_any_element(),
                self.folder_picker.clone().into_any_element(),
            ],
            Self::submit,
        )
    }
}

impl EventEmitter<event::ImageOpenEvent> for CreateImageDialog {}
