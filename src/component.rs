use gpui_kit::base::input::{InputBaseState, InputMode};
use gpui_kit::base::v_flex;
use gpui_kit::component::button::Button;
use gpui_kit::component::input::InputState;
use gpui_kit::component::label::Label;
use gpui_kit::component::menu::AppMenuBar;
use gpui_kit::component::slider::{Slider, SliderEvent, SliderState};
use gpui_kit::component::{ActiveTheme, Theme, TitleBar};
use gpui_kit::{
    App, AppContext, Context, Div, ElementId, Entity, IntoElement, ParentElement, Render, Styled,
    Subscription, Window, div, px, transparent_black,
};
use std::range::RangeInclusive;

pub fn button(id: impl Into<ElementId>) -> Button {
    Button::new(id)
        .border_0()
        .rounded_sm()
        .p(px(6.0))
        .bg(transparent_black())
}

pub fn application_content(theme: &Theme) -> Div {
    div()
        .size_full()
        .flex()
        .flex_col()
        .bg(theme.background)
        .text_color(theme.foreground)
        .font_family("Nordwand Mono")
}

pub fn title_items(children: Vec<impl IntoElement>) -> Div {
    div().flex().items_center().children(children)
}

pub struct Input {
    state: Entity<InputBaseState<InputMode>>,
    label: String,
}

impl Input {
    fn new(
        context: &mut Context<Self>,
        window: &mut Window,
        label: &str,
        placeholder: &str,
    ) -> Self {
        let state =
            context.new(|context| InputState::new(window, context).placeholder(placeholder));

        Self {
            state,
            label: label.to_string(),
        }
    }
}

impl Render for Input {
    fn render(&mut self, window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(Label::new(&self.label))
            .child(gpui_kit::component::input::Input::new(&self.state))
    }
}

pub fn input_with_label(
    context: &mut App,
    window: &mut Window,
    label: &str,
    placeholder: &str,
) -> Entity<Input> {
    context.new(|context| Input::new(context, window, label, placeholder))
}

pub struct SizeSelect {
    state: Entity<SliderState>,
    subscription: Subscription,
    value: u32,
}

impl SizeSelect {
    pub fn new(context: &mut Context<Self>, range: RangeInclusive<u32>, step: u32) -> Self {
        let state = context.new(|_context| {
            SliderState::new()
                .min(range.start as f32)
                .max(range.last as f32)
                .default_value(16.0)
                .step(step as f32)
        });

        let subscription = context.subscribe(
            &state,
            |this, state, event: &SliderEvent, context| match event {
                SliderEvent::Change(value) => {
                    this.value = value.start() as u32;
                    context.notify()
                }
                _ => {}
            },
        );

        Self {
            state,
            subscription,
            value: 16,
        }
    }
}

impl Render for SizeSelect {
    fn render(&mut self, _window: &mut Window, _context: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(Label::new(format!("Size {}mb", self.value)))
            .child(Slider::new(&self.state))
    }
}

pub fn size_select(context: &mut App, range: RangeInclusive<u32>, step: u32) -> Entity<SizeSelect> {
    context.new(|context| SizeSelect::new(context, range, step))
}

pub fn title_bar(context: &mut App, menu_bar: &Entity<AppMenuBar>) -> TitleBar {
    let theme = context.theme();

    TitleBar::new().child(menu_bar.clone()).bg(theme.title_bar)
}
