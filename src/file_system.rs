use fatfs::{FatType, FileSystem};
use fscommon::BufStream;
use std::fs;
use std::fs::File;
use std::io::Error;
use std::path::Path;

pub fn create_image(
    path: &Path,
    size: u64,
    volume_label: [u8; 11],
) -> Result<FileSystem<BufStream<File>>, Error> {
    let image_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)?;

    image_file.set_len(size)?;

    let mut buffered_stream = BufStream::new(image_file);

    let format_options = fatfs::FormatVolumeOptions::new()
        .fat_type(FatType::Fat16)
        .volume_label(volume_label);

    fatfs::format_volume(&mut buffered_stream, format_options)?;
    FileSystem::new(buffered_stream, fatfs::FsOptions::new())
}

pub fn open_image(path: &Path) -> Result<FileSystem<BufStream<File>>, Error> {
    let image_file = fs::OpenOptions::new().read(true).write(true).open(path)?;

    let buffered_stream = BufStream::new(image_file);

    FileSystem::new(buffered_stream, fatfs::FsOptions::new())
}
