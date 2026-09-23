pub mod create;
pub mod error;
pub mod exit;

use crate::ApplicationMessage;
use crate::modal::create::{CreateModal, CreateModalMessage};
use crate::modal::error::ErrorModal;
use crate::modal::exit::ExitModal;
use iced::{Element, Task};

pub trait ModalWindow {
    type Message;

    fn view(&self) -> Element<'_, Self::Message>;
}

#[derive(Debug, Clone)]
pub enum ModalMessage {
    Create(CreateModalMessage),
    Exit,
    Error,
}

#[derive(Debug, Clone)]
pub enum Modal {
    Exit(ExitModal),
    Error(ErrorModal),
    Create(CreateModal),
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
        match message {
            ModalMessage::Create(CreateModalMessage::ImageFileCreate) => {
                if let Modal::Create(window) = self {
                    return Task::done(ApplicationMessage::CreateImageFile(
                        window.create_image_file(),
                    ));
                }

                Task::none()
            }

            ModalMessage::Create(message) => {
                if let Modal::Create(window) = self {
                    return window.update(message).map(|message| {
                        ApplicationMessage::ModalAction(ModalMessage::Create(message))
                    });
                }

                Task::none()
            }

            _ => Task::none(),
        }
    }
}
