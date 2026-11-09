pub mod file_name_input;
pub mod file_tree;
pub mod path_picker;
pub mod size_select;

pub use crate::component::file_name_input::FileNameInput;
pub use crate::component::file_tree::FileTree;
pub use crate::component::path_picker::PathPicker;
pub use crate::component::size_select::SizeSelect;
use gpui_kit::base::TreeState;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::dialog::DialogFooter;
use gpui_kit::component::menu::AppMenuBar;
use gpui_kit::component::{ActiveTheme, Theme, TitleBar, WindowExt};
use gpui_kit::{
    App, AppContext, ClickEvent, Context, Div, ElementId, Entity, IntoElement, ParentElement,
    Styled, Window, div, px, transparent_black,
};
use std::range::RangeInclusive;

pub fn button(id: impl Into<ElementId>) -> Button {
    Button::new(id)
        .border_0()
        .rounded_sm()
        .p(px(6.0))
        .bg(transparent_black())
}

pub fn application_content(theme: &Theme) -> Div {
    div()
        .size_full()
        .flex()
        .flex_col()
        .bg(theme.background)
        .text_color(theme.foreground)
        .font_family("Nordwand Mono")
}

pub fn title_items(children: Vec<impl IntoElement>) -> Div {
    div().flex().items_center().children(children)
}

pub fn file_name_input(
    window: &mut Window,
    context: &mut App,
    label: &str,
    extension: &str,
    placeholder: &str,
) -> Entity<FileNameInput> {
    context.new(|context| FileNameInput::new(window, context, label, extension, placeholder))
}

pub fn size_select(context: &mut App, range: RangeInclusive<u32>, step: u32) -> Entity<SizeSelect> {
    context.new(|context| SizeSelect::new(context, range, step))
}

pub fn title_bar(context: &mut App, menu_bar: &Entity<AppMenuBar>) -> TitleBar {
    let theme = context.theme();

    TitleBar::new().child(menu_bar.clone()).bg(theme.title_bar)
}

pub fn file_tree(state: Entity<TreeState>) -> FileTree {
    FileTree::new(state)
}

pub fn dialog_footer<T: crate::dialog::Dialog>(
    context: &mut Context<T>,
    submit: impl Fn(&mut T, &ClickEvent, &mut Window, &mut Context<T>) + 'static,
) -> DialogFooter {
    DialogFooter::new()
        .child(
            Button::new("ok")
                .primary()
                .label("Submit")
                .on_click(context.listener(submit)),
        )
        .child(
            Button::new("cancel")
                .label("Cancel")
                .on_click(context.listener(|instance, _, window, context| {
                    instance.cancel(window, context);
                })),
        )
}

pub fn folder_picker(window: &mut Window, context: &mut App, label: &str) -> Entity<PathPicker> {
    context.new(|context| PathPicker::folder(window, context, label))
}

pub fn file_picker(window: &mut Window, context: &mut App, label: &str) -> Entity<PathPicker> {
    context.new(|context| PathPicker::file(window, context, label))
}
