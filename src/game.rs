// The main struct representing the Game of Life state
pub struct GameOfLife {
    pub grid: Vec<Vec<bool>>, // 2D grid of cells (true = alive, false = dead)
    pub rows: usize,          // Number of rows in the grid
    pub cols: usize,          // Number of columns in the grid
}

impl GameOfLife {
    // Constructor: initializes a new Game of Life grid
    pub fn new(rows: usize, cols: usize) -> Self {
        // Create an empty grid (all cells dead)
        let grid = vec![vec![false; cols]; rows];

        Self { grid, rows, cols }
    }

    // Toggle a cell's state at the given position
    pub fn toggle_cell(&mut self, row: usize, col: usize) {
        if row < self.rows && col < self.cols {
            self.grid[row][col] = !self.grid[row][col];
        }
    }

    // Get the state of a cell
    pub fn is_alive(&self, row: usize, col: usize) -> bool {
        if row < self.rows && col < self.cols {
            self.grid[row][col]
        } else {
            false
        }
    }

    // Count alive neighbors for a given cell
    fn count_neighbors(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;

        // Check all 8 surrounding cells
        for dr in -1..=1 {
            for dc in -1..=1 {
                // Skip the cell itself
                if dr == 0 && dc == 0 {
                    continue;
                }

                // Calculate neighbor position
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;

                // Check bounds and if cell is alive
                if new_row >= 0
                    && new_row < self.rows as i32
                    && new_col >= 0
                    && new_col < self.cols as i32
                    && self.grid[new_row as usize][new_col as usize]
                {
                    count += 1;
                }
            }
        }

        count
    }

    // Calculate the next generation based on Conway's rules
    #[allow(clippy::needless_range_loop)]
    pub fn next_generation(&mut self) {
        let mut new_grid = self.grid.clone();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let neighbors = self.count_neighbors(row, col);
                let is_alive = self.grid[row][col];

                // Apply Conway's rules
                new_grid[row][col] = match (is_alive, neighbors) {
                    (true, 2) | (true, 3) => true, // Survival
                    (false, 3) => true,            // Birth
                    _ => false,                    // Death or stays dead
                };
            }
        }

        self.grid = new_grid;
    }
}
