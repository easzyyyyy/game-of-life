// Module declarations
mod camera;
mod game;
mod ui;

use eframe::egui;
use ui::App;

// Entry point of the application
fn main() {
    // Set up the window options (fullscreen)
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_fullscreen(true),
        ..Default::default()
    };

    // Launch the native eframe application with our Game of Life app
    eframe::run_native(
        "Game of Life",
        options,
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
    .unwrap();
}
