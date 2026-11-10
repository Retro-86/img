pub mod file_system;
pub mod image;

use crate::component;
use gpui_kit::component::dialog::DialogEntrance;
use gpui_kit::component::notification::NotificationType;
use gpui_kit::component::{WindowExt, v_flex};
use gpui_kit::{
    AppContext, Context, Div, Entity, EventEmitter, IntoElement, ParentElement, Render,
    SharedString, Styled, Window,
};
pub use image::{CreateImageDialog, OpenImageDialog};
use std::cell::RefCell;

pub enum DialogResult<T> {
    Submitted(T),
    Cancelled,
}

pub struct DialogEvent<T>(pub RefCell<Option<DialogResult<T>>>);

pub trait Dialog: Render + EventEmitter<DialogEvent<Self::Output>> + Sized + 'static {
    type Output: 'static;
    type Error: std::fmt::Display;

    fn title(&self) -> SharedString;
    fn error_message(&self) -> SharedString;

    fn submit(
        &mut self,
        window: &mut Window,
        context: &mut Context<Self>,
    ) -> Result<Self::Output, Self::Error>;

    fn confirm(&mut self, window: &mut Window, context: &mut Context<Self>) {
        let result = match self.submit(window, context) {
            Ok(output) => DialogResult::Submitted(output),
            Err(error) => {
                window.push_notification(
                    (
                        NotificationType::Error,
                        format!("{}\n{error}", self.error_message()),
                    ),
                    context,
                );

                DialogResult::Cancelled
            }
        };

        context.emit(DialogEvent(RefCell::new(Some(result))));
    }

    fn cancel(&mut self, _window: &mut Window, context: &mut Context<Self>) {
        context.emit(DialogEvent::<Self::Output>(RefCell::new(Some(
            DialogResult::Cancelled,
        ))));
    }
}

pub fn show_dialog<D: Dialog, T: 'static>(
    window: &mut Window,
    context: &mut Context<T>,
    build: impl FnOnce(&mut Window, &mut Context<D>) -> D,
    on_result: impl FnOnce(&mut T, DialogResult<D::Output>, &mut Window, &mut Context<T>) + 'static,
) -> Entity<D> {
    let view = context.new(|context| build(window, context));
    let mut on_result = Some(on_result);

    context
        .subscribe_in(
            &view,
            window,
            move |owner, _, event: &DialogEvent<D::Output>, window, context| {
                let Some(result) = event.0.borrow_mut().take() else {
                    return;
                };

                let Some(on_result) = on_result.take() else {
                    return;
                };

                window.close_dialog(context);
                on_result(owner, result, window, context);
            },
        )
        .detach();

    let dialog_view = view.clone();

    window.open_dialog(context, move |dialog, _, context| {
        let confirm_view = dialog_view.clone();
        let cancel_view = dialog_view.clone();

        dialog
            .title(dialog_view.read(context).title())
            .child(dialog_view.clone())
            .entrance(DialogEntrance::Fade)
            .on_ok(move |_, window, context| {
                confirm_view.update(context, |dialog, context| {
                    dialog.confirm(window, context);
                });

                false
            })
            .on_cancel(move |_, window, context| {
                cancel_view.update(context, |dialog, context| {
                    dialog.cancel(window, context);
                });

                false
            })
    });

    view
}

pub fn dialog_form<T: Dialog>(
    context: &mut Context<T>,
    children: impl IntoIterator<Item = impl IntoElement>,
    submit: impl Fn(&mut T, &mut Window, &mut Context<T>) + 'static,
) -> Div {
    v_flex()
        .gap_6()
        .children(children)
        .child(component::dialog_footer(
            context,
            move |instance, _, window, context| {
                submit(instance, window, context);
            },
        ))
}
