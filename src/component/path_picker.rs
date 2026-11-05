use gpui_kit::assets::IconName;
use gpui_kit::base::input::{InputBaseState, InputMode, InputState};
use gpui_kit::base::v_flex;
use gpui_kit::component::button::ButtonVariants;
use gpui_kit::component::input::{
    InputGroup, InputGroupAddon, InputGroupAddonAlignment, InputGroupButton, InputGroupInput,
};
use gpui_kit::component::label::Label;
use gpui_kit::{AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window};

pub struct PathPicker {
    state: Entity<InputBaseState<InputMode>>,
    label: String,
}

impl PathPicker {
    pub fn new(context: &mut Context<Self>, window: &mut Window, label: &str) -> Self {
        let state = context.new(|context| InputState::new(window, context));

        Self {
            state,
            label: label.to_string(),
        }
    }
}

impl Render for PathPicker {
    fn render(&mut self, window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        v_flex().gap_2().child(Label::new(&self.label)).child(
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
                                .icon(IconName::Folder),
                        ),
                ),
        )
    }
}
