use gpui_kit::assets::IconName;
use gpui_kit::base::input::{InputBaseState, InputMode, InputState};
use gpui_kit::base::v_flex;
use gpui_kit::component::button::ButtonVariants;
use gpui_kit::component::input::{
    InputGroup, InputGroupAddon, InputGroupAddonAlignment, InputGroupButton, InputGroupInput,
};
use gpui_kit::component::label::Label;
use gpui_kit::{
    App, AppContext, ClickEvent, Context, Entity, IntoElement, ParentElement, Render, SharedString,
    Styled, Window,
};
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PickerMode {
    File,
    Folder,
}

pub struct PathPicker {
    state: Entity<InputBaseState<InputMode>>,
    label: SharedString,
    mode: PickerMode,
}

impl PathPicker {
    pub fn new(
        window: &mut Window,
        context: &mut Context<Self>,
        label: &str,
        mode: PickerMode,
    ) -> Self {
        let state = context.new(|context| InputState::new(window, context));

        Self {
            state,
            label: SharedString::from(label),
            mode,
        }
    }

    pub fn file(window: &mut Window, context: &mut Context<Self>, label: &str) -> Self {
        Self::new(window, context, label, PickerMode::File)
    }

    pub fn folder(window: &mut Window, context: &mut Context<Self>, label: &str) -> Self {
        Self::new(window, context, label, PickerMode::Folder)
    }

    fn select(&mut self, _event: &ClickEvent, window: &mut Window, context: &mut Context<Self>) {
        let mode = self.mode;

        context
            .spawn_in(window, async move |this, context| {
                let dialog = rfd::AsyncFileDialog::new();
                let selected = match mode {
                    PickerMode::File => dialog.pick_file().await,
                    PickerMode::Folder => dialog.pick_folder().await,
                };

                let Some(selected) = selected else {
                    return;
                };

                let value = path_to_shared_string(selected.path());

                this.update_in(context, |this, window, context| {
                    this.set_value(value, window, context);
                })
                .unwrap();
            })
            .detach();
    }

    fn set_value(&mut self, value: SharedString, window: &mut Window, context: &mut Context<Self>) {
        self.state.update(context, |state, context| {
            state.set_value(value, window, context);
        });
    }

    pub fn value(&self, context: &App) -> String {
        self.state.read(context).value().to_string()
    }
}

impl Render for PathPicker {
    fn render(&mut self, _window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(Label::new(self.label.clone()))
            .child(
                InputGroup::new("path-picker")
                    .input(InputGroupInput::new(&self.state))
                    .addon(
                        InputGroupAddon::new("path-picker-addon")
                            .align(InputGroupAddonAlignment::InlineEnd)
                            .p_0()
                            .h_full()
                            .child(
                                InputGroupButton::new("select")
                                    .primary()
                                    .h_full()
                                    .w_8()
                                    .rounded_lg()
                                    .rounded_l_none()
                                    .icon(IconName::Folder)
                                    .on_click(context.listener(Self::select)),
                            ),
                    ),
            )
    }
}

fn path_to_shared_string(path: &Path) -> SharedString {
    SharedString::from(path.to_string_lossy().to_string())
}
