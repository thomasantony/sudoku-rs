mod utils;
use varisat::{Var, Lit};
use varisat::solver::Solver;
use itertools::{iproduct};
use varisat::{CnfFormula, ExtendFormula};

fn lit_from_value(row: usize, col: usize, value: usize) -> Lit {
    Var::from_index(row * 9 * 9 + col * 9 + value).lit(true)
}

fn exactly_once(literals: &Vec<Lit>) -> CnfFormula{
    let mut formula = CnfFormula::new();
    
    formula.add_clause(literals);
    for (i, lit1) in literals.iter().enumerate() {
        for lit2 in literals[i+1..].iter() {
            formula.add_clause(&[!lit1.clone(), !lit2.clone()]);
        }
    }
    formula
}
fn literals_from_board(board: &utils::SudokuGrid) -> Vec<Lit>
{
    board.iter().map(|((row, col), cell)| {
        match cell {
            Some(value) => {
                println!("Adding value {} at {},{}", value, row, col);
                Some(lit_from_value(*row, *col, *value-1))
            },
            _ => None
        }
    })
    .flatten()
    .collect()
}
fn main() {
    // // let boxes = utils::create_grid_9x9_keys();
    let grid_str = "..3.2.6..9..3.5..1..18.64....81.29..7.......8..67.82....26.95..8..2.3..9..5.1.3..";

    // // println!("{:?}", utils::parse_grid(grid_str.to_string()));
    let grid = utils::parse_grid(grid_str.to_string());
    utils::display_grid(&grid);

    let mut solver = Solver::new();

    // Each row has all numbers from 1..9 exactly once
    for (row, value) in iproduct!(0..9, 0..9)
    {
        let mut literals: Vec<Lit> = Vec::new();
        for col in 0..9 {
            literals.push(lit_from_value(row, col, value));
        }
        solver.add_formula(& exactly_once(&literals));
    }
    // Each column has all numbers from 1..9 exactly once
    for (col, value) in iproduct!(0..9, 0..9)
    {
        let mut literals: Vec<Lit> = Vec::new();
        for row in 0..9 {
            literals.push(lit_from_value(row, col, value));
        }
        solver.add_clause(literals.as_slice());
    }

    // Each box has all numbers from 1..9 exactly once
    for value in 0..9 
    {
        for (r, c) in iproduct!(&[0,3,6], &[0,3,6])
        {
            let mut literals: Vec<Lit> = Vec::new();
            for (rr, cc) in iproduct!(&[0, 1, 2], 
                                      &[0, 1, 2])
            {
                let row = (r + rr) as usize;
                let col = (c + cc) as usize;
                literals.push(lit_from_value(row, col, value));
            }
            solver.add_clause(literals.as_slice());
        }
    }
    // Add in pre-filled boxes
    let board_lit = literals_from_board(&grid);
    println!("{:?}", &board_lit);
    solver.add_clause(& board_lit);

    // let solution = solver.solve().unwrap();
    let model = solver.model().unwrap(); // None if solve didn't return Ok(true)
    println!("Solution: {:?}", model);
}
