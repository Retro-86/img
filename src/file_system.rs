use fatfs::{Dir, FatType, FileSystem, ReadWriteSeek};
use fscommon::BufStream;
use gpui_kit::base::TreeItem;
use std::fs::File;
use std::path::Path;
use std::{fs, io};

pub fn read_directory_and_build_file_tree<T: ReadWriteSeek>(
    directory: &Dir<T>,
    current_path: &str,
) -> io::Result<Vec<TreeItem>> {
    let mut items = Vec::new();

    for entry in directory.iter() {
        let entry = entry.map_err(|error| {
            let path = if current_path.is_empty() {
                "/"
            } else {
                current_path
            };

            io::Error::new(
                error.kind(),
                format!("Unable to read the catalog {path}\n {error}"),
            )
        })?;

        let name = entry.file_name();

        if name == "." || name == ".." {
            continue;
        }

        let full_path = if current_path.is_empty() {
            name.clone()
        } else {
            format!("{}/{}", current_path, name)
        };

        let mut tree_item = TreeItem::new(full_path.clone(), name);

        if entry.is_dir() {
            let children = read_directory_and_build_file_tree(&entry.to_dir(), &full_path)?;
            tree_item = tree_item.children(children);
        }

        items.push(tree_item);
    }

    Ok(items)
}

pub fn create_image(
    path: &Path,
    size: u64,
    volume_label: [u8; 11],
) -> Result<FileSystem<BufStream<File>>, io::Error> {
    let image_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(path)?;

    image_file.set_len(size)?;

    let mut buffered_stream = BufStream::new(image_file);

    let format_options = fatfs::FormatVolumeOptions::new()
        .fat_type(FatType::Fat16)
        .volume_label(volume_label);

    fatfs::format_volume(&mut buffered_stream, format_options)?;
    FileSystem::new(buffered_stream, fatfs::FsOptions::new())
}

pub fn open_image(path: &Path) -> Result<FileSystem<BufStream<File>>, io::Error> {
    let image_file = fs::OpenOptions::new().read(true).write(true).open(path)?;

    let buffered_stream = BufStream::new(image_file);

    FileSystem::new(buffered_stream, fatfs::FsOptions::new())
}

pub fn copy_file_to_image(
    source_file_path: &Path,
    file_system: &mut FileSystem<BufStream<File>>,
    destination_file_path: &str,
) -> Result<(), io::Error> {
    let mut source_file = File::open(source_file_path)?;

    let root_directory = file_system.root_dir();
    let mut destination_file = root_directory
        .create_file(destination_file_path)
        .map_err(io::Error::other)?;

    destination_file.truncate()?;

    io::copy(&mut source_file, &mut destination_file)?;

    drop(source_file);
    drop(destination_file);

    Ok(())
}
