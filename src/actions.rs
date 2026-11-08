use gpui_kit::private::schemars;
use gpui_kit::{Action, SharedString, actions};
use serde::Deserialize;

actions!(application, [Quit, CreateImage, OpenImage, CloseImage]);
actions!(image, [AddFile, CreateFile]);

#[derive(Action, Clone, PartialEq, Deserialize, schemars::JsonSchema)]
#[action(namespace = file_tree)]
pub struct Rename {
    pub identifier: SharedString,
}

#[derive(Action, Clone, PartialEq, Deserialize, schemars::JsonSchema)]
#[action(namespace = file_tree)]
pub struct Delete {
    pub identifier: SharedString,
}
