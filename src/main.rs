#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::widget::{Button, Column, Row, Slider, button, container, text, text_input};
use iced::{Alignment, Element, Length, Size, Task, Theme};
use iced_aw::{menu, menu_bar};
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;

fn main() -> iced::Result {
    iced::application(Application::default, Application::update, Application::view)
        .theme(Application::theme)
        .title("img")
        .window_size(Size::new(1000.0, 600.0))
        .run()
}

#[derive(Debug)]
struct ImageModalState {
    name: String,
    folder: Option<PathBuf>,
    size: u32,
}

#[derive(Debug, Clone)]
enum ImageModalMessage {
    Close,
    NameChanged(String),
    PickFolder,
    FolderSelected(Option<PathBuf>),
    SliderChanged(u32),
    Submit,
}

#[derive(Default, Debug)]
struct Application {
    theme: Option<Theme>,
    file: Option<ImageFile>,
    create_modal: Option<ImageModalState>,
}

#[derive(Debug, Clone)]
enum Message {
    OpenImageFile,
    Exit,
    ShowCreateModal,
    CreateImageModal(ImageModalMessage),
}

#[derive(Debug)]
enum Error {
    ImageFileCreateFailed(PathBuf),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ImageFileCreateFailed(path) => {
                write!(formatter, "Failed to create image file: {}", path.display())
            }
        }
    }
}

impl Application {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenImageFile => {
                self.file = open_image_file();

                Task::none()
            }
            Message::ShowCreateModal => {
                self.create_modal = Some(ImageModalState {
                    name: String::from("image.img"),
                    folder: None,
                    size: 512,
                });

                Task::none()
            }
            Message::Exit => {
                std::process::exit(0);
            }
            Message::CreateImageModal(modal_message) => match modal_message {
                ImageModalMessage::Close => {
                    self.create_modal = None;
                    Task::none()
                }
                ImageModalMessage::NameChanged(name) => {
                    if let Some(ref mut modal) = self.create_modal {
                        modal.name = name;
                    }
                    Task::none()
                }
                ImageModalMessage::PickFolder => Task::perform(
                    async move { rfd::FileDialog::new().pick_folder() },
                    |folder| {
                        Message::CreateImageModal(ImageModalMessage::FolderSelected(folder))
                    },
                ),
                ImageModalMessage::FolderSelected(folder) => {
                    if let Some(ref mut modal) = self.create_modal {
                        modal.folder = folder;
                    }
                    Task::none()
                }
                ImageModalMessage::SliderChanged(size) => {
                    if let Some(ref mut modal) = self.create_modal {
                        modal.size = size;
                    }
                    Task::none()
                }
                ImageModalMessage::Submit => {
                    if let Some(ref modal) = self.create_modal {
                        if let Some(ref folder) = modal.folder {
                            let mut full_path = folder.clone();
                            full_path.push(&modal.name);

                            let size_in_bytes = (modal.size as u64) * 1024 * 1024;

                            let new_file = ImageFile {
                                path: full_path,
                                data: vec![0; size_in_bytes as usize],
                                size: size_in_bytes as u32,
                            };

                            if let Err(e) = save_image_file(new_file.clone()) {
                                eprintln!("Ошибка сохранения: {}", e);
                            } else {
                                self.file = Some(new_file);
                            }
                        }
                    }
                    self.create_modal = None;
                    Task::none()
                }
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let file_menu = menu!(
            (Button::new(text("Открыть образ").size(14))
                .on_press(Message::OpenImageFile)
                .style(button::text)
                .width(Length::Fill)),
            (Button::new(text("Создать образ").size(14))
                .on_press(Message::ShowCreateModal)
                .style(button::text)
                .width(Length::Fill)),
            (Button::new(text("Выход").size(14))
                .on_press(Message::Exit)
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

        if let Some(modal_state) = &self.create_modal {
            let folder_text = match &modal_state.folder {
                Some(p) => p.display().to_string(),
                None => String::from("Папка не выбрана"),
            };

            let mut modal_content =
                Column::new()
                    .spacing(15)
                    .padding(20)
                    .align_x(Alignment::Start)
                    .push(text("Создание FAT16 образа").size(20))
                    .push(text("Название файла:"))
                    .push(
                        text_input("Введите имя...", &modal_state.name).on_input(|s| {
                            Message::CreateImageModal(ImageModalMessage::NameChanged(s))
                        }),
                    )
                    .push(text("Путь сохранения:"))
                    .push(
                        Row::new()
                            .spacing(10)
                            .align_y(Alignment::Center)
                            .push(Button::new(text("Обзор...")).on_press(
                                Message::CreateImageModal(ImageModalMessage::PickFolder),
                            ))
                            .push(text(folder_text).size(14)),
                    )
                    .push(text(format!("Размер образа: {} МБ", modal_state.size)))
                    .push(
                        Slider::new(4..=2048, modal_state.size, |v| {
                            Message::CreateImageModal(ImageModalMessage::SliderChanged(v))
                        })
                        .step(4u32),
                    );

            let is_submit_enabled = modal_state.folder.is_some() && !modal_state.name.is_empty();
            let mut actions = Row::new().spacing(20).push(
                Button::new(text("Отмена"))
                    .on_press(Message::CreateImageModal(ImageModalMessage::Close)),
            );

            if is_submit_enabled {
                actions = actions.push(
                    Button::new(text("Создать"))
                        .on_press(Message::CreateImageModal(ImageModalMessage::Submit)),
                );
            } else {
                actions = actions.push(Button::new(text("Создать")));
            }

            modal_content = modal_content.push(actions);

            let modal_box = container(modal_content)
                .width(450)
                .padding(10)
                .style(container::bordered_box);

            let overlay = container(modal_box)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .style(|_theme| container::Style {
                    background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.7).into()),
                    ..Default::default()
                });

            iced::widget::stack![layout, overlay].into()
        } else {
            layout
        }
    }

    fn theme(&self) -> Theme {
        if self.theme.is_none() {
            Theme::Dark
        } else {
            self.theme.clone().unwrap()
        }
    }
}

fn open_image_file() -> Option<ImageFile> {
    let path = rfd::FileDialog::new()
        .add_filter("Disk image file", &["img"])
        .pick_file();

    None
}

fn save_image_file(image_file: ImageFile) -> Result<PathBuf, Error> {
    let file_path = image_file.path;
    fs::File::create(file_path.as_path())
        .map_err(|_| Error::ImageFileCreateFailed(file_path.to_path_buf()))?;

    Ok(file_path)
}

#[derive(Debug, Clone)]
struct ImageFile {
    path: PathBuf,
    data: Vec<u8>,
    size: u32,
}
