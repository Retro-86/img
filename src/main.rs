pub mod actions;
pub mod app_menu;
pub mod application;
pub mod component;
pub mod dialog;
pub mod event;
pub mod file_system;
pub mod image;

use crate::application::Application;
use gpui_kit::component::TitleBar;
use gpui_kit::{
    AppContext, Bounds, WindowBounds, WindowOptions, actions, application, point, px, size,
};
use native_theme_gpui::{AccessibilityPreferences, apply, from_preset};

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
