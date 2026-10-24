pub mod create;
pub mod error;
pub mod exit;
pub mod file;

use crate::ApplicationMessage;
use iced::widget::{container, button, stack, text};
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
    FileCreate(file::create::Message),
}

#[derive(Debug, Clone)]
pub enum Modal {
    Exit(exit::Modal),
    Error(error::Modal),
    Create(create::Modal),
    FileCreate(file::create::Modal),
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
            Modal::FileCreate(window) => window
                .view()
                .map(ModalMessage::FileCreate)
                .map(ApplicationMessage::ModalAction),
        }
    }

    pub fn update(&mut self, message: ModalMessage) -> Task<ApplicationMessage> {
        match (self, message) {
            (Self::Create(modal), ModalMessage::Create(message)) => {
                modal.update(message).map(map_create_event)
            }
            (Self::FileCreate(modal), ModalMessage::FileCreate(message)) => {
                modal.update(message).map(map_file_create_event)
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

fn map_file_create_event(event: file::create::Event) -> ApplicationMessage {
    match event {
        file::create::Event::Message(message) => {
            ApplicationMessage::ModalAction(ModalMessage::FileCreate(message))
        }
        file::create::Event::Submitted(file_name) => ApplicationMessage::CreateFileInImage(file_name),
    }
}

pub fn open(modal: &Modal) -> Element<'_, ApplicationMessage> {
    let modal_content = modal.view();

    let close_button = container(
        button(text("✕"))
            .on_press(ApplicationMessage::CloseModal)
            .style(|_theme, _status| button::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: Color::from_rgb(0.7, 0.7, 0.7),
                ..Default::default()
            })
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::End)       
    .align_y(Alignment::Start)
    .padding(5);

    let modal_container_content = stack![modal_content, close_button];

    let modal_container = container(modal_container_content)
        .style(move |_theme| container::Style {
            background: Some(Background::Color(Color::from_rgb(0.17, 0.19, 0.22))),
            ..Default::default()
        });

    container(modal_container)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .into()
}

