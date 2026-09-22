#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod image;
mod modal;

use crate::image::ImageFile;
use crate::modal::{ErrorModal, ExitModal, Modal};
use fatfs::FileSystem;
use fscommon::BufStream;
use iced::widget::{Button, Column, button, container, stack, text};
use iced::{Alignment, Background, Color, Element, Length, Size, Task, Theme, window};
use iced_aw::{menu, menu_bar};
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::path::PathBuf;

fn main() -> iced::Result {
    iced::application(Application::default, Application::update, Application::view)
        .theme(Application::theme)
        .title("img")
        .window_size(Size::new(1000.0, 600.0))
        .run()
}

#[derive(Default)]
struct Application {
    theme: Option<Theme>,
    file_system: Option<FileSystem<BufStream<File>>>,
    active_modal: Option<Modal>,
}

#[derive(Debug, Clone)]
enum Message {
    SelectImageFile,
    OpenImageFile(PathBuf),
    CreateImageFile(ImageFile),
    Exit,
    OpenModal(Modal),
    CloseModal,
}

#[derive(Debug, Clone, PartialEq)]
enum Error {
    ImageFileCreateFailed(PathBuf),
    ImageFileOpenFailed(PathBuf),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ImageFileCreateFailed(path) => {
                write!(formatter, "Failed to create image file: {}", path.display())
            }
            Error::ImageFileOpenFailed(path) => {
                write!(formatter, "Failed to open image file: {}", path.display())
            }
        }
    }
}

impl Application {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectImageFile => {
                let picked_image_file_path = rfd::FileDialog::new()
                    .add_filter("Image file", &["img"])
                    .pick_file();

                if let Some(image_file_path) = picked_image_file_path {
                    return Task::done(Message::OpenImageFile(image_file_path));
                };
            }
            Message::OpenImageFile(path) => {
                let file_system = image::open(&path);

                if let Ok(file_system) = file_system {
                    self.file_system = Some(file_system);
                } else {
                    self.show_error(Error::ImageFileOpenFailed(path));
                }
            }
            Message::CreateImageFile(image_file) => {
                let file_system = image::create(&image_file);

                if let Ok(image_file) = file_system {
                    self.file_system = Some(image_file);
                } else {
                    self.show_error(Error::ImageFileCreateFailed(image_file.path))
                }
            }
            Message::OpenModal(modal) => self.active_modal = Some(modal),
            Message::Exit => return window::latest().and_then(window::close),
            Message::CloseModal => self.active_modal = None,
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let file_menu = menu!(
            (Button::new(text("Открыть образ").size(14))
                .on_press(Message::SelectImageFile)
                .style(button::text)
                .width(Length::Fill)),
            (Button::new(text("Создать образ").size(14))
                .style(button::text)
                .width(Length::Fill)),
            (Button::new(text("Выход").size(14))
                .on_press(Message::OpenModal(Modal::Exit(ExitModal::default())))
                .style(button::text)
                .width(Length::Fill))
        )
        .width(150.0);

        let menu_bar = menu_bar!((text("Файл").size(14), file_menu),);

        let content = Column::new().padding(8).spacing(10).push(menu_bar);

        let layout: Element<'_, Message> = container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        if let Some(modal) = &self.active_modal {
            let modal_content = modal.view();

            let overlay = button(
                container(modal_content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
            )
            .on_press(Message::CloseModal)
            .style(button::text)
            .width(Length::Fill)
            .height(Length::Fill);

            let styled_overlay = container(overlay)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_| container::Style {
                    background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
                    ..Default::default()
                })
                .align_x(Alignment::Center)
                .align_y(Alignment::Center);

            stack![layout, styled_overlay].into()
        } else {
            layout
        }
    }

    fn show_error(&mut self, error: Error) {
        self.active_modal = Some(Modal::Error(ErrorModal::new(error)));
    }

    fn theme(&self) -> Theme {
        if self.theme.is_none() {
            Theme::Dark
        } else {
            self.theme.clone().unwrap()
        }
    }
}
