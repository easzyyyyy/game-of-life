use eframe::egui;

use crate::camera::Camera;
use crate::game::GameOfLife;

// Main application state combining game logic and UI state
pub struct App {
    game: GameOfLife,
    camera: Camera,
    cell_size: f32,
    last_camera_offset: egui::Vec2,
    last_camera_zoom: f32,
    is_playing: bool,
    speed: f32,
    generation_count: usize,
    time_accumulator: f32,
    grid_history: Vec<Vec<Vec<bool>>>,
}

impl App {
    pub fn new() -> Self {
        let rows = 1000;
        let cols = 1000;
        let game = GameOfLife::new(rows, cols);
        let camera = Camera::new();

        Self {
            game,
            camera,
            cell_size: 15.0,
            last_camera_offset: egui::Vec2::ZERO,
            last_camera_zoom: 1.0,
            is_playing: false,
            speed: 5.0,
            generation_count: 0,
            time_accumulator: 0.0,
            grid_history: Vec::new(),
        }
    }

    // Render the grid of cells
    fn render_grid(&self, ui: &egui::Ui) {
        let scaled_cell_size = self.camera.scaled_cell_size(self.cell_size);
        let visible_rect = ui.clip_rect();

        // Draw black background for the entire visible grid area (optimization: single draw call)
        let grid_width = self.game.cols as f32 * scaled_cell_size;
        let grid_height = self.game.rows as f32 * scaled_cell_size;
        let grid_rect = egui::Rect::from_min_size(
            self.camera.grid_to_screen(0, 0, self.cell_size),
            egui::vec2(grid_width, grid_height),
        );
        ui.painter().rect_filled(grid_rect, 0.0, egui::Color32::BLACK);

        // Calculate which cells are visible on screen (culling optimization)
        let start_col = ((visible_rect.min.x - self.camera.offset.x) / scaled_cell_size)
            .floor()
            .max(0.0) as usize;
        let end_col = ((visible_rect.max.x - self.camera.offset.x) / scaled_cell_size)
            .ceil()
            .min(self.game.cols as f32) as usize;
        let start_row = ((visible_rect.min.y - self.camera.offset.y) / scaled_cell_size)
            .floor()
            .max(0.0) as usize;
        let end_row = ((visible_rect.max.y - self.camera.offset.y) / scaled_cell_size)
            .ceil()
            .min(self.game.rows as f32) as usize;

        // Only render alive cells (optimization: skip dead cells)
        for row in start_row..end_row {
            for col in start_col..end_col {
                if self.game.is_alive(row, col) {
                    let pos = self.camera.grid_to_screen(row, col, self.cell_size);
                    let rect = egui::Rect::from_min_size(
                        pos,
                        egui::vec2(scaled_cell_size, scaled_cell_size),
                    );

                    ui.painter().rect_filled(rect, 0.0, egui::Color32::WHITE);
                }
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

    // Handle mouse clicks on cells (only when paused)
    fn handle_cell_clicks(&mut self, ui: &egui::Ui) {
        if !self.is_playing
            && let Some(pos) = ui.ctx().pointer_interact_pos()
            && ui.ui_contains_pointer()
            && ui.input(|i| i.pointer.primary_clicked())
        {
            // Use the position as-is (panel coordinates)
            let (grid_x, grid_y) = self.camera.screen_to_grid(pos, self.cell_size);
            let row = grid_y as usize;
            let col = grid_x as usize;

            self.game.toggle_cell(row, col);
        }
    }

    // Render the top control panel with buttons
    fn render_controls(&mut self, ui: &mut egui::Ui, screen_center: egui::Vec2) {
        ui.horizontal(|ui| {
            // Play/Pause button (icon only)
            let play_pause_icon = if self.is_playing { "⏸" } else { "▶" };
            if ui.button(play_pause_icon).clicked() {
                self.is_playing = !self.is_playing;
            }

            ui.separator();

            // Previous generation button (only when paused)
            ui.add_enabled_ui(!self.is_playing && self.generation_count > 0, |ui| {
                if ui.button("◀").clicked()
                    && let Some(prev_grid) = self.grid_history.pop()
                {
                    self.game.grid = prev_grid;
                    self.generation_count -= 1;
                }
            });

            // Next generation button (only when paused)
            ui.add_enabled_ui(!self.is_playing, |ui| {
                if ui.button("▶").clicked() {
                    // Save current state to history (limit to 100 states)
                    self.grid_history.push(self.game.grid.clone());
                    if self.grid_history.len() > 100 {
                        self.grid_history.remove(0);
                    }

                    self.game.next_generation();
                    self.generation_count += 1;
                }
            });

            ui.separator();

            // Speed slider
            ui.label("Speed:");
            ui.add(egui::Slider::new(&mut self.speed, 1.0..=30.0).suffix(" gen/s"));

            ui.separator();

            // Zoom slider with center anchoring
            ui.label("Zoom:");
            let old_zoom = self.camera.zoom;
            ui.add(egui::Slider::new(&mut self.camera.zoom, 0.1..=5.0).logarithmic(true));

            // Adjust offset to keep screen center fixed when zooming
            if old_zoom != self.camera.zoom {
                // Calculate the world position of the screen center before zoom
                let center_world_x = (screen_center.x - self.camera.offset.x) / old_zoom;
                let center_world_y = (screen_center.y - self.camera.offset.y) / old_zoom;
                // Recalculate offset to keep that world position at screen center
                self.camera.offset.x = screen_center.x - center_world_x * self.camera.zoom;
                self.camera.offset.y = screen_center.y - center_world_y * self.camera.zoom;
            }
        });
    }

    // Render stats (FPS, etc.) in the top-right corner
    fn render_stats(&self, ctx: &egui::Context) {
        egui::Area::new(egui::Id::new("fps_display"))
            .anchor(egui::Align2::RIGHT_TOP, [-10.0, 10.0])
            .show(ctx, |ui| {
                ui.allocate_ui(egui::vec2(180.0, 50.0), |ui| {
                    let fps = 1.0 / ctx.input(|i| i.stable_dt.max(0.001));
                    ui.label(format!("{:.0} FPS", fps));
                    ui.label(format!("Gen: {}", self.generation_count));
                });
            });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle automatic generation advancement when playing
        if self.is_playing {
            let dt = ctx.input(|i| i.stable_dt);
            self.time_accumulator += dt;

            let time_per_generation = 1.0 / self.speed;
            // Limit to max 5 generations per frame to prevent freeze/crash
            let mut generations_this_frame = 0;
            const MAX_GENERATIONS_PER_FRAME: usize = 1;

            while self.time_accumulator >= time_per_generation && generations_this_frame < MAX_GENERATIONS_PER_FRAME {
                // Save current state to history (limit to 100 states)
                self.grid_history.push(self.game.grid.clone());
                if self.grid_history.len() > 100 {
                    self.grid_history.remove(0);
                }

                self.game.next_generation();
                self.generation_count += 1;
                self.time_accumulator -= time_per_generation;
                generations_this_frame += 1;
            }

            // Reset accumulator if we hit the limit to prevent endless catch-up
            if generations_this_frame >= MAX_GENERATIONS_PER_FRAME {
                self.time_accumulator = 0.0;
            }

            ctx.request_repaint();
        }

        // Lazy rendering: only repaint when something changes
        let camera_changed = self.camera.offset != self.last_camera_offset
            || self.camera.zoom != self.last_camera_zoom;

        if camera_changed {
            self.last_camera_offset = self.camera.offset;
            self.last_camera_zoom = self.camera.zoom;
            ctx.request_repaint();
        }

        // Render stats (FPS)
        self.render_stats(ctx);

        // Top panel for controls
        let screen_center = egui::vec2(ctx.viewport_rect().width() / 2.0, ctx.viewport_rect().height() / 2.0);
        egui::TopBottomPanel::top("controls").show(ctx, |ui| {
            self.render_controls(ui, screen_center);
        });

        // Central panel for the game grid
        egui::CentralPanel::default().show(ctx, |ui| {
            self.center_grid_if_needed(ui);

            // Handle panning
            self.camera.handle_pan(ui);

            // Handle zoom with mouse wheel or trackpad pinch
            let panel_center = ui.available_rect_before_wrap().center().to_vec2();
            self.camera.handle_zoom(ui, panel_center);

            // Handle cell clicks
            self.handle_cell_clicks(ui);

            // Render the grid
            self.render_grid(ui);
        });
    }
}
