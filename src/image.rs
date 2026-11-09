use crate::file_system::read_directory;
use fatfs::FileSystem;
use fscommon::BufStream;
use gpui_kit::base::TreeState;
use gpui_kit::{AppContext, Context, Entity, Window};
use std::fs::File;
use std::io;
use gpui_kit::component::notification::NotificationType;
use gpui_kit::component::WindowExt;
use crate::Application;
use crate::dialog::DialogResult;

pub struct Image {
    file_system: FileSystem<BufStream<File>>,
    file_tree_state: Entity<TreeState>,
}

impl Image {
    pub fn from(
        file_system: FileSystem<BufStream<File>>,
        file_tree_state: Entity<TreeState>,
    ) -> Self {
        Self {
            file_system,
            file_tree_state,
        }
    }

    pub fn tree_state(&self) -> &Entity<TreeState> {
        &self.file_tree_state
    }

    pub fn file_system(&self) -> &FileSystem<BufStream<File>> {
        &self.file_system
    }

    pub fn is_empty(&self) -> bool {
        self.file_system.root_dir().iter().next().is_none()
    }
}

pub fn create_tree_state<T>(
    context: &mut Context<T>,
    file_system: &FileSystem<BufStream<File>>,
) -> io::Result<Entity<TreeState>> {
    let items = read_directory(&file_system.root_dir(), "")?;

    let file_tree_state = context.new(|context| {
        let mut state = TreeState::new(context);
        state.set_items(items, context);
        state
    });

    Ok(file_tree_state)
}

pub fn handle_dialog_result(
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
            let image = context.new(|_| {
                Image::from(file_system, state)
            });

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
