#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod error;
pub mod image;
mod modal;

use crate::error::Error;
use crate::image::ImageFile;
use crate::modal::{Modal, ModalMessage, create, exit, file};
use fatfs::{DirEntry, FileSystem};
use fscommon::BufStream;
use iced::widget::{Button, Column, Row, button, container, stack, text};
use iced::{Alignment, Element, Event, Length, Size, Subscription, Task, Theme, event, window};
use iced_aw::{menu, menu_bar};
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

fn main() -> iced::Result {
    iced::application(Application::default, Application::update, Application::view)
        .subscription(Application::subscription)
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
    CreateFileInImage(String),
    EventOccurred(Event),
    AddFileToImage(PathBuf),
    SelectFileAddToImage,
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
                if let Some(task) = self.unmount_image() {
                    return task;
                }

                match image::open(&path) {
                    Ok(file_system) => self.file_system = Some(file_system),
                    Err(error) => {
                        return self
                            .show_error(Error::ImageFileOpenFailed(path, error.to_string()));
                    }
                }
            }
            ApplicationMessage::CreateImageFile(image_file) => {
                if let Some(task) = self.unmount_image() {
                    return task;
                }

                return match image::create(&image_file) {
                    Ok(file_system) => {
                        self.file_system = Some(file_system);
                        Task::done(ApplicationMessage::CloseModal)
                    }
                    Err(error) => self.show_error(Error::ImageFileCreateFailed(
                        image_file.path,
                        error.to_string(),
                    )),
                };
            }
            ApplicationMessage::OpenModal(modal) => self.active_modal = Some(modal),
            ApplicationMessage::Exit => return window::latest().and_then(window::close),
            ApplicationMessage::CloseModal => self.active_modal = None,
            ApplicationMessage::ModalAction(message) => {
                if let Some(modal) = &mut self.active_modal {
                    return modal.update(message);
                }
            }
            ApplicationMessage::CreateFileInImage(name) => {
                let result = if let Some(file_system) = self.file_system.as_mut() {
                    match file_system.root_dir().create_file(&name) {
                        Ok(_) => Ok(()),
                        Err(error) => Err(error),
                    }
                } else {
                    Ok(())
                };

                return match result {
                    Ok(_) => Task::done(ApplicationMessage::CloseModal),
                    Err(error) => self.show_error(Error::FileCreateFailed(name, error.to_string())),
                };
            }
            ApplicationMessage::EventOccurred(event) => match event {
                Event::Window(window_event) => match window_event {
                    window::Event::FileDropped(path) => {
                        if path.is_file() {
                            return Task::done(ApplicationMessage::AddFileToImage(path));
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            ApplicationMessage::AddFileToImage(file_path) => {
                let file_name = file_path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "unnamed_file".to_string());

                let mut source_file = match File::open(&file_path) {
                    Ok(f) => f,
                    Err(error) => {
                        return self.show_error(Error::FileDropFailed(
                            file_name,
                            error.to_string(),
                        ));
                    }
                };

                let result: Result<(), String> = (|| {
                    let file_system = self
                        .file_system
                        .as_mut()
                        .ok_or_else(|| "FileSystem not initialized".to_string())?;

                    let root_directory = file_system.root_dir();
                    let mut file = root_directory
                        .create_file(&file_name)
                        .map_err(|error| error.to_string())?;

                    file
                        .truncate()
                        .and_then(|_| {
                            io::copy(&mut source_file, &mut file).map(|_| ())
                        })
                        .and_then(|_| file.flush())
                        .map_err(|error| error.to_string())?;

                    Ok(())
                })();

                if let Err(error) = result {
                    return self.show_error(Error::FileDropFailed(
                        file_path.display().to_string(),
                        error.to_string(),
                    ));
                }
            }
            ApplicationMessage::SelectFileAddToImage => {
                let file_path = rfd::FileDialog::new().pick_file();

                if let Some(file_path) = file_path {
                    return Task::done(ApplicationMessage::AddFileToImage(file_path));
                }
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, ApplicationMessage> {
        let file_menu = menu!(
            (Button::new(text("Open").size(14))
                .on_press(ApplicationMessage::SelectImageFile)
                .padding([6, 12])
                .style(button::text)
                .width(Length::Fill)),
            (Button::new(text("Create").size(14))
                .on_press(ApplicationMessage::OpenModal(Modal::Create(
                    create::Modal::default()
                )))
                .padding([6, 12])
                .style(button::text)
                .width(Length::Fill)),
            (Button::new(text("Exit").size(14))
                .on_press(ApplicationMessage::OpenModal(Modal::Exit(
                    exit::Modal::default()
                )))
                .padding([6, 12])
                .style(button::text)
                .width(Length::Fill))
        )
        .width(100);

        let image_menu = menu!(
            (Button::new(text("Create file").size(14))
                .on_press_maybe(match self.file_system {
                    None => None,
                    Some(_) => Some(ApplicationMessage::OpenModal(Modal::FileCreate(
                        file::create::Modal::default()
                    ))),
                })
                .padding([6, 4])
                .style(button::text)
                .width(Length::Fill)),
            (Button::new(text("Add file").size(14))
                .on_press_maybe(match self.file_system {
                    None => None,
                    Some(_) => Some(ApplicationMessage::SelectFileAddToImage),
                })
                .padding([6, 4])
                .style(button::text)
                .width(Length::Fill))
        )
        .width(75);

        let menu_bar = menu_bar!(
            (text("File").size(14), file_menu),
            (text("Image").size(14), image_menu)
        )
        .width(Length::Fill)
        .spacing(20);

        let body = container(match self.file_system {
            Some(_) => self.view_file_list(),
            None => text("Please open or create image")
                .align_y(Alignment::Center)
                .align_x(Alignment::Center)
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
        });

        let content = Column::new()
            .padding(8)
            .spacing(10)
            .push(menu_bar)
            .push(body);

        let layout: Element<'_, ApplicationMessage> = container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        if let Some(modal) = &self.active_modal {
            let modal_content = modal::open(modal);
            stack![layout, modal_content].into()
        } else {
            layout
        }
    }

    fn subscription(&self) -> Subscription<ApplicationMessage> {
        event::listen().map(ApplicationMessage::EventOccurred)
    }

    fn show_error(&mut self, error: Error) -> Task<ApplicationMessage> {
        Task::done(ApplicationMessage::OpenModal(Modal::Error(
            modal::error::Modal::new(error),
        )))
    }

    fn theme(&self) -> Theme {
        if self.theme.is_none() {
            Theme::Dark
        } else {
            self.theme.clone().unwrap()
        }
    }

    fn view_file_list(&self) -> Element<'_, ApplicationMessage> {
        let mut list = Column::new().spacing(2);

        if let Some(file_system) = &self.file_system {
            let root_directory_iterator = file_system.root_dir().iter();

            for entry in root_directory_iterator {
                match entry {
                    Ok(value) => {
                        let file = file_entry(value);
                        list = list.push(file);
                    }
                    Err(error) => {}
                }
            }
        }

        list.into()
    }

    fn unmount_image(&mut self) -> Option<Task<ApplicationMessage>> {
        if let Some(file_system) = self.file_system.take() {
            if let Err(error) = file_system.unmount() {
                return Some(self.show_error(Error::FileSystemUnmountFailed(error.to_string())));
            }
        }

        None
    }
}

fn file_entry<'a>(entry: DirEntry<BufStream<File>>) -> Element<'a, ApplicationMessage> {
    let icon = if entry.is_dir() { "📁 " } else { "📄 " };

    Button::new(
        Row::new()
            .spacing(10)
            .align_y(Alignment::Center)
            .push(text(icon).size(16))
            .push(text(entry.file_name()).size(14)),
    )
    .style(button::text)
    .width(Length::Fill)
    .padding(6)
    .into()
}
