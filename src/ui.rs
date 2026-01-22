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
        let rows = 150;
        let cols = 150;
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

    // Center the grid in the available space if the camera offset is untouched
    fn center_grid_if_needed(&mut self, ui: &egui::Ui) {
        let grid_width = self.game.cols as f32 * self.cell_size * self.camera.zoom;
        let grid_height = self.game.rows as f32 * self.cell_size * self.camera.zoom;
        let avail = ui.available_size();
        if self.camera.offset == egui::Vec2::ZERO {
            self.camera.offset = egui::vec2(
                (avail.x - grid_width) / 2.0,
                (avail.y - grid_height) / 2.0,
            );
        }
    }

    // Handle mouse clicks on cells
    fn handle_cell_clicks(&mut self, ui: &egui::Ui) {
        if let Some(pos) = ui.ctx().pointer_interact_pos() {
            if ui.ui_contains_pointer() && ui.input(|i| i.pointer.primary_clicked()) {
                // Use the position as-is (panel coordinates)
                let (grid_x, grid_y) = self.camera.screen_to_grid(pos, self.cell_size);
                let row = grid_y as usize;
                let col = grid_x as usize;

                self.game.toggle_cell(row, col);
            }
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
            self.center_grid_if_needed(ui);

            // Handle panning
            self.camera.handle_pan(ui);

            // Handle cell clicks
            self.handle_cell_clicks(ui);

            // Render the grid
            self.render_grid(ui);
        });
    }
}
