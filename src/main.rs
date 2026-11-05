pub mod component;
pub mod dialog;

use std::collections::binary_heap::Iter;
use gpui_kit::component::menu::AppMenuBar;
use gpui_kit::component::{ActiveTheme, GlobalState, TitleBar};
use gpui_kit::{
    AppContext, Context, Entity, FocusHandle, InteractiveElement, IntoElement, Menu, MenuItem,
    ParentElement, Render, Window, actions, application,
};
use native_theme_gpui::{AccessibilityPreferences, apply, from_preset};

pub struct Application {
    menu_bar: Entity<AppMenuBar>,
    focus_handle: FocusHandle,
}

impl Application {
    pub fn new(window: &mut Window, context: &mut Context<Self>) -> Self {
        let focus_handle = context.focus_handle();
        window.focus(&focus_handle, context);

        Self {
            menu_bar: AppMenuBar::new(context),
            focus_handle,
        }
    }
}

impl Render for Application {
    fn render(&mut self, _window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        let theme = context.theme();

        component::application_content(theme)
            .track_focus(&self.focus_handle)
            .child(component::title_bar(context, &self.menu_bar))
            .on_action(context.listener(|_, _: &CreateImage, window, context| {
                dialog::CreateImageDialog::open(window, context);
            }))
            .on_action(context.listener(|_, _: &Quit, _, context| {
                context.quit();
            }))
    }
}

fn main() {
    let application = application().with_assets(gpui_kit::assets::Assets);

    application.run(|context| {
        gpui_kit::init(context);

        let preference = AccessibilityPreferences::from_system();
        if let Ok((theme, resolved)) = from_preset("one-dark", true, &preference) {
            apply(theme, &resolved, &preference, context);
        }

        let menus = vec![
            Menu {
                name: "File".into(),
                items: vec![
                    MenuItem::action("Open image", OpenImage),
                    MenuItem::action("Create image", CreateImage),
                    MenuItem::separator(),
                    MenuItem::action("Quit", Quit),
                ],
                disabled: false,
            },
            Menu {
                name: "Image".into(),
                items: vec![
                    MenuItem::action("Create file", CreateFile),
                    MenuItem::action("Add file", AddFile),
                ],
                disabled: false,
            },
        ];

        #[cfg(target_os = "macos")]
        context.set_menus(menus);

        #[cfg(not(target_os = "macos"))]
        GlobalState::global_mut(context)
            .set_app_menus(menus.into_iter().map(Menu::owned).collect());


        let mut window_options = TitleBar::window_options();

        if let Some(title_bar) = window_options.titlebar.as_mut() {
            title_bar.title = Some("img".into());
            title_bar.appears_transparent = true;
        }

        gpui_kit::open_window(window_options, context, |window, context| {
            context.new(|context| Application::new(window, context))
        })
        .expect("Failed to open window");

        context.activate(true);
    });
}

actions!(
    application,
    [Quit, CreateImage, OpenImage, AddFile, CreateFile]
);
