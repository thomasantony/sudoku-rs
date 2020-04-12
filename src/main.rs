mod utils;
use varisat::{Lit};
use varisat::solver::Solver;
use itertools::{iproduct};
use varisat::{CnfFormula, ExtendFormula};
use utils::SudokuGrid;
use std::iter::FromIterator;

fn lit_from_value(row: usize, col: usize, value: usize) -> Lit {
    Lit::from_index(row * 9 * 9 + col * 9 + value, true)
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
fn literals_from_board(board: &SudokuGrid) -> CnfFormula
{
    let mut formula = CnfFormula::new();
    board.iter().for_each(|((row, col), cell)| {
        match cell {
            Some(value) => {
                formula.add_clause(&[lit_from_value(*row, *col, *value -1)]);
            },
            _ => {}
        }
    });
    formula
}
fn board_from_solution(model: Vec<Lit>) -> SudokuGrid
{
    let grid_cells = model.iter()
        .filter(|l| l.is_positive())
        .map(|lit|{
            let index = lit.index();
            let row = index / 81;
            let col = (index % 81) / 9;
            let value = (index % 81) % 9;

            ((row, col), Some(value+1))
        });
    SudokuGrid::from_iter(grid_cells)
}
fn main() {
    let puzzle_str = "..3.2.6..9..3.5..1..18.64....81.29..7.......8..67.82....26.95..8..2.3..9..5.1.3..";
    let puzzle = utils::parse_grid(puzzle_str.to_string());
    println!("Puzzle:");
    utils::display_grid(&puzzle);

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

    // Each number only once
    for (row, col) in iproduct!(0..9, 0..9)
    {
        let mut literals: Vec<Lit> = Vec::new();
        for value in 0..9
        {
            literals.push(lit_from_value(row, col, value));
        }
        solver.add_formula(& exactly_once(&literals));
    }
    // Add in pre-filled numbers
    solver.add_formula(& literals_from_board(&puzzle));

    solver.solve().unwrap();
    let model = solver.model().unwrap(); // None if solve didn't return Ok(true)

    println!("\nSolution:");
   
    let solution = board_from_solution(model);
    utils::display_grid(&solution);
}
