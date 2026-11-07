use fatfs::{Dir, FileSystem, ReadWriteSeek};
use fscommon::BufStream;
use gpui_kit::assets::IconName;
use gpui_kit::base::{TreeItem, TreeState, h_flex};
use gpui_kit::component::list::ListItem;
use gpui_kit::component::tree::tree;
use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, px,
};
use std::fs::File;
use std::sync::Arc;

#[derive(Clone)]
pub struct FileTreeItem {
    pub label: String,
    pub is_dir: bool,
    pub path: String,
}

pub struct FileTree {
    state: Entity<TreeState>,
    file_system: Arc<FileSystem<BufStream<File>>>,
}

impl FileTree {
    pub fn new(context: &mut Context<Self>, file_system: Arc<FileSystem<BufStream<File>>>) -> Self {
        let state = context.new(|context| TreeState::new(context));

        let mut file_tree = Self { state, file_system };

        file_tree.populate_tree(context);

        file_tree
    }

    fn populate_tree(&mut self, context: &mut Context<Self>) {
        let file_system = self.file_system.clone();
        let root_directory = file_system.root_dir();

        let items = read_directory(&root_directory, "");

        self.state.update(context, |state, context| {
            state.set_items(items, context);
        });
    }
}

impl Render for FileTree {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        tree(&self.state, |index, entry, selected, _, _| {
            let item = entry.item();

            let icon = if !entry.is_folder() {
                IconName::File
            } else if entry.is_expanded() {
                IconName::FolderOpen
            } else {
                IconName::Folder
            };

            ListItem::new(index)
                .selected(selected)
                .pl(px(16.) * entry.depth() + px(12.))
                .child(h_flex().gap_2().child(icon).child(item.label.clone()))
        })
    }
}

fn read_directory<T: ReadWriteSeek>(directory: &Dir<T>, current_path: &str) -> Vec<TreeItem> {
    let mut items = Vec::new();

    if let Ok(entries) = directory.iter().collect::<Result<Vec<_>, _>>() {
        for entry in entries {
            let name = entry.file_name();

            let full_path = if current_path.is_empty() {
                name.clone()
            } else {
                format!("{}/{}", current_path, name)
            };

            let mut tree_item = TreeItem::new(full_path.clone(), name);

            if entry.is_dir() {
                let sub_directory = entry.to_dir();
                let children = read_directory(&sub_directory, &full_path);
                tree_item = tree_item.children(children);
            }

            items.push(tree_item);
        }
    }

    items
}
