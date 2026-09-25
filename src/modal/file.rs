use crate::modal::ModalWindow;
use iced::widget::{Column, button, container, text, text_input};
use iced::{Element, Task};

#[derive(Clone, Debug)]
pub enum CreateMessage {
    FileNameChanged(String),
    Submit,
}

pub enum CreateEvent {
    Message(CreateMessage),
    Submitted(String),
}

#[derive(Default, Debug, Clone)]
pub struct CreateModal {
    file_name: String,
}

impl ModalWindow for CreateModal {
    type Message = CreateMessage;

    fn view(&self) -> Element<'_, Self::Message> {
        container(
            Column::new()
                .spacing(20)
                .push(text("File name"))
                .push(
                    text_input("name..", &self.file_name).on_input(CreateMessage::FileNameChanged),
                )
                .push(button("Create").on_press(CreateMessage::Submit)),
        )
        .width(300)
        .padding(20)
        .style(container::rounded_box)
        .into()
    }
}

impl CreateModal {
    pub fn update(&mut self, message: CreateMessage) -> Task<CreateEvent> {
        match message {
            CreateMessage::FileNameChanged(file_name) => {
                self.file_name = file_name;
            }
            CreateMessage::Submit => {
                return Task::done(CreateEvent::Submitted(self.file_name.clone()));
            }
        }

        Task::none()
    }
}
