#![no_std]

use modular_bitfield::prelude::*;

/// The state of a Cell
#[derive(Clone, Copy, Debug, PartialEq, Eq, BitfieldSpecifier)]
pub enum State {
    Dead,
    Alive,
}

/// Cell
#[bitfield(bits = 8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    state: State,
    live_neighbors: B7,
}

impl Cell {
    /// Sets a new state for the cell based on the current state and live neighbors
    fn evolve(&mut self) {
        let state = match (self.state(), self.live_neighbors()) {
            (State::Dead, 3) => State::Alive,
            (State::Alive, 2) | (State::Alive, 3) => State::Alive,
            _ => State::Dead,
        };
        self.set_state(state);
    }

    /// Returns true if the cell is alive
    pub fn is_alive(&self) -> bool {
        self.state() == State::Alive
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}

/// The Universe with a fixed width and height
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
            grid: [[Cell::default(); W]; H],
        }
    }

    /// Returns the reference to the grid
    pub fn grid(&self) -> [[Cell; W]; H] {
        self.grid
    }

    /// Sets the state of the cell
    pub fn set_cell(&mut self, row: usize, column: usize, state: State) {
        self.grid[row][column].set_state(state);
    }

    /// Evolves the universe
    pub fn evolve(&mut self) {
        for row in 0..self.height {
            for column in 0..self.width {
                let live_neighbors = self.live_neighbor_count(row, column);
                self.grid[row][column].set_live_neighbors(live_neighbors);
            }
        }

        self.grid.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|cell| {
                cell.evolve();
            })
        });
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

                count += self.grid[neighbor_row][neighbor_col].state() as u8;
            }
        }
        count
    }

    // For testing
    #[allow(dead_code)]
    fn state_grid(&self) -> [[State; W]; H] {
        let mut states = [[State::Dead; W]; H];
        for (row_index, row) in self.grid.iter().enumerate() {
            for (col_index, cell) in row.iter().enumerate() {
                states[row_index][col_index] = cell.state();
            }
        }
        states
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
        assert_eq!(universe.grid, [[Cell::default(); 3]; 3]);
    }

    #[test]
    fn test_live_neighbor_count_no_live_neighbors() {
        let mut universe = Universe::<3, 3>::new();

        // Set the center cell to Alive
        universe.grid[1][1].set_state(State::Alive);

        // No live neighbors
        let count = universe.live_neighbor_count(1, 1);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_live_neighbor_count_some_live_neighbors() {
        let mut universe = Universe::<3, 3>::new();

        let live_cell = Cell::new().with_state(State::Alive);

        // Set some neighboring cells to Alive
        universe.grid[0][0] = live_cell;
        universe.grid[0][1] = live_cell;
        universe.grid[1][0] = live_cell;

        // Center cell has 3 live neighbors
        let count = universe.live_neighbor_count(1, 1);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_live_neighbor_count_wrap_around() {
        let mut universe = Universe::<3, 3>::new();

        let live_cell = Cell::new().with_state(State::Alive);
        // Set cells near the edges to Alive
        universe.grid[0][0] = live_cell;
        universe.grid[0][2] = live_cell;
        universe.grid[2][0] = live_cell;
        universe.grid[2][2] = live_cell;

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

        let live_cell = Cell::new().with_state(State::Alive);

        // Set up a scenario where specific evolution behavior is expected
        universe.grid[0][0] = live_cell;
        universe.evolve();

        assert_eq!(universe.state_grid(), [[State::Dead; 3]; 3]);
    }

    #[test]
    fn test_evolution_with_mutated_logic_2() {
        let mut universe = Universe::<4, 4>::new();

        let live_cell = Cell::new().with_state(State::Alive);

        // Set up a scenario where specific evolution behavior is expected
        universe.grid[0][0] = live_cell;
        universe.grid[0][1] = live_cell;
        universe.grid[1][0] = live_cell;
        universe.evolve();

        assert_eq!(
            universe.state_grid(),
            [
                [State::Alive, State::Alive, State::Dead, State::Dead],
                [State::Alive, State::Alive, State::Dead, State::Dead],
                [State::Dead, State::Dead, State::Dead, State::Dead],
                [State::Dead, State::Dead, State::Dead, State::Dead]
            ]
        );
    }
}
