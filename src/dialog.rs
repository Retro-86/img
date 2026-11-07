pub mod image;

use crate::component;
use gpui_kit::component::v_flex;
use gpui_kit::{ClickEvent, Context, Div, IntoElement, ParentElement, Styled, Window};
pub use image::{CreateImageDialog, OpenImageDialog};

pub fn dialog_form<T: 'static>(
    context: &mut Context<T>,
    children: impl IntoIterator<Item = impl IntoElement>,
    submit: impl Fn(&mut T, &ClickEvent, &mut Window, &mut Context<T>) + 'static,
) -> Div {
    v_flex()
        .gap_6()
        .children(children)
        .child(component::dialog_footer(context, submit))
}
