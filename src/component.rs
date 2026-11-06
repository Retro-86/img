pub mod file_name;
pub mod path_picker;
pub mod size_select;

pub use crate::component::file_name::FileNameInput;
pub use crate::component::path_picker::PathPicker;
pub use crate::component::size_select::SizeSelect;
use gpui_kit::component::button::Button;
use gpui_kit::component::menu::AppMenuBar;
use gpui_kit::component::{ActiveTheme, Theme, TitleBar};
use gpui_kit::{
    App, AppContext, Div, ElementId, Entity, IntoElement, ParentElement, Styled, Window, div, px,
    transparent_black,
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
    context: &mut App,
    window: &mut Window,
    label: &str,
    extension: &str,
    placeholder: &str,
) -> Entity<FileNameInput> {
    context.new(|context| FileNameInput::new(context, window, label, extension, placeholder))
}

pub fn size_select(context: &mut App, range: RangeInclusive<u32>, step: u32) -> Entity<SizeSelect> {
    context.new(|context| SizeSelect::new(context, range, step))
}

pub fn title_bar(context: &mut App, menu_bar: &Entity<AppMenuBar>) -> TitleBar {
    let theme = context.theme();

    TitleBar::new().child(menu_bar.clone()).bg(theme.title_bar)
}

pub fn path_picker(context: &mut App, window: &mut Window, label: &str) -> Entity<PathPicker> {
    context.new(|context| PathPicker::new(window, context, label))
}
