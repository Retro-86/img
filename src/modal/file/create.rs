use crate::modal::ModalWindow;
use iced::widget::{Column, button, container, text, text_input};
use iced::{Element, Task};

#[derive(Clone, Debug)]
pub enum Message {
    FileNameChanged(String),
    Submit,
}

pub enum Event {
    Message(Message),
    Submitted(String),
}

#[derive(Default, Debug, Clone)]
pub struct Modal {
    file_name: String,
}

impl ModalWindow for Modal {
    type Message = Message;

    fn view(&self) -> Element<'_, Self::Message> {
        container(
            Column::new()
                .spacing(20)
                .push(text("File name"))
                .push(
                    text_input("name..", &self.file_name).on_input(Message::FileNameChanged),
                )
                .push(button("Create").on_press(Message::Submit)),
        )
            .width(300)
            .padding(20)
            .style(container::rounded_box)
            .into()
    }
}

impl Modal {
    pub fn update(&mut self, message: Message) -> Task<Event> {
        match message {
            Message::FileNameChanged(file_name) => {
                self.file_name = file_name;
            }
            Message::Submit => {
                return Task::done(Event::Submitted(self.file_name.clone()));
            }
        }

        Task::none()
    }
}
