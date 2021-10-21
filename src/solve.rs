use crate::check::check_grid;
use crate::grid::{Cell, Grid, BLOCK_WIDTH, GRID_WIDTH};

fn value_in_row(grid: &Grid, i: usize, x: u32) -> bool {
    for j in 0..GRID_WIDTH {
        if let Cell::Known(y) = grid.cell(i, j) {
            if y == x {
                return true;
            }
        }
    }
    false
}

fn value_in_column(grid: &Grid, j: usize, x: u32) -> bool {
    for i in 0..GRID_WIDTH {
        if let Cell::Known(y) = grid.cell(i, j) {
            if y == x {
                return true;
            }
        }
    }
    false
}

fn value_in_block(grid: &Grid, i: usize, j: usize, x: u32) -> bool {
    for k in (i * BLOCK_WIDTH)..((i + 1) * BLOCK_WIDTH) {
        for l in (j * BLOCK_WIDTH)..((j + 1) * BLOCK_WIDTH) {
            if let Cell::Known(y) = grid.cell(k, l) {
                if y == x {
                    return true;
                }
            }
        }
    }
    false
}

fn is_safe(grid: &Grid, i: usize, j: usize, x: u32) -> bool {
    !value_in_row(grid, i, x)
        && !value_in_column(grid, j, x)
        && !value_in_block(grid, i / BLOCK_WIDTH, j / BLOCK_WIDTH, x)
}

fn get_unassigned_cell(grid: &Grid) -> Option<(usize, usize)> {
    for i in 0..GRID_WIDTH {
        for j in 0..GRID_WIDTH {
            if let Cell::Unknown = grid.cell(i, j) {
                return Some((i, j));
            }
        }
    }
    None
}

pub fn solve(grid: &mut Grid) -> bool {
    let (i, j) = match get_unassigned_cell(grid) {
        Some(unassigned) => unassigned,
        None => return check_grid(grid),
    };

    for x in 1..=9 {
        if is_safe(grid, i, j, x) {
            // assign the candidate value in the grid
            grid.set_cell(i, j, Cell::Known(x));

            // check the grid with the candidate value
            if solve(grid) {
                // if the grid with candidate value is a solution return true
                return true;
            } else {
                // if it's not a solution, set the cell back to Unknown
                grid.set_cell(i, j, Cell::Unknown);
            }
        }
    }

    // if we've reached here, all the possible candidates have been tried for the
    // current unassigned cell, and returning false allows us to backtrack
    false
}
