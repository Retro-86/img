use crate::ApplicationMessage;
use crate::modal::ModalWindow;
use iced::widget::{Button, Column, button, container, row, text};
use iced::{Alignment, Element, Length};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ExitModal;

impl ModalWindow for ExitModal {
    type Message = ApplicationMessage;

    fn view(&self) -> Element<'_, Self::Message> {
        container(
            Column::new()
                .spacing(20)
                .align_x(Alignment::Center)
                .push(
                    text("Вы действительно хотите выйти?")
                        .size(18)
                        .align_x(Alignment::Center),
                )
                .push(
                    row!(
                        Button::new(text("Выйти"))
                            .on_press(ApplicationMessage::Exit)
                            .style(button::danger),
                        Button::new(text("Отмена"))
                            .on_press(ApplicationMessage::CloseModal)
                            .style(button::primary)
                    )
                    .spacing(20),
                ),
        )
        .width(Length::Shrink)
        .padding(20)
        .style(container::rounded_box)
        .into()
    }
}
