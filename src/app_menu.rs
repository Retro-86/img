use crate::actions;
use gpui_kit::{App, Menu, MenuItem};

pub fn initialize(context: &mut App) {
    let menus = vec![
        Menu {
            name: "File".into(),
            items: vec![
                MenuItem::action("Open image", actions::OpenImage),
                MenuItem::action("Create image", actions::CreateImage),
                MenuItem::action("Close image", actions::CloseImage),
                MenuItem::separator(),
                MenuItem::action("Quit", actions::Quit),
            ],
            disabled: false,
        },
        Menu {
            name: "Image".into(),
            items: vec![
                MenuItem::action("Create file", actions::CreateFile),
                MenuItem::action("Add file", actions::AddFile),
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
}
