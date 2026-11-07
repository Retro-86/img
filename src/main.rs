pub mod component;
pub mod dialog;
pub mod event;
pub mod file_system;

use fatfs::FileSystem;
use fscommon::BufStream;
use gpui_kit::component::menu::AppMenuBar;
use gpui_kit::component::{ActiveTheme, TitleBar};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    AppContext, Bounds, Context, Entity, FocusHandle, Global, InteractiveElement, IntoElement,
    Menu, MenuItem, ParentElement, Render, Window, WindowBounds, actions, application, point, px,
    size,
};
use native_theme_gpui::{AccessibilityPreferences, apply, from_preset};
use std::fs::File;
use std::sync::Arc;

pub struct Application {
    menu_bar: Entity<AppMenuBar>,
    focus_handle: FocusHandle,
    file_system: Option<Arc<FileSystem<BufStream<File>>>>,
}

impl Application {
    pub fn new(window: &mut Window, context: &mut Context<Self>) -> Self {
        let focus_handle = context.focus_handle();
        window.focus(&focus_handle, context);

        Self {
            menu_bar: AppMenuBar::new(context),
            focus_handle,
            file_system: None,
        }
    }
}

impl Render for Application {
    fn render(&mut self, _window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        let theme = context.theme();

        component::application_content(theme)
            .track_focus(&self.focus_handle)
            .child(component::title_bar(context, &self.menu_bar))
            .when_some(self.file_system.as_ref(), |this, file_system| {
                this.child(component::file_tree(context, file_system.clone()))
            })
            .on_action(context.listener(|_, _: &CreateImage, window, context| {
                dialog::CreateImageDialog::open(window, context);
            }))
            .on_action(context.listener(|_, _: &Quit, _, context| {
                context.quit();
            }))
            .on_action(context.listener(|this, _: &CloseImage, _, _| this.file_system = None))
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
                    MenuItem::action("Close image", CloseImage),
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
        {
            use gpui_kit::component::GlobalState;
            GlobalState::global_mut(context)
                .set_app_menus(menus.into_iter().map(Menu::owned).collect());
        }

        let mut window_options = TitleBar::window_options();

        if let Some(title_bar) = window_options.titlebar.as_mut() {
            title_bar.title = Some("img".into());
            title_bar.appears_transparent = true;
        }

        window_options.window_bounds = Some(WindowBounds::Windowed(Bounds {
            origin: point(px(100.0), px(100.0)),
            size: size(px(600.0), px(750.0)),
        }));

        gpui_kit::open_window(window_options, context, |window, context| {
            context.new(|context| Application::new(window, context))
        })
        .expect("Failed to open window");

        context.activate(true);
    });
}

impl Global for Application {}

actions!(
    application,
    [
        Quit,
        CreateImage,
        OpenImage,
        AddFile,
        CreateFile,
        CloseImage
    ]
);
