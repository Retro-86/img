use crate::dialog::{Dialog, DialogEvent, dialog_form};
use crate::{component, file_system};
use std::fs::File;

use fatfs::FileSystem;
use fscommon::BufStream;
use gpui_kit::{Context, Entity, EventEmitter, IntoElement, Render, SharedString, Window};
use std::io;
use std::path::PathBuf;
use std::range::RangeInclusive;

pub struct OpenImageDialog {
    file_picker: Entity<component::PathPicker>,
}

impl OpenImageDialog {
    pub fn new(window: &mut Window, context: &mut Context<Self>) -> Self {
        Self {
            file_picker: component::file_picker(window, context, "Image path"),
        }
    }
}

impl Dialog for OpenImageDialog {
    type Output = FileSystem<BufStream<File>>;
    type Error = io::Error;

    fn title(&self) -> SharedString {
        "Open image".into()
    }

    fn error_message(&self) -> SharedString {
        "Failed to open image file. Please try again.".into()
    }

    fn submit(
        &mut self,
        _window: &mut Window,
        context: &mut Context<Self>,
    ) -> Result<Self::Output, Self::Error> {
        let path = PathBuf::from(self.file_picker.read(context).value(context));

        file_system::open_image(&path)
    }
}

impl Render for OpenImageDialog {
    fn render(&mut self, _window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        dialog_form(context, [self.file_picker.clone()], Self::confirm)
    }
}

impl EventEmitter<DialogEvent<FileSystem<BufStream<File>>>> for OpenImageDialog {}

pub struct CreateImageDialog {
    file_name_input: Entity<component::FileNameInput>,
    size_select: Entity<component::SizeSelect>,
    folder_picker: Entity<component::PathPicker>,
}

impl CreateImageDialog {
    pub fn new(window: &mut Window, context: &mut Context<Self>) -> Self {
        Self {
            file_name_input: component::file_name_input(window, context, "Name", ".img", "example"),
            size_select: component::size_select(context, RangeInclusive::from(2..=1024), 2),
            folder_picker: component::folder_picker(window, context, "Folder path"),
        }
    }
}

impl Dialog for CreateImageDialog {
    type Output = FileSystem<BufStream<File>>;
    type Error = io::Error;

    fn title(&self) -> SharedString {
        "Create image".into()
    }

    fn error_message(&self) -> SharedString {
        "Failed to create image file. Please try again.".into()
    }

    fn submit(
        &mut self,
        _window: &mut Window,
        context: &mut Context<Self>,
    ) -> Result<Self::Output, Self::Error> {
        let name = self.file_name_input.read(context).value(context) + ".img";

        let folder = self.folder_picker.read(context).value(context);
        let path = PathBuf::from(folder).join(name);

        let size = self.size_select.read(context).value() as u64 * 1_048_576;

        file_system::create_image(&path, size, *b"EMULATED_FS")
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
            Self::confirm,
        )
    }
}

impl EventEmitter<DialogEvent<FileSystem<BufStream<File>>>> for CreateImageDialog {}
