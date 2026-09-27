use crate::image::ImageFile;
use crate::modal::ModalWindow;
use iced::widget::{Column, Row, button, container, slider, text, text_input};
use iced::{Alignment, Element, Task};
use std::ops::RangeInclusive;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum Message {
    ImageFileNameChanged(String),
    ImageFilePathChanged(String),
    ImageFileSelectPath,
    ImageFilePathSelected(PathBuf),
    ImageFileSizeChanged(u32),
    ImageFileCreate,
}

pub enum Event {
    Message(Message),
    Submitted(ImageFile),
}

#[derive(Clone, Debug)]
pub struct Modal {
    file_name: String,
    file_path: String,
    image_size: u32,
}

impl Default for Modal {
    fn default() -> Self {
        Self {
            file_name: "image".to_string(),
            file_path: "".to_string(),
            image_size: 2,
        }
    }
}

impl ModalWindow for Modal {
    type Message = Message;

    fn view(&self) -> Element<'_, Self::Message> {
        container(
            Column::new()
                .spacing(20)
                .align_x(Alignment::Center)
                .push(
                    Column::new()
                        .push(text("Image file name"))
                        .push(
                            text_input("image", &self.file_name)
                                .on_input(Message::ImageFileNameChanged),
                        )
                        .spacing(2),
                )
                .push(
                    Column::new()
                        .push(text("Image file path"))
                        .push(
                            Row::new()
                                .spacing(10)
                                .align_y(Alignment::Center)
                                .push(
                                    text_input("path", &self.file_path)
                                        .on_input(Message::ImageFilePathChanged),
                                )
                                .push(button("Select").on_press(Message::ImageFileSelectPath)),
                        )
                        .spacing(2),
                )
                .push(
                    Column::new()
                        .push(text(format!("Image size - {}MiB", self.image_size)))
                        .push(
                            slider(
                                RangeInclusive::new(1, 2048),
                                self.image_size,
                                Message::ImageFileSizeChanged,
                            )
                            .step(1u32)
                            .width(400),
                        )
                        .spacing(2),
                )
                .push(
                    button("Create")
                        .style(button::primary)
                        .on_press(Message::ImageFileCreate),
                ),
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
            Message::ImageFileNameChanged(value) => self.file_name = value,
            Message::ImageFilePathChanged(value) => self.file_path = value,
            Message::ImageFileSizeChanged(value) => self.image_size = value,
            Message::ImageFileCreate => {
                if self.file_path.is_empty() || self.file_name.is_empty() {
                    
                }

                return Task::done(Event::Submitted(self.image_file()))
            },
            Message::ImageFileSelectPath => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    return Task::done(Event::Message(Message::ImageFilePathSelected(path)));
                }
            }

            Message::ImageFilePathSelected(path) => {
                self.file_path = path.to_string_lossy().into_owned();
            }
        }

        Task::none()
    }

    fn image_file(&self) -> ImageFile {
        let image_size = self.image_size as u64 * 1_048_576;

        ImageFile::new(
            PathBuf::from(&self.file_path).join(format!("{}.img", self.file_name)),
            *b"EMULATED_FS",
            image_size,
        )
    }
}
