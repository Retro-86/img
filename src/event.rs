use fatfs::FileSystem;
use fscommon::BufStream;
use std::{cell::RefCell, fs::File};

pub struct ImageCreatedEvent(pub RefCell<Option<FileSystem<BufStream<File>>>>);
