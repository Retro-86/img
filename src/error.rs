use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    ImageFileCreateFailed(PathBuf, String),
    ImageFileOpenFailed(PathBuf, String),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ImageFileCreateFailed(path, error) => {
                write!(formatter, "Failed to create image file: {} {}", path.display(), error)
            }
            Error::ImageFileOpenFailed(path, error) => {
                write!(formatter, "Failed to open image file: {} {}", path.display(), error)
            }
        }
    }
}