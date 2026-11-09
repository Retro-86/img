use crate::image::Image;
use crate::{actions, component};
use gpui_kit::base::v_flex;
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::label::Label;
use gpui_kit::component::menu::AppMenuBar;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    Context, Entity, FocusHandle, Focusable, Global, InteractiveElement, IntoElement,
    ParentElement, Render, Styled, Window,
};

pub struct Application {
    menu_bar: Entity<AppMenuBar>,
    focus_handle: FocusHandle,
    image: Option<Entity<Image>>,
}

impl Application {
    pub fn new(window: &mut Window, context: &mut Context<Self>) -> Self {
        let focus_handle = context.focus_handle();
        window.focus(&focus_handle, context);

        Self {
            menu_bar: AppMenuBar::new(context),
            focus_handle,
            image: None,
        }
    }

    pub fn set_image(&mut self, context: &mut Context<Self>, image: Entity<Image>) {
        self.image = Some(image);

        context.notify();
    }

    pub fn image(&self) -> Option<&Entity<Image>> {
        self.image.as_ref()
    }

    pub fn close_image(&mut self, window: &mut Window, context: &mut Context<Self>) {
        self.image = None;
        context.notify();

        let focus_handle = self.focus_handle.clone();

        window.defer(context, move |window, context| {
            window.focus(&focus_handle, context);
        });
    }
}

impl Focusable for Application {
    fn focus_handle(&self, _context: &gpui_kit::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Global for Application {}

impl Render for Application {
    fn render(&mut self, _window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        let theme = context.theme();

        component::application_content(theme)
            .track_focus(&self.focus_handle)
            .child(component::title_bar(context, &self.menu_bar))
            .when_some(self.image.as_ref(), |this, image| {
                let image = image.read(context);

                if image.is_empty(context) {
                    this.child(
                        v_flex()
                            .h_full()
                            .w_full()
                            .items_center()
                            .justify_center()
                            .child(Label::new("this image is empty").text_center()),
                    )
                } else {
                    this.child(component::file_tree(image.tree_state().clone()))
                        .on_action(context.listener(actions::rename_file))
                        .on_action(
                            context.listener(|this, action: &actions::Delete, window, context| {}),
                        )
                }
            })
            .when_none(&self.image, |this| {
                this.child(
                    v_flex()
                        .h_full()
                        .w_full()
                        .items_center()
                        .justify_center()
                        .child(Label::new("open or create image file").text_center()),
                )
            })
            .on_action(context.listener(actions::open_image))
            .on_action(context.listener(actions::create_image))
            .on_action(context.listener(actions::close_image))
    }
}
