use fatfs::FileSystem;
use fscommon::BufStream;
use std::fs::File;

pub struct ImageCreatedEvent(pub FileSystem<BufStream<File>>);
