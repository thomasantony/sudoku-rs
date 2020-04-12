use itertools::iproduct;
use std::collections::HashMap;
use std::iter::FromIterator;


pub type SudokuGrid = HashMap<(usize, usize), Option<usize>>;

pub fn grid_9x9_keys() -> impl Iterator<Item=(usize, usize)>
{
    let boxes = iproduct!(0..9, 0..9);
    boxes
}
pub fn parse_grid(s: String) -> SudokuGrid
{
    let boxes = grid_9x9_keys();
    let values = s.chars().map(|c| match c {
        '.' => None,
        c => c.to_digit(10).map(|d| d as usize),
    });

    SudokuGrid::from_iter(boxes.zip(values))
}

pub fn display_grid(g: &SudokuGrid)
{
    // width = 1+max(len(values[s]) for s in boxes)
    // let width = 2;
    let line = std::iter::repeat("-").take(9).collect::<String>();
    let line = std::iter::repeat(line).take(3).collect::<Vec<String>>().join("+");
    for r in 0..9
    {
        let value_str = (0..9).map(|c| (r, c))
                            .map(|k|
                            {
                                let num = g[&k];
                                let mut num_str = num.map_or(" . ".to_string(), |num|{
                                    format!("{:^3}", num)
                                });
                                if k.1 == 2 || k.1 == 5
                                {
                                    num_str += "|";
                                }
                                num_str
                            }).collect::<String>();
        println!("{}", value_str);
        if r == 2 || r == 5
        {
            println!("{}", line);
        }
    }
}