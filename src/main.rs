mod utils;

fn main() {
    // let boxes = utils::create_grid_9x9_keys();
    let grid_str = "..3.2.6..9..3.5..1..18.64....81.29..7.......8..67.82....26.95..8..2.3..9..5.1.3..";

    // println!("{:?}", utils::parse_grid(grid_str.to_string()));
    let grid = utils::parse_grid(grid_str.to_string());
    utils::display_grid(grid);
}
