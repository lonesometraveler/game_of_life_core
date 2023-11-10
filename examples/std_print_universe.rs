use game_of_life_core::{Cell, Universe};

const WIDTH: usize = 24;
const HEIGHT: usize = 16;

fn main() {
    let mut matrix = Universe::<WIDTH, HEIGHT>::new();
    seed(&mut matrix);

    for index in 0..20 {
        print_generation(index, &matrix);
        matrix.evolve();
    }
}

/// Seed the universe
fn seed(universe: &mut Universe<WIDTH, HEIGHT>) {
    for (r_index, row) in universe.grid().iter().enumerate() {
        for (c_index, _cell) in row.iter().enumerate() {
            if rand::random() {
                universe.set_cell(r_index, c_index, Cell::Alive);
            } else {
                universe.set_cell(r_index, c_index, Cell::Dead);
            };
        }
    }
}

/// Print the generation
fn print_generation(index: usize, matrix: &Universe<WIDTH, HEIGHT>) {
    println!("Generation: {}", index);
    for row in matrix.grid() {
        for cell in row {
            print!("{} ", cell_to_string(cell));
        }
        println!();
    }
    println!();
}

// Convert Cell to String
fn cell_to_string(cell: Cell) -> String {
    match cell {
        Cell::Dead => "-".to_string(),
        Cell::Alive => "*".to_string(),
    }
}
