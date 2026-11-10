use crate::actions;
use gpui_kit::assets::IconName;
use gpui_kit::base::{TreeState, h_flex};
use gpui_kit::component::list::ListItem;
use gpui_kit::component::tree::tree;
use gpui_kit::{App, Entity, IntoElement, ParentElement, RenderOnce, Styled, Window, px};

#[derive(IntoElement)]
pub struct FileTree {
    state: Entity<TreeState>,
}

impl FileTree {
    pub fn new(state: Entity<TreeState>) -> Self {
        Self { state }
    }
}

impl RenderOnce for FileTree {
    fn render(self, _window: &mut Window, _context: &mut App) -> impl IntoElement {
        tree(&self.state, |index, entry, selected, _, _| {
            let item = entry.item();
            let is_directory = item.is_folder();

            let icon = if !is_directory {
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
        .context_menu(|_index, entry, menu, _, _| {
            let item = entry.item();
            let file_identifier = &item.id;

            menu.menu(
                "Rename",
                Box::new(actions::Rename {
                    identifier: file_identifier.clone(),
                }),
            )
            .menu(
                "Delete",
                Box::new(actions::Delete {
                    identifier: file_identifier.clone(),
                }),
            )
        })
    }
}
