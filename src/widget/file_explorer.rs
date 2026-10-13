use fatfs::{DirEntry, FileSystem};
use fscommon::BufStream;
use iced::widget::{Button, Column, Row, button, text};
use iced::{Alignment, Element, Length};
use std::fs::File;

pub struct FileExplorerWidget;

#[derive(Debug, Clone)]
pub enum FileExplorerMessage {}

impl FileExplorerWidget {
    pub fn view(
        &self,
        file_system: &FileSystem<BufStream<File>>,
    ) -> Element<'static, FileExplorerMessage> {
        let mut list = Column::new().spacing(2);

        let root_directory = file_system.root_dir();
        let root_directory_iterator = root_directory.iter();

        for entry in root_directory_iterator {
            match entry {
                Ok(value) => {
                    let file = file_entry(value);
                    list = list.push(file);
                }
                Err(error) => {}
            }
        }

        list.into()
    }
}

impl FileExplorerWidget {
    pub fn new() -> Self {
        Self {}
    }
}

fn file_entry<'a>(entry: DirEntry<BufStream<File>>) -> Element<'a, FileExplorerMessage> {
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
