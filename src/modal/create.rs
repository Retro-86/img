use crate::image::ImageFile;
use crate::modal::ModalWindow;
use iced::widget::{Column, Row, button, container, slider, text_input};
use iced::{Alignment, Element, Task};
use std::ops::RangeInclusive;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum CreateModalMessage {
    ImageFileNameChanged(String),
    ImageFilePathChanged(String),
    ImageFileSelectPath,
    ImageFilePathSelected(PathBuf),
    ImageFileSizeChanged(u32),
    ImageFileCreate,
}

#[derive(Clone, Debug)]
pub struct CreateModal {
    file_name: String,
    file_path: String,
    image_size: u32,
}

impl Default for CreateModal {
    fn default() -> Self {
        Self {
            file_name: "image".to_string(),
            file_path: "".to_string(),
            image_size: 2,
        }
    }
}

impl ModalWindow for CreateModal {
    type Message = CreateModalMessage;

    fn view(&self) -> Element<'_, Self::Message> {
        container(
            Column::new()
                .spacing(20)
                .align_x(Alignment::Center)
                .push(
                    text_input("Image file name", &self.file_name)
                        .on_input(CreateModalMessage::ImageFileNameChanged),
                )
                .push(
                    Row::new()
                        .spacing(20)
                        .align_y(Alignment::Center)
                        .push(
                            text_input("Image file path", &self.file_path)
                                .on_input(CreateModalMessage::ImageFilePathChanged),
                        )
                        .push(button("Select").on_press(CreateModalMessage::ImageFileSelectPath)),
                )
                .push(slider(
                    RangeInclusive::new(1, 2048),
                    self.image_size,
                    CreateModalMessage::ImageFileSizeChanged,
                ))
                .push(
                    button("Create image")
                        .style(button::primary)
                        .on_press(CreateModalMessage::ImageFileCreate),
                ),
        )
        .width(300)
        .padding(20)
        .style(container::rounded_box)
        .into()
    }
}

impl CreateModal {
    pub fn update(&mut self, message: CreateModalMessage) -> Task<CreateModalMessage> {
        match message {
            CreateModalMessage::ImageFileNameChanged(value) => self.file_name = value,
            CreateModalMessage::ImageFilePathChanged(value) => self.file_path = value,
            CreateModalMessage::ImageFileSizeChanged(value) => self.image_size = value,
            CreateModalMessage::ImageFileCreate => return Task::none(),
            CreateModalMessage::ImageFileSelectPath => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    return Task::done(CreateModalMessage::ImageFilePathSelected(path));
                }
            }

            CreateModalMessage::ImageFilePathSelected(path) => {
                self.file_path = path.to_string_lossy().into_owned();
            }
        }

        Task::none()
    }

    pub fn create_image_file(&self) -> ImageFile {
        let image_size = self.image_size as u64 * 1_048_576;

        ImageFile::new(
            PathBuf::from(&self.file_path)
                .join(format!("{}.img", self.file_name)),
            *b"EMULATED_FS",
            image_size,
        )
    }
}
