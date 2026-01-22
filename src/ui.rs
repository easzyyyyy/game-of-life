use eframe::egui;

use crate::camera::Camera;
use crate::game::GameOfLife;

// Main application state combining game logic and UI state
pub struct App {
    game: GameOfLife,
    camera: Camera,
    cell_size: f32,
}

impl App {
    pub fn new() -> Self {
        let rows = 50;
        let cols = 50;
        let game = GameOfLife::new(rows, cols);
        let camera = Camera::new();

        Self {
            game,
            camera,
            cell_size: 15.0,
        }
    }

    // Render the grid of cells
    fn render_grid(&self, ui: &egui::Ui) {
        let scaled_cell_size = self.camera.scaled_cell_size(self.cell_size);

        for row in 0..self.game.rows {
            for col in 0..self.game.cols {
                let is_alive = self.game.is_alive(row, col);

                let color = if is_alive {
                    egui::Color32::WHITE
                } else {
                    egui::Color32::BLACK
                };

                let pos = self.camera.grid_to_screen(row, col, self.cell_size);
                let rect = egui::Rect::from_min_size(
                    pos,
                    egui::vec2(scaled_cell_size, scaled_cell_size),
                );

                ui.painter().rect_filled(rect, 0.0, color);
            }
        }
    }

    // Handle mouse clicks on cells
    fn handle_cell_clicks(&mut self, ui: &egui::Ui) {
        if ui.input(|i| i.pointer.primary_clicked())
            && let Some(pos) = ui.input(|i| i.pointer.interact_pos())
        {
            let (grid_x, grid_y) = self.camera.screen_to_grid(pos, self.cell_size);
            let row = grid_y as usize;
            let col = grid_x as usize;

            self.game.toggle_cell(row, col);
        }
    }

    // Render the top control panel with buttons
    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Next Generation").clicked() {
                self.game.next_generation();
            }

            ui.separator();

            ui.label(format!("Zoom: {:.1}x", self.camera.zoom));
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel for controls
        egui::TopBottomPanel::top("controls").show(ctx, |ui| {
            self.render_controls(ui);
        });

        // Central panel for the game grid
        egui::CentralPanel::default().show(ctx, |ui| {
            // Handle panning
            self.camera.handle_pan(ui);

            // Handle cell clicks
            self.handle_cell_clicks(ui);

            // Render the grid
            self.render_grid(ui);
        });
    }
}
