use gpui_kit::base::slider::{SliderEvent, SliderState};
use gpui_kit::base::v_flex;
use gpui_kit::component::label::Label;
use gpui_kit::component::slider::Slider;
use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, Window,
};
use std::range::RangeInclusive;

pub struct SizeSelect {
    state: Entity<SliderState>,
    _subscription: Subscription,
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

        let subscription =
            context.subscribe(
                &state,
                |this, _, event: &SliderEvent, context| match event {
                    SliderEvent::Change(value) => {
                        this.value = value.start() as u32;
                        context.notify()
                    }
                    _ => {}
                },
            );

        Self {
            state,
            _subscription: subscription,
            value: 16,
        }
    }

    pub fn value(&self) -> u32 {
        self.value
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
