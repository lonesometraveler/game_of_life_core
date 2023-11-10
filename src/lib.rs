#![no_std]

/// The state of a Cell
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

/// The Universe
pub struct Universe<const W: usize, const H: usize> {
    grid: [[Cell; W]; H],
    height: usize,
    width: usize,
}

impl<const W: usize, const H: usize> Universe<W, H> {
    pub fn new() -> Self {
        Universe {
            width: W,
            height: H,
            grid: [[Cell::Dead; W]; H],
        }
    }

    pub fn grid(&self) -> [[Cell; W]; H] {
        self.grid
    }

    pub fn set_cell(&mut self, row: usize, column: usize, cell: Cell) {
        self.grid[row][column] = cell;
    }

    pub fn evolve(&mut self) {
        let mut next = [[Cell::Dead; W]; H];

        for (row, next_row) in next.iter_mut().enumerate().take(self.height) {
            for (column, next_cell) in next_row.iter_mut().enumerate().take(self.width) {
                let cell = self.grid[row][column];
                let live_neighbors = self.live_neighbor_count(row, column);

                *next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
            }
        }

        self.grid = next;
    }

    fn live_neighbor_count(&self, row: usize, column: usize) -> u8 {
        let mut count = 0;
        for &delta_row in [self.height - 1, 0, 1].iter() {
            for &delta_col in [self.width - 1, 0, 1].iter() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                // Calculate the neighbor's coordinates with wrapping
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;

                count += self.grid[neighbor_row][neighbor_col] as u8;
            }
        }
        count
    }
}

impl<const W: usize, const H: usize> Default for Universe<W, H> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_universe() {
        let universe = Universe::<3, 3>::new();
        assert_eq!(universe.width, 3);
        assert_eq!(universe.height, 3);
        assert_eq!(universe.grid, [[Cell::Dead; 3]; 3]);
    }

    #[test]
    fn test_live_neighbor_count_no_live_neighbors() {
        let mut universe = Universe::<3, 3>::new();

        // Set the center cell to Alive
        universe.grid[1][1] = Cell::Alive;

        // No live neighbors
        let count = universe.live_neighbor_count(1, 1);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_live_neighbor_count_some_live_neighbors() {
        let mut universe = Universe::<3, 3>::new();

        // Set some neighboring cells to Alive
        universe.grid[0][0] = Cell::Alive;
        universe.grid[0][1] = Cell::Alive;
        universe.grid[1][0] = Cell::Alive;

        // Center cell has 3 live neighbors
        let count = universe.live_neighbor_count(1, 1);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_live_neighbor_count_wrap_around() {
        let mut universe = Universe::<3, 3>::new();

        // Set cells near the edges to Alive
        universe.grid[0][0] = Cell::Alive;
        universe.grid[0][2] = Cell::Alive;
        universe.grid[2][0] = Cell::Alive;
        universe.grid[2][2] = Cell::Alive;

        // Center cell has 4 live neighbors, including wrapping around the edges
        let count = universe.live_neighbor_count(1, 1);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_get_matrix() {
        let universe = Universe::<3, 3>::new();
        let matrix = universe.grid();

        assert_eq!(matrix.len(), 3);
        assert_eq!(matrix[0].len(), 3);
    }

    #[test]
    fn test_evolution_with_mutated_logic_1() {
        let mut universe = Universe::<3, 3>::new();
        // Set up a scenario where specific evolution behavior is expected
        universe.grid[0][0] = Cell::Alive;
        universe.evolve();
        assert_eq!(universe.grid, [[Cell::Dead; 3]; 3]);
    }

    #[test]
    fn test_evolution_with_mutated_logic_2() {
        let mut universe = Universe::<4, 4>::new();
        // Set up a scenario where specific evolution behavior is expected
        universe.grid[0][0] = Cell::Alive;
        universe.grid[0][1] = Cell::Alive;
        universe.grid[1][0] = Cell::Alive;
        universe.evolve();
        assert_eq!(
            universe.grid,
            [
                [Cell::Alive, Cell::Alive, Cell::Dead, Cell::Dead],
                [Cell::Alive, Cell::Alive, Cell::Dead, Cell::Dead],
                [Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead],
                [Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead]
            ]
        );
    }
}
