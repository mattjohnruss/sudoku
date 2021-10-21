use sudoku::check::check_grid;
use sudoku::grid::Grid;
use sudoku::solve::solve;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: no sudoku provided on the command line");
        std::process::exit(1);
    }

    let grid_chars = &args[1];

    let mut grid = match Grid::from_str(&grid_chars) {
        Ok(grid) => grid,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            std::process::exit(1);
        }
    };

    println!("{}", grid);
    println!("{}", check_grid(&grid));

    let success = solve(&mut grid);

    println!("{}", success);
    println!("{}", grid);
}
