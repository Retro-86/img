use fatfs::FileSystem;
use fscommon::BufStream;
use std::{cell::RefCell, fs::File};

pub struct ImageOpenEvent(pub RefCell<Option<FileSystem<BufStream<File>>>>);
