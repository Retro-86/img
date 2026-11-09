pub mod actions;
pub mod app_menu;
pub mod component;
pub mod dialog;
pub mod event;
pub mod file_system;
pub mod image;

use crate::image::Image;
use gpui_kit::component::menu::AppMenuBar;
use gpui_kit::component::{ActiveTheme, TitleBar};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    AppContext, Bounds, Context, Entity, FocusHandle, Focusable, Global, InteractiveElement,
    IntoElement, ParentElement, Render, Styled, Window, WindowBounds, WindowOptions, actions,
    application, div, point, px, size, text,
};
use native_theme_gpui::{AccessibilityPreferences, apply, from_preset};

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
}

impl Focusable for Application {
    fn focus_handle(&self, _context: &gpui_kit::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Application {
    fn render(&mut self, _window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        let theme = context.theme();

        component::application_content(theme)
            .track_focus(&self.focus_handle)
            .child(component::title_bar(context, &self.menu_bar))
            .when_some(self.image.as_ref(), |this, image| {
                this.child(component::file_tree(
                    image.read(context).tree_state().clone(),
                ))
                .on_action(context.listener(|this, action: &actions::Rename, window, context| {}))
                .on_action(context.listener(|this, action: &actions::Delete, window, context| {}))
            })
            .when_none(&self.image, |this| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .h_full()
                        .w_full()
                        .child(text!("open or create image file")),
                )
            })
            .on_action(
                context.listener(|_, _: &actions::CreateImage, window, context| {
                    dialog::show_dialog(
                        window,
                        context,
                        dialog::CreateImageDialog::new,
                        image::handle_dialog_result,
                    );
                }),
            )
            .on_action(
                context.listener(|_, _: &actions::OpenImage, window, context| {
                    dialog::show_dialog(
                        window,
                        context,
                        dialog::OpenImageDialog::new,
                        image::handle_dialog_result,
                    );
                }),
            )
            .on_action(
                context.listener(|this, _: &actions::CloseImage, window, context| {
                    this.image = None;
                    context.notify();

                    let focus_handle = this.focus_handle.clone();

                    window.defer(context, move |window, context| {
                        window.focus(&focus_handle, context);
                    });
                }),
            )
    }
}

fn main() {
    let application = application().with_assets(gpui_kit::assets::Assets);

    application.run(|context| {
        gpui_kit::init(context);

        context.on_action(|_: &actions::Quit, context| {
            context.quit();
        });

        let preference = AccessibilityPreferences::from_system();
        if let Ok((theme, resolved)) = from_preset("one-dark", true, &preference) {
            apply(theme, &resolved, &preference, context);
        }

        app_menu::initialize(context);

        gpui_kit::open_window(window_options(), context, |window, context| {
            context.new(|context| Application::new(window, context))
        })
        .expect("Failed to open window");

        context.activate(true);
    });
}

fn window_options() -> WindowOptions {
    let mut window_options = TitleBar::window_options();

    if let Some(title_bar) = window_options.titlebar.as_mut() {
        title_bar.title = Some("img".into());
        title_bar.appears_transparent = true;
    }

    window_options.window_bounds = Some(WindowBounds::Windowed(Bounds {
        origin: point(px(100.0), px(100.0)),
        size: size(px(600.0), px(750.0)),
    }));

    window_options
}

impl Global for Application {}
