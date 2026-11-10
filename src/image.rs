use crate::file_system;
use crate::file_system::read_directory_and_build_file_tree;
use fatfs::FileSystem;
use fscommon::BufStream;
use gpui_kit::base::TreeState;
use gpui_kit::{AppContext, Context, Entity};
use std::fs::File;
use std::io;
use std::path::Path;

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

    pub fn file_system(&mut self) -> &mut FileSystem<BufStream<File>> {
        &mut self.file_system
    }

    pub fn is_empty(&self, context: &gpui_kit::App) -> bool {
        self.file_tree_state.read(context).entry(0).is_none()
    }

    pub fn rename_entry(
        &mut self,
        context: &mut Context<Self>,
        path: &str,
        new_name: &str,
    ) -> io::Result<()> {
        file_system::rename_entry(&self.file_system, path, new_name)?;

        let refresh_result = refresh_tree(self, context);

        refresh_result.map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("The entry was renamed, but the file tree could not be refreshed: {error}"),
            )
        })
    }

    pub fn delete_entry(&mut self, context: &mut Context<Self>, path: &str) -> io::Result<()> {
        file_system::delete_entry(&self.file_system, path)?;

        let refresh_result = refresh_tree(self, context);

        refresh_result.map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("The entry was deleted, but the file tree could not be refreshed: {error}"),
            )
        })
    }

    pub fn transfer_file(
        &mut self,
        context: &mut Context<Self>,
        source_file_path: &Path,
    ) -> io::Result<()> {
        let file_name = source_file_path
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("");
        let destination_file_path = format!("/{file_name}");

        file_system::copy_file_to_image(
            &source_file_path,
            &mut self.file_system,
            &destination_file_path,
        )?;

        let refresh_result = refresh_tree(self, context);

        refresh_result.map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("The file was transferred to image, but the file tree could not be refreshed: {error}"),
            )
        })
    }

    pub fn create_file(&mut self, context: &mut Context<Self>, file_path: &str) -> io::Result<()> {
        file_system::create_file(&mut self.file_system, file_path)?;

        let refresh_result = refresh_tree(self, context);

        refresh_result.map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("The file was created in image, but the file tree could not be refreshed: {error}"),
            )
        })
    }
}

pub fn create_tree_state<T>(
    context: &mut Context<T>,
    file_system: &FileSystem<BufStream<File>>,
) -> io::Result<Entity<TreeState>> {
    let items = read_directory_and_build_file_tree(&file_system.root_dir(), "")?;

    let file_tree_state = context.new(|context| {
        let mut state = TreeState::new(context);
        state.set_items(items, context);
        state
    });

    Ok(file_tree_state)
}

fn refresh_tree(image: &mut Image, context: &mut Context<Image>) -> io::Result<()> {
    let items = read_directory_and_build_file_tree(&image.file_system.root_dir(), "")?;

    image.file_tree_state.update(context, |state, context| {
        state.set_items(items, context);
    });

    context.notify();
    Ok(())
}
