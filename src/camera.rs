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

    // Handle mouse drag and trackpad scroll for panning
    pub fn handle_pan(&mut self, ui: &egui::Ui) {
        let response = ui.interact(
            ui.max_rect(),
            ui.id().with("camera_drag"),
            egui::Sense::drag(),
        );

        // Mouse drag panning
        if response.dragged() {
            self.offset += response.drag_delta();
        }

        // Two-finger trackpad scroll panning
        let scroll_delta = ui.input(|i| i.smooth_scroll_delta);
        self.offset += scroll_delta;
    }

    // Handle zoom with mouse wheel or trackpad pinch
    pub fn handle_zoom(&mut self, ui: &egui::Ui, screen_center: egui::Vec2) -> bool {
        let zoom_delta = ui.input(|i| i.zoom_delta());

        if zoom_delta != 1.0 {
            let old_zoom = self.zoom;
            self.zoom = (self.zoom * zoom_delta).clamp(0.1, 5.0);

            // Adjust offset to keep screen center fixed when zooming
            let center_world_x = (screen_center.x - self.offset.x) / old_zoom;
            let center_world_y = (screen_center.y - self.offset.y) / old_zoom;
            self.offset.x = screen_center.x - center_world_x * self.zoom;
            self.offset.y = screen_center.y - center_world_y * self.zoom;

            true
        } else {
            false
        }
    }

    // Transform a screen position to grid coordinates
    pub fn screen_to_grid(&self, screen_pos: egui::Pos2, cell_size: f32) -> (f32, f32) {
        let grid_x = (screen_pos.x - self.offset.x) / (cell_size * self.zoom);
        let grid_y = (screen_pos.y - self.offset.y) / (cell_size * self.zoom);
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
