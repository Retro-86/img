use crate::ApplicationMessage;
use crate::error::Error;
use crate::modal::ModalWindow;
use iced::widget::{Column, button, container, text};
use iced::{Alignment, Element, Length};

#[derive(Debug, PartialEq, Clone)]
pub struct Modal {
    error: Error,
}

impl Modal {
    pub fn new(error: Error) -> Self {
        Modal { error }
    }
}

impl ModalWindow for Modal {
    type Message = ApplicationMessage;

    fn view(&self) -> Element<'_, Self::Message> {
        container(
            Column::new()
                .spacing(20)
                .align_x(Alignment::Center)
                .push(text("Ошибка").size(18).style(text::danger))
                .push(text(format!("{}", self.error)))
                .push(
                    button(text("Закрыть"))
                        .on_press(ApplicationMessage::CloseModal)
                        .style(button::primary),
                ),
        )
        .width(Length::Shrink)
        .padding(20)
        .style(container::rounded_box)
        .into()
    }
}
