use crate::file_system::read_directory;
use fatfs::FileSystem;
use fscommon::BufStream;
use gpui_kit::base::TreeState;
use gpui_kit::{AppContext, Context, Entity};
use std::fs::File;
use std::io;

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
