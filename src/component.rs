use gpui_kit::component::Theme;
use gpui_kit::component::button::Button;
use gpui_kit::{div, px, transparent_black, Div, ElementId, Fill, IntoElement, ParentElement, Styled, InteractiveElement};

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

pub fn title_item(child: impl IntoElement) -> Div {
    div().flex().items_center().child(child)
}
