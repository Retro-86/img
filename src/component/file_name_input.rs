use gpui_kit::base::input::{InputBaseState, InputMode, InputState};
use gpui_kit::base::v_flex;
use gpui_kit::component::input::{
    InputGroup, InputGroupAddon, InputGroupAddonAlignment, InputGroupInput, InputGroupText,
};
use gpui_kit::component::label::Label;
use gpui_kit::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window,
};

pub struct FileNameInput {
    state: Entity<InputBaseState<InputMode>>,
    label: String,
    extension: String,
}

impl FileNameInput {
    pub fn new(
        window: &mut Window,
        context: &mut Context<Self>,
        label: &str,
        extension: &str,
        placeholder: &str,
    ) -> Self {
        let state = context.new(|context| {
            InputState::new(window, context)
                .default_value(placeholder)
                .placeholder(placeholder)
        });

        Self {
            state,
            label: label.to_string(),
            extension: extension.to_string(),
        }
    }

    pub fn state(&self) -> &Entity<InputBaseState<InputMode>> {
        &self.state
    }

    pub fn value(&self, context: &App) -> String {
        self.state.read(context).value().to_string()
    }
}

impl Render for FileNameInput {
    fn render(&mut self, _window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        let value = self.value(context);
        let count = value.chars().count();

        v_flex().gap_2().child(Label::new(&self.label)).child(
            InputGroup::new("file-name")
                .invalid(count > 255)
                .input(InputGroupInput::new(&self.state))
                .addon(
                    InputGroupAddon::new(self.label.to_lowercase())
                        .align(InputGroupAddonAlignment::InlineEnd)
                        .child(InputGroupText::new().child(self.extension.clone())),
                ),
        )
    }
}
