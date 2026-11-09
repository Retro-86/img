use crate::application::Application;
use crate::dialog;
use crate::dialog::{CreateImageDialog, DialogResult, OpenImageDialog};
use crate::image::{Image, create_tree_state};
use fatfs::FileSystem;
use fscommon::BufStream;
use gpui_kit::component::WindowExt;
use gpui_kit::component::notification::NotificationType;
use gpui_kit::private::schemars;
use gpui_kit::{Action, App, AppContext, Context, SharedString, Window, actions};
use serde::Deserialize;
use std::fs::File;

actions!(application, [Quit, CreateImage, OpenImage, CloseImage]);
actions!(image, [AddFile, CreateFile]);

#[derive(Action, Clone, PartialEq, Deserialize, schemars::JsonSchema)]
#[action(namespace = file_tree)]
pub struct Rename {
    pub identifier: SharedString,
}

#[derive(Action, Clone, PartialEq, Deserialize, schemars::JsonSchema)]
#[action(namespace = file_tree)]
pub struct Delete {
    pub identifier: SharedString,
}

pub fn quit(_: &Quit, context: &mut App) {
    context.quit();
}

pub fn open_image(
    _application: &mut Application,
    _: &OpenImage,
    window: &mut Window,
    context: &mut Context<Application>,
) {
    dialog::show_dialog(window, context, OpenImageDialog::new, install_image);
}

pub fn create_image(
    _application: &mut Application,
    _: &CreateImage,
    window: &mut Window,
    context: &mut Context<Application>,
) {
    dialog::show_dialog(window, context, CreateImageDialog::new, install_image);
}

pub fn install_image(
    application: &mut Application,
    result: DialogResult<FileSystem<BufStream<File>>>,
    window: &mut Window,
    context: &mut Context<Application>,
) {
    let DialogResult::Submitted(file_system) = result else {
        return;
    };

    match create_tree_state(context, &file_system) {
        Ok(state) => {
            let image = context.new(|_| Image::from(file_system, state));

            application.set_image(context, image);
        }
        Err(error) => {
            window.push_notification(
                (
                    NotificationType::Error,
                    format!("Unable to read the image contents\n{error}"),
                ),
                context,
            );
        }
    }
}

pub fn close_image(
    application: &mut Application,
    _: &CloseImage,
    window: &mut Window,
    context: &mut Context<Application>,
) {
    application.close_image(window, context);
}

pub fn rename_entry(
    application: &mut Application,
    action: &Rename,
    window: &mut Window,
    context: &mut Context<Application>,
) {
    let Some(image) = application.image().cloned() else {
        return;
    };

    let path = action.identifier.to_string();
    let current_name = path.rsplit('/').next().unwrap_or(&path).to_owned();

    dialog::show_dialog(
        window,
        context,
        move |window, context| {
            dialog::file_system::RenameEntryDialog::new(&current_name, window, context)
        },
        move |_, result, window, context| {
            let DialogResult::Submitted(name) = result else {
                return;
            };

            let result = image.update(context, |image, context| {
                image.rename(&path, &name, context)
            });

            context.notify();

            if let Err(error) = result {
                window.push_notification(
                    (
                        NotificationType::Error,
                        format!("Unable to rename\n{error}"),
                    ),
                    context,
                );
            }
        },
    );
}

pub fn delete_entry(
    application: &mut Application,
    action: &Delete,
    window: &mut Window,
    context: &mut Context<Application>,
) {
    let Some(image) = application.image().cloned() else {
        return;
    };

    let path = action.identifier.to_string();
    let name = path.rsplit('/').next().unwrap_or(&path).to_owned();

    dialog::show_dialog(
        window,
        context,
        move |_, _| dialog::file_system::DeleteEntryDialog::new(name),
        move |_, result, window, context| {
            let DialogResult::Submitted(()) = result else {
                return;
            };

            let result = image.update(context, |image, context| image.delete_entry(&path, context));

            context.notify();

            if let Err(error) = result {
                window.push_notification(
                    (
                        NotificationType::Error,
                        format!("Unable to delete\n{error}"),
                    ),
                    context,
                );
            }
        },
    );
}
