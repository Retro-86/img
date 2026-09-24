use fatfs::{FatType, FileSystem};
use fscommon::BufStream;
use std::fs;
use std::fs::File;
use std::io::Error;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct ImageFile {
    pub path: PathBuf,
    pub volume_label: [u8; 11],
    pub size: u64,
}

impl ImageFile {
    pub fn new(path: PathBuf, volume_label: [u8; 11], size: u64) -> Self {
        Self {
            path,
            volume_label,
            size,
        }
    }
}

pub fn open(image_file_path: &Path) -> Result<FileSystem<BufStream<File>>, Error> {
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(image_file_path)?;

    let buffered_stream = BufStream::new(file);

    FileSystem::new(buffered_stream, fatfs::FsOptions::new())
}

pub fn create(image_file: &ImageFile) -> Result<FileSystem<BufStream<File>>, Error> {
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(image_file.path.clone())?;

    file.set_len(image_file.size)?;

    let mut buffered_stream = BufStream::new(file);

    let format_options = fatfs::FormatVolumeOptions::new()
        .fat_type(FatType::Fat16)
        .volume_label(image_file.volume_label);

    fatfs::format_volume(&mut buffered_stream, format_options)?;

    FileSystem::new(buffered_stream, fatfs::FsOptions::new())
}
