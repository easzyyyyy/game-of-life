use eframe::egui;

// Camera controls for panning and zooming the grid view
pub struct Camera {
    pub offset: egui::Vec2, // Current pan offset
    pub zoom: f32,          // Current zoom level
}

impl Camera {
    pub fn new() -> Self {
        Self {
            offset: egui::Vec2::ZERO,
            zoom: 1.0,
        }
    }

    // Handle mouse drag for panning
    pub fn handle_pan(&mut self, ui: &egui::Ui) {
        let response = ui.interact(
            ui.max_rect(),
            ui.id().with("camera_drag"),
            egui::Sense::drag(),
        );

        if response.dragged() {
            self.offset += response.drag_delta();
        }
    }

    // Transform a screen position to grid coordinates
    pub fn screen_to_grid(&self, screen_pos: egui::Pos2, cell_size: f32) -> (f32, f32) {
        let adjusted_pos = screen_pos - self.offset;
        let grid_x = adjusted_pos.x / (cell_size * self.zoom);
        let grid_y = adjusted_pos.y / (cell_size * self.zoom);
        (grid_x, grid_y)
    }

    // Transform grid coordinates to screen position
    pub fn grid_to_screen(&self, row: usize, col: usize, cell_size: f32) -> egui::Pos2 {
        let x = col as f32 * cell_size * self.zoom + self.offset.x;
        let y = row as f32 * cell_size * self.zoom + self.offset.y;
        egui::pos2(x, y)
    }

    // Get the scaled cell size with zoom applied
    pub fn scaled_cell_size(&self, base_size: f32) -> f32 {
        base_size * self.zoom
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
