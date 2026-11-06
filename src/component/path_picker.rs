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

pub struct PathPicker {
    state: Entity<InputBaseState<InputMode>>,
    label: SharedString,
}

impl PathPicker {
    pub fn new(window: &mut Window, context: &mut Context<Self>, label: &str) -> Self {
        let state = context.new(|context| InputState::new(window, context));

        Self {
            state,
            label: SharedString::from(label),
        }
    }

    fn select_folder(
        &mut self,
        _event: &ClickEvent,
        window: &mut Window,
        context: &mut Context<Self>,
    ) {
        context
            .spawn_in(window, async move |this, context| {
                let Some(selected_path) = rfd::AsyncFileDialog::new().pick_folder().await else {
                    return;
                };

                let value = SharedString::from(selected_path.path().to_string_lossy().to_string());

                this.update_in(context, |this, window, context| {
                    this.state.update(context, |state, context| {
                        state.set_value(value, window, context);
                    });
                })
                .unwrap();
            })
            .detach();
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
                                    .on_click(context.listener(Self::select_folder)),
                            ),
                    ),
            )
    }
}
