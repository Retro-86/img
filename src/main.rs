pub mod component;

use gpui_kit::component::menu::DropdownMenu;
use gpui_kit::component::{ActiveTheme, TitleBar};
use gpui_kit::{
    AppContext, Context, InteractiveElement, IntoElement, ParentElement, Render, SharedString,
    Styled, TitlebarOptions, Window, WindowOptions, actions, application,
};
use native_theme_gpui::{AccessibilityPreferences, apply, from_preset};

pub struct Application;

impl Render for Application {
    fn render(&mut self, window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        let theme = context.theme();

        component::application_content(theme)
            .child(
                TitleBar::new().child(
                    component::title_item(
                        component::button("file")
                            .label("File")
                            .text_color(theme.foreground)
                            .dropdown_menu(|menu, _, _| {
                                menu.menu("Open image", Box::new(OpenImage))
                                    .menu("Create image", Box::new(CreateImage))
                                    .separator()
                                    .menu("Quit", Box::new(Quit))
                            }),
                    )
                    .child(component::title_item(
                        component::button("image")
                            .label("Image")
                            .text_color(theme.foreground)
                            .dropdown_menu(|menu, _, _| {
                                menu.menu("Create file", Box::new(CreateFile))
                                    .menu("Add file", Box::new(AddFile))
                            }),
                    ))
                    .bg(theme.title_bar),
                ),
            )
            .on_action::<Quit>(|_, _, context| context.quit())
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

        let mut window_options = WindowOptions::default();

        let mut title_bar_options = TitlebarOptions::default();
        title_bar_options.title = Some(SharedString::from("img"));
        title_bar_options.appears_transparent = true;

        window_options.titlebar = Some(title_bar_options);

        gpui_kit::open_window(window_options, context, |window, context| {
            context.new(|_| Application)
        })
        .expect("Failed to open window");
    });
}

actions!(application, [Quit]);
actions!(image, [CreateImage, OpenImage, AddFile, CreateFile]);
