use crate::grid::{Cell, Grid, BLOCK_WIDTH, GRID_WIDTH, NUM_BLOCKS_1D};
use std::collections::HashSet;

#[must_use]
pub fn check_grid(grid: &Grid) -> bool {
    check_rows(grid) && check_columns(grid) && check_blocks(grid)
}

fn check_rows(grid: &Grid) -> bool {
    for i in 0..GRID_WIDTH {
        let mut seen = HashSet::with_capacity(GRID_WIDTH);

        for j in 0..GRID_WIDTH {
            match grid.cell(i, j) {
                Cell::Known(x) => {
                    if seen.contains(&x) {
                        return false;
                    } else {
                        seen.insert(x);
                    }
                }
                Cell::Unknown => return false,
            }
        }
    }
    true
}

fn check_columns(grid: &Grid) -> bool {
    for j in 0..GRID_WIDTH {
        let mut seen = HashSet::with_capacity(GRID_WIDTH);

        for i in 0..GRID_WIDTH {
            match grid.cell(i, j) {
                Cell::Known(x) => {
                    if seen.contains(&x) {
                        return false;
                    } else {
                        seen.insert(x);
                    }
                }
                Cell::Unknown => return false,
            }
        }
    }
    true
}

fn check_blocks(grid: &Grid) -> bool {
    for i in 0..NUM_BLOCKS_1D {
        for j in 0..NUM_BLOCKS_1D {
            if !check_block(&grid, i, j) {
                return false;
            }
        }
    }
    true
}

