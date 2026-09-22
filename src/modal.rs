use crate::{Error, Message};
use iced::widget::{Button, Column, button, container, row, text};
use iced::{Alignment, Element, Length};

pub trait ModalWindow {
    fn view(&self) -> Element<'_, Message>;
}

#[derive(Debug, Clone)]
pub enum Modal {
    Exit(ExitModal),
    Error(ErrorModal),
    Create(CreateModal),
}

impl Modal {
    pub fn view(&self) -> Element<'_, Message> {
        match self {
            Modal::Exit(window) => window.view(),
            Modal::Error(window) => window.view(),
            Modal::Create(window) => window.view(),
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ExitModal;

impl ModalWindow for ExitModal {
    fn view(&self) -> Element<'_, Message> {
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
                            .on_press(Message::Exit)
                            .style(button::danger),
                        Button::new(text("Отмена"))
                            .on_press(Message::CloseModal)
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

#[derive(Debug, PartialEq, Clone)]
pub struct ErrorModal {
    pub error: Error,
}

impl ErrorModal {
    pub fn new(error: Error) -> Self {
        ErrorModal { error }
    }
}

impl ModalWindow for ErrorModal {
    fn view(&self) -> Element<'_, Message> {
        container(
            Column::new()
                .spacing(20)
                .align_x(Alignment::Center)
                .push(text("Ошибка").size(18).style(text::danger))
                .push(text(format!("{}", self.error)))
                .push(
                    button(text("Закрыть"))
                        .on_press(Message::CloseModal)
                        .style(button::primary),
                ),
        )
        .width(Length::Shrink)
        .padding(20)
        .style(container::rounded_box)
        .into()
    }
}

#[derive(Clone, Debug)]
struct CreateModal {}

impl ModalWindow for CreateModal {
    
    fn view(&self) -> Element<'_, Message> {
        todo!()
    }
}
