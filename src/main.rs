mod utils;
use varisat::{Var, Lit};
use varisat::solver::Solver;
use varisat::ExtendFormula;

fn lit_from_value(row: usize, col: usize, value: usize) -> Lit {
    Var::from_index(row * 9 * 9 + col * 9 + value).lit(true)
}
fn main() {
    // // let boxes = utils::create_grid_9x9_keys();
    // let grid_str = "..3.2.6..9..3.5..1..18.64....81.29..7.......8..67.82....26.95..8..2.3..9..5.1.3..";

    // // println!("{:?}", utils::parse_grid(grid_str.to_string()));
    // let grid = utils::parse_grid(grid_str.to_string());
    // utils::display_grid(grid);

    let mut solver = Solver::new();

    // Each row has all numbers from 1..9 exactly once
    for row in 0..9 {
        for value in 0..9 {
            let mut literals: Vec<Lit> = Vec::new();
            for col in 0..9 {
                literals.push(lit_from_value(row, col, value));
            }
            println!("{}:{} {:?}", row, value, &literals);
            solver.add_clause(&literals);
        }
    }
    // Each column has all numbers from 1..9 exactly once
    for col in 0..9 {
        for value in 0..9 {
            let mut literals: Vec<Lit> = Vec::new();
            for row in 0..9 {
                literals.push(lit_from_value(row, col, value));
            }
            solver.add_clause(literals.as_slice());
        }
    }

    // Each box has all numbers from 1..9 exactly once
    for col in 0..9 {
        for value in 0..9 {
            let mut literals: Vec<Lit> = Vec::new();
            for row in 0..9 {
                literals.push(lit_from_value(row, col, value));
            }
            solver.add_clause(literals.as_slice());
        }
    }
}
