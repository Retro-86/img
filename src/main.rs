#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod error;
pub mod image;
mod modal;

use crate::error::Error;
use crate::image::ImageFile;
use crate::modal::create::CreateModal;
use crate::modal::error::ErrorModal;
use crate::modal::exit::ExitModal;
use crate::modal::{Modal, ModalMessage};
use fatfs::FileSystem;
use fscommon::BufStream;
use iced::widget::{Button, Column, button, container, stack, text};
use iced::{Alignment, Background, Color, Element, Length, Size, Task, Theme, window};
use iced_aw::{menu, menu_bar};
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
pub enum ApplicationMessage {
    SelectImageFile,
    OpenImageFile(PathBuf),
    CreateImageFile(ImageFile),
    Exit,
    OpenModal(Modal),
    CloseModal,
    ModalAction(ModalMessage),
}

impl Application {
    fn update(&mut self, message: ApplicationMessage) -> Task<ApplicationMessage> {
        match message {
            ApplicationMessage::SelectImageFile => {
                let picked_image_file_path = rfd::FileDialog::new()
                    .add_filter("Image file", &["img"])
                    .pick_file();

                if let Some(image_file_path) = picked_image_file_path {
                    return Task::done(ApplicationMessage::OpenImageFile(image_file_path));
                };
            }
            ApplicationMessage::OpenImageFile(path) => {
                let file_system = image::open(&path);

                match file_system {
                    Ok(file_system) => self.file_system = Some(file_system),
                    Err(error) => {
                        self.show_error(Error::ImageFileOpenFailed(path, error.to_string()))
                    }
                }
            }
            ApplicationMessage::CreateImageFile(image_file) => {
                let file_system = image::create(&image_file);

                match file_system {
                    Ok(file_system) => self.file_system = Some(file_system),
                    Err(error) => {
                        self.show_error(Error::ImageFileCreateFailed(image_file.path, error.to_string()))
                    }
                }
            }
            ApplicationMessage::OpenModal(modal) => self.active_modal = Some(modal),
            ApplicationMessage::Exit => return window::latest().and_then(window::close),
            ApplicationMessage::CloseModal => self.active_modal = None,
            ApplicationMessage::ModalAction(modal_message) => {
                if let Some(modal) = &mut self.active_modal {
                    return modal.update(modal_message);
                }
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, ApplicationMessage> {
        let file_menu = menu!(
            (Button::new(text("Открыть образ").size(14))
                .on_press(ApplicationMessage::SelectImageFile)
                .style(button::text)
                .width(Length::Fill)),
            (Button::new(text("Создать образ").size(14))
                .on_press(ApplicationMessage::OpenModal(Modal::Create(
                    CreateModal::default()
                )))
                .style(button::text)
                .width(Length::Fill)),
            (Button::new(text("Выход").size(14))
                .on_press(ApplicationMessage::OpenModal(Modal::Exit(
                    ExitModal::default()
                )))
                .style(button::text)
                .width(Length::Fill))
        )
        .width(150.0);

        let menu_bar = menu_bar!((text("Файл").size(14), file_menu),);

        let content = Column::new().padding(8).spacing(10).push(menu_bar);

        let layout: Element<'_, ApplicationMessage> = container(content)
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
            .on_press(ApplicationMessage::CloseModal)
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
