use eframe::egui::{Button, Ui, ViewportCommand, Widget};
use eframe::{Frame, egui};

pub struct App {
    menu_bar: MenuBar,
}

impl Default for App {
    fn default() -> Self {
        Self { menu_bar: MenuBar }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        self.menu_bar.ui(ui, frame);
    }
}

struct MenuBar;

impl MenuBar {
    fn ui(&self, ui: &mut Ui, _frame: &mut Frame) {
        egui::MenuBar::new().ui(ui, |ui| {
            self.file_menu_button(ui);
        });
    }

    fn file_menu_button(&self, ui: &mut Ui) {
        ui.menu_button("File", |ui| {
            self.create_button(ui);
            self.open_button(ui);
            self.exit_button(ui);
        });
    }

    fn create_button(&self, ui: &mut Ui) {
        let button = Button::new("Create").ui(ui);

        if button.clicked() {
            println!("Clicked on button");
        }
    }

    fn open_button(&self, ui: &mut Ui) {
        let button = Button::new("Open").ui(ui);
        let file_dialog = rfd::FileDialog::new().add_filter("Disk image file", &["img"]);

        if button.clicked()
            && let Some(path) = file_dialog.pick_file()
        {}
    }

    fn exit_button(&self, ui: &mut Ui) {
        let button = Button::new("Exit").ui(ui);

        if button.clicked() {
            ui.ctx().send_viewport_cmd(ViewportCommand::Close);
        }
    }
}