fn check_block(grid: &Grid, i: usize, j: usize) -> bool {
    let mut seen = HashSet::with_capacity(BLOCK_WIDTH * BLOCK_WIDTH);

    for k in (i * BLOCK_WIDTH)..((i + 1) * BLOCK_WIDTH) {
        for l in (j * BLOCK_WIDTH)..((j + 1) * BLOCK_WIDTH) {
            match grid.cell(k, l) {
                Cell::Known(x) => {
                    if seen.contains(&x) {
                        return false;
                    } else {
                        seen.insert(x);
                    }
                }
                Cell::Unknown => return false,
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_rows() {
        #[rustfmt::skip]
        let digits = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            2, 3, 1, 5, 6, 9, 4, 7, 8,
            2, 1, 3, 4, 6, 5, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
        ];
        let grid = Grid::from_digits(&digits).unwrap();
        assert!(check_rows(&grid));
    }

    #[test]
    fn test_check_rows_duplicates() {
        #[rustfmt::skip]
        let digits = vec![
            1, 1, 3, 4, 5, 6, 7, 8, 9,
            2, 3, 1, 5, 6, 9, 4, 7, 8,
            2, 1, 3, 4, 6, 5, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
        ];
        let grid = Grid::from_digits(&digits).unwrap();
        assert!(!check_rows(&grid));
    }

    #[test]
    fn test_check_rows_unknowns() {
        #[rustfmt::skip]
        let digits = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            2, 3, 1, 5, 6, 9, 4, 7, 8,
            2, 1, 3, 4, 0, 5, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
        ];
        let grid = Grid::from_digits(&digits).unwrap();
        assert!(!check_rows(&grid));
    }

    #[test]
    fn test_check_columns() {
        #[rustfmt::skip]
        let digits = vec![
            1, 2, 2, 1, 1, 1, 1, 1, 1,
            2, 3, 1, 2, 2, 2, 2, 2, 2,
            3, 1, 3, 3, 3, 3, 3, 3, 3,
            4, 5, 4, 4, 4, 4, 4, 4, 4,
            5, 6, 6, 5, 5, 5, 5, 5, 5,
            6, 9, 5, 6, 6, 6, 6, 6, 6,
            7, 4, 7, 7, 7, 7, 7, 7, 7,
            8, 7, 8, 8, 8, 8, 8, 8, 8,
            9, 8, 9, 9, 9, 9, 9, 9, 9,
        ];
        let grid = Grid::from_digits(&digits).unwrap();
        assert!(check_columns(&grid));
    }

    #[test]
    fn test_check_columns_duplicates() {
        #[rustfmt::skip]
        let digits = vec![
            1, 2, 2, 1, 1, 1, 1, 1, 1,
            1, 3, 1, 2, 2, 2, 2, 2, 2,
            3, 1, 3, 3, 3, 3, 3, 3, 3,
            4, 5, 4, 4, 4, 4, 4, 4, 4,
            5, 6, 6, 5, 5, 5, 5, 5, 5,
            6, 9, 5, 6, 6, 6, 6, 6, 6,
            7, 4, 7, 7, 7, 7, 7, 7, 7,
            8, 7, 8, 8, 8, 8, 8, 8, 8,
            9, 8, 9, 9, 9, 9, 9, 9, 9,
        ];
        let grid = Grid::from_digits(&digits).unwrap();
        assert!(!check_columns(&grid));
    }

    #[test]
    fn test_check_columns_unknowns() {
        #[rustfmt::skip]
        let digits = vec![
            1, 2, 2, 1, 1, 1, 1, 1, 1,
            2, 0, 1, 2, 2, 2, 2, 2, 2,
            3, 1, 3, 3, 3, 3, 3, 3, 3,
            4, 5, 4, 4, 4, 4, 4, 4, 4,
            5, 6, 6, 5, 5, 5, 5, 5, 5,
            6, 9, 5, 6, 6, 6, 6, 6, 6,
            7, 4, 7, 7, 7, 7, 7, 7, 7,
            8, 7, 8, 8, 8, 8, 8, 8, 8,
            9, 8, 9, 9, 9, 9, 9, 9, 9,
        ];
        let grid = Grid::from_digits(&digits).unwrap();
        assert!(!check_columns(&grid));
    }

    #[test]
    fn test_check_blocks() {
        #[rustfmt::skip]
        let digits = vec![
            1, 3, 2, 1, 2, 3, 1, 2, 3,
            9, 8, 7, 4, 5, 6, 4, 5, 6,
            6, 5, 4, 7, 8, 9, 7, 8, 9,
            1, 2, 3, 4, 9, 1, 1, 2, 3,
            4, 5, 6, 3, 6, 5, 4, 5, 6,
            7, 8, 9, 8, 7, 2, 7, 8, 9,
            1, 2, 3, 1, 2, 3, 1, 2, 3,
            4, 5, 6, 4, 5, 6, 4, 5, 6,
            7, 8, 9, 7, 8, 9, 7, 8, 9,
        ];
        let grid = Grid::from_digits(&digits).unwrap();
        assert!(check_blocks(&grid));
    }

    #[test]
    fn test_check_blocks_duplicates() {
        #[rustfmt::skip]
        let digits = vec![
            1, 3, 2, 1, 2, 3, 1, 2, 3,
            9, 8, 7, 4, 5, 6, 4, 5, 6,
            6, 5, 4, 7, 8, 9, 7, 8, 9,
            1, 2, 3, 4, 9, 1, 1, 2, 3,
            4, 5, 6, 3, 7, 5, 4, 5, 6,
            7, 8, 9, 8, 7, 2, 7, 8, 9,
            1, 2, 3, 1, 2, 3, 1, 2, 3,
            4, 5, 6, 4, 5, 6, 4, 5, 6,
            7, 8, 9, 7, 8, 9, 7, 8, 9,
        ];
        let grid = Grid::from_digits(&digits).unwrap();
        assert!(!check_blocks(&grid));
    }

    #[test]
    fn test_check_blocks_unknowns() {
        #[rustfmt::skip]
        let digits = vec![
            1, 3, 2, 1, 2, 3, 1, 2, 3,
            9, 8, 7, 4, 5, 0, 4, 5, 6,
            6, 5, 4, 7, 8, 9, 7, 8, 9,
            1, 2, 3, 4, 9, 1, 1, 2, 3,
            4, 5, 6, 3, 6, 5, 4, 5, 6,
            7, 8, 9, 8, 7, 2, 7, 8, 9,
            1, 2, 3, 1, 2, 3, 1, 2, 3,
            4, 5, 6, 4, 5, 6, 4, 5, 6,
            7, 8, 9, 7, 8, 0, 7, 8, 9,
        ];
        let grid = Grid::from_digits(&digits).unwrap();
        assert!(!check_blocks(&grid));
    }
}
