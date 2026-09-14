#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod gui;

use crate::gui::App;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "img",
        options,
        Box::new(|_context| {
            Ok(Box::<App>::default())
        }),
    )
}


