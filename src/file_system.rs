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

pub fn rename_entry<T: ReadWriteSeek>(
    file_system: &FileSystem<T>,
    path: &str,
    new_name: &str,
) -> io::Result<()> {
    validate_entry_name(new_name)?;

    let (parent, current_name) = path.rsplit_once('/').unwrap_or(("", path));

    if current_name.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "The source path does not identify an entry.",
        ));
    }

    if new_name == current_name {
        return Ok(());
    }

    if new_name.to_uppercase() == current_name.to_uppercase() {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Changing only letter case is not supported.",
        ));
    }

    let destination = if parent.is_empty() {
        new_name.to_owned()
    } else {
        format!("{parent}/{new_name}")
    };

    let root = file_system.root_dir();
    root.rename(path, &root, &destination)
}

fn validate_entry_name(name: &str) -> io::Result<()> {
    if name.trim().is_empty() || name == "." || name == ".." {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "File name is empty.",
        ));
    }

    if name.len() > 255 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "File name exceeds 255 symbols.",
        ));
    }

    let valid = name.chars().all(|character| {
        matches!(
            character,
            'a'..='z'
                | 'A'..='Z'
                | '0'..='9'
                | '\u{80}'..='\u{FFFF}'
                | '$' | '%' | '\'' | '-' | '_' | '@'
                | '~' | '`' | '!' | '(' | ')' | '{' | '}'
                | '.' | ' ' | '+' | ',' | ';' | '='
                | '[' | ']' | '^' | '#' | '&'
        )
    });

    if !valid {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "File name contains unsupported characters.",
        ));
    }

    Ok(())
}

pub fn delete_entry<T: ReadWriteSeek>(file_system: &FileSystem<T>, path: &str) -> io::Result<()> {
    if path
        .split('/')
        .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid entry path.",
        ));
    }

    let root = file_system.root_dir();
    let (parent_path, name) = path.rsplit_once('/').unwrap_or(("", path));

    let parent = if parent_path.is_empty() {
        root
    } else {
        root.open_dir(parent_path)?
    };

    for entry in parent.iter() {
        let entry = entry?;

        if entry.file_name() != name {
            continue;
        }

        if entry.is_dir() {
            let directory = entry.to_dir();
            delete_directory_contents(&directory)?;
        }

        return parent.remove(name);
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("Entry not found: {path}"),
    ))
}

fn delete_directory_contents<T: ReadWriteSeek>(directory: &Dir<T>) -> io::Result<()> {
    let mut children = Vec::new();

    for entry in directory.iter() {
        let entry = entry?;
        let name = entry.file_name();

        if name == "." || name == ".." {
            continue;
        }

        children.push((name, entry.is_dir()));
    }

    for (name, is_directory) in children {
        if is_directory {
            let child = directory.open_dir(&name)?;
            delete_directory_contents(&child)?;
        }

        directory.remove(&name)?;
    }

    Ok(())
}
