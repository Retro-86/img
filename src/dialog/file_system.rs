use crate::dialog::{Dialog, DialogEvent};
use crate::{component, dialog};
use std::convert::Infallible;

use gpui_kit::component::label::Label;
use gpui_kit::{
    Context, Entity, EventEmitter, IntoElement, ParentElement, Render, SharedString, Window, div,
};
use std::io;
use std::path::{Path, PathBuf};

pub struct RenameEntryDialog {
    name_input: Entity<component::FileNameInput>,
}

impl RenameEntryDialog {
    pub fn new(current_name: &str, window: &mut Window, context: &mut Context<Self>) -> Self {
        Self {
            name_input: component::file_name_input(window, context, "Name", "", current_name),
        }
    }
}

impl Dialog for RenameEntryDialog {
    type Output = String;
    type Error = io::Error;

    fn title(&self) -> SharedString {
        "Rename".into()
    }

    fn error_message(&self) -> SharedString {
        "Invalid name.".into()
    }

    fn submit(&mut self, _window: &mut Window, context: &mut Context<Self>) -> io::Result<String> {
        let name = self.name_input.read(context).value(context);

        if name.trim().is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Enter name."));
        }

        Ok(name)
    }
}

impl Render for RenameEntryDialog {
    fn render(&mut self, _window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        dialog::dialog_form(context, [self.name_input.clone()], Self::confirm)
    }
}

impl EventEmitter<DialogEvent<String>> for RenameEntryDialog {}

pub struct DeleteEntryDialog {
    name: String,
}

impl DeleteEntryDialog {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Dialog for DeleteEntryDialog {
    type Output = ();
    type Error = Infallible;

    fn title(&self) -> SharedString {
        "Confirm delete".into()
    }

    fn error_message(&self) -> SharedString {
        "Unable to confirm deletion.".into()
    }

    fn submit(
        &mut self,
        _window: &mut Window,
        _context: &mut Context<Self>,
    ) -> Result<(), Infallible> {
        Ok(())
    }
}

impl Render for DeleteEntryDialog {
    fn render(&mut self, _window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        dialog::dialog_form(
            context,
            [div().children([
                Label::new(format!("Delete \"{}\"?", self.name)),
                Label::new("If this is a folder, all its contents will also be deleted."),
            ])],
            Self::confirm,
        )
    }
}

impl EventEmitter<DialogEvent<()>> for DeleteEntryDialog {}

pub struct TransferFileDialog {
    file_path_picker: Entity<component::PathPicker>,
}

impl TransferFileDialog {
    pub fn new(window: &mut Window, context: &mut Context<Self>) -> Self {
        let file_path_picker = component::file_picker(window, context, "File path");

        Self { file_path_picker }
    }
}

impl Dialog for TransferFileDialog {
    type Output = PathBuf;
    type Error = io::Error;

    fn title(&self) -> SharedString {
        "Transfer file".into()
    }

    fn error_message(&self) -> SharedString {
        "Failed to transfer file. Please retry later".into()
    }

    fn submit(
        &mut self,
        _window: &mut Window,
        context: &mut Context<Self>,
    ) -> Result<Self::Output, Self::Error> {
        let file_path = self.file_path_picker.read(context).value(context);

        Ok(Path::new(&file_path).to_path_buf())
    }
}

impl Render for TransferFileDialog {
    fn render(&mut self, _window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        dialog::dialog_form(context, [self.file_path_picker.clone()], Self::confirm)
    }
}

impl EventEmitter<DialogEvent<PathBuf>> for TransferFileDialog {}

pub struct CreateFileDialog {
    file_name_input: Entity<component::FileNameInput>,
}

impl CreateFileDialog {
    pub fn new(window: &mut Window, context: &mut Context<Self>) -> Self {
        let file_name_input =
            component::file_name_input(window, context, "File name", "", "example.txt");

        Self { file_name_input }
    }
}

impl Dialog for CreateFileDialog {
    type Output = String;
    type Error = io::Error;

    fn title(&self) -> SharedString {
        "Create file".into()
    }

    fn error_message(&self) -> SharedString {
        "Failed to transfer file. Please retry later".into()
    }

    fn submit(
        &mut self,
        _window: &mut Window,
        context: &mut Context<Self>,
    ) -> Result<Self::Output, Self::Error> {
        let file_path = self.file_name_input.read(context).value(context);

        Ok(file_path)
    }
}

impl Render for CreateFileDialog {
    fn render(&mut self, _window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        dialog::dialog_form(context, [self.file_name_input.clone()], Self::confirm)
    }
}

impl EventEmitter<DialogEvent<String>> for CreateFileDialog {}
