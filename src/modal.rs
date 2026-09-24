pub mod create;
pub mod error;
pub mod exit;

use crate::ApplicationMessage;
use iced::widget::container;
use iced::{Alignment, Background, Color, Element, Length, Task};

pub trait ModalWindow {
    type Message;

    fn view(&self) -> Element<'_, Self::Message>;
}

#[derive(Debug, Clone)]
pub enum ModalMessage {
    Create(create::Message),
    Exit,
    Error,
}

#[derive(Debug, Clone)]
pub enum Modal {
    Exit(exit::Modal),
    Error(error::Modal),
    Create(create::Modal),
}

impl Modal {
    pub fn view(&self) -> Element<'_, ApplicationMessage> {
        match self {
            Modal::Exit(window) => window.view(),
            Modal::Error(window) => window.view(),
            Modal::Create(window) => window
                .view()
                .map(ModalMessage::Create)
                .map(ApplicationMessage::ModalAction),
        }
    }

    pub fn update(&mut self, message: ModalMessage) -> Task<ApplicationMessage> {
        match (self, message) {
            (Self::Create(modal), ModalMessage::Create(message)) => {
                modal.update(message).map(map_create_event)
            }

            _ => Task::none(),
        }
    }
}

fn map_create_event(event: create::Event) -> ApplicationMessage {
    match event {
        create::Event::Message(message) => {
            ApplicationMessage::ModalAction(ModalMessage::Create(message))
        }
        create::Event::Submitted(image) => ApplicationMessage::CreateImageFile(image),
    }
}

pub fn open(modal: &Modal) -> Element<'_, ApplicationMessage> {
    let modal_content = modal.view();

    container(modal_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
            ..Default::default()
        })
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .into()
}
