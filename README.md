# img

A lightweight utility with a graphical interface for creating and managing disk images with the FAT16 file system. Written in Rust

## Features

- Create FAT16 images in the `.img` format
- Open existing images and view their contents
- Add files from your computer to an image
- Delete files from an image
- Create empty files inside an image
- Rename files inside an image

## Future Plans

- Move files between directories within an image

## Build

1. You’ll need [Rust and Cargo](https://rustup.rs/) to build this

2. Clone this repository:
```sh
git clone https://github.com/Retro-86/img.git
```

3. And in the project directory, run:
```sh
cargo run --release
```

## Usage

1. Create an image via **File -> Create image**, specifying a name, size, and folder, or open an existing one via **File -> Open image** 
2. Add a file from your computer via **Image -> Transfer file** or create an empty file via **Image -> Create file**
3. To rename or delete a file, right-click on it and select **Rename** or **Delete**
4. When you’re finished, close the image via **File -> Close image**

## License

[GPL-v3](LICENSE)
