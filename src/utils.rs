use itertools::iproduct;
use std::collections::HashMap;
use std::iter::FromIterator;


pub type SudokuGrid = HashMap<(char, char), Option<u32>>;
static ROWS: &str = "ABCDEFGHI";
static COLS: &str = "123456789";

pub fn grid_9x9_keys(rows: &'static str, cols: &'static str) -> impl Iterator<Item=(char, char)>
{
    let boxes = iproduct!(rows.chars(), cols.chars());
    boxes
}
pub fn parse_grid(s: String) -> SudokuGrid
{
    let boxes = grid_9x9_keys(ROWS, COLS);
    let values = s.chars().map(|c| match c {
        '.' => None,
        c => c.to_digit(10),
    });

    SudokuGrid::from_iter(boxes.zip(values))
}

pub fn display_grid(g: SudokuGrid)
{
    // width = 1+max(len(values[s]) for s in boxes)
    // let width = 2;
    let line = "---------+---------+---------";
    for r in ROWS.chars()
    {
        let value_str = COLS.chars()
                            .map(|c| (r, c))
                            .map(|k|
                            {
                                let num = g[&k];
                                let mut num_str = num.map_or(" . ".to_string(), |num|{
                                    format!("{:^3}", num)
                                });
                                if k.1 == '3' || k.1 == '6'
                                {
                                    num_str += "|";
                                }
                                num_str
                            }).collect::<String>();
        println!("{}", value_str);
        if r == 'C' || r == 'F'
        {
            println!("{}", line);
        }
    }
}