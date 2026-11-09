use crate::component;
use crate::dialog::{Dialog, DialogEvent, dialog_form};

use gpui_kit::{Context, Entity, EventEmitter, IntoElement, Render, SharedString, Window};
use std::io;

pub struct RenameFileDialog {
    name_input: Entity<component::FileNameInput>,
}

impl RenameFileDialog {
    pub fn new(current_name: &str, window: &mut Window, context: &mut Context<Self>) -> Self {
        Self {
            name_input: component::file_name_input(window, context, "Name", "", current_name),
        }
    }
}

impl Dialog for RenameFileDialog {
    type Output = String;
    type Error = io::Error;

    fn title(&self) -> SharedString {
        "Rename file".into()
    }

    fn error_message(&self) -> SharedString {
        "Invalid file name.".into()
    }

    fn submit(&mut self, _window: &mut Window, context: &mut Context<Self>) -> io::Result<String> {
        let name = self.name_input.read(context).value(context);

        if name.trim().is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Enter a file name.",
            ));
        }

        Ok(name)
    }
}

impl Render for RenameFileDialog {
    fn render(&mut self, _window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        dialog_form(context, [self.name_input.clone()], Self::confirm)
    }
}

impl EventEmitter<DialogEvent<String>> for RenameFileDialog {}
