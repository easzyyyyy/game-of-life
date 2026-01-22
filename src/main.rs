// Import the egui crate from eframe for GUI rendering
use eframe::egui;

// Entry point of the application
fn main() {
    // Set up the window options (fullscreen)
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_fullscreen(true),
        ..Default::default()
    };

    // Launch the native eframe application with our GameOfLife app
    eframe::run_native(
        "Game of Life",
        options,
        Box::new(|_cc| Ok(Box::new(GameOfLife::new()))),
    )
    .unwrap();
}

// The main struct representing the Game of Life state
struct GameOfLife {
    grid: Vec<Vec<bool>>, // 2D grid of cells (true = alive, false = dead)
    rows: usize,          // Number of rows in the grid
    cols: usize,          // Number of columns in the grid
    cell_size: f32,       // Size of each cell in pixels
}

impl GameOfLife {
    // Constructor: initializes a new Game of Life grid
    fn new() -> Self {
        let rows = 50;
        let cols = 50;

        // Create an empty grid (all cells dead)
        let grid = vec![vec![false; cols]; rows];

        Self {
            grid,
            rows,
            cols,
            cell_size: 15.0,
        }
    }

    // Render a single cell on the grid
    fn render_cell(&self, ui: &egui::Ui, row: usize, col: usize) {
        let is_alive = self.grid[row][col];

        let color = if is_alive {
            egui::Color32::WHITE
        } else {
            egui::Color32::BLACK
        };

        let rect = egui::Rect::from_min_size(
            egui::pos2(col as f32 * self.cell_size, row as f32 * self.cell_size),
            egui::vec2(self.cell_size, self.cell_size),
        );

        ui.painter().rect_filled(rect, 0.0, color);
    }

    // Handle mouse interaction for a single cell
    fn handle_cell_interaction(&mut self, ui: &mut egui::Ui, row: usize, col: usize) {
        let rect = egui::Rect::from_min_size(
            egui::pos2(col as f32 * self.cell_size, row as f32 * self.cell_size),
            egui::vec2(self.cell_size, self.cell_size),
        );

        let response = ui.allocate_rect(rect, egui::Sense::click());

        if response.clicked() {
            self.grid[row][col] = !self.grid[row][col];
        }
    }
}

// Implement the eframe::App trait for our GameOfLife struct
impl eframe::App for GameOfLife {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for row in 0..self.rows {
                for col in 0..self.cols {
                    self.render_cell(ui, row, col);
                    self.handle_cell_interaction(ui, row, col);
                }
            }
        });
    }
}
