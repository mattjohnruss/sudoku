// sizes in each dimension
pub(crate) const BLOCK_WIDTH: usize = 3;
pub(crate) const NUM_BLOCKS_1D: usize = 3;
pub(crate) const GRID_WIDTH: usize = BLOCK_WIDTH * NUM_BLOCKS_1D;

#[derive(Debug, PartialEq)]
pub enum GridError {
    IncorrectSize { expected: usize, actual: usize },
    InvalidDigit(u32),
    InvalidCharacter(char),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Cell {
    Unknown,
    Known(u32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    cells: Vec<Cell>,
}

impl Grid {
    pub fn from_digits(digits: &[u32]) -> Result<Grid, GridError> {
        if digits.len() != GRID_WIDTH * GRID_WIDTH {
            let err = GridError::IncorrectSize {
                expected: GRID_WIDTH * GRID_WIDTH,
                actual: digits.len(),
            };
            return Err(err);
        }

        let mut cells = Vec::with_capacity(GRID_WIDTH * GRID_WIDTH);
        for &digit in digits.iter() {
            let digit = match digit {
                0 => Cell::Unknown,
                i if i >= 1 && i <= GRID_WIDTH as u32 => Cell::Known(i),
                i => return Err(GridError::InvalidDigit(i)),
            };
            cells.push(digit);
        }

        Ok(Grid { cells })
    }

    pub fn from_str(s: &str) -> Result<Grid, GridError> {
        let mut digits = Vec::with_capacity(GRID_WIDTH * GRID_WIDTH);

        for c in s.chars() {
            if c.is_whitespace() {
                continue;
            }
            if c.is_ascii_digit() {
                let digit = c.to_digit(10).unwrap();
                digits.push(digit);
                continue;
            } else if c == '.' {
                digits.push(0);
                continue;
            } else {
                return Err(GridError::InvalidCharacter(c));
            }
        }

        Self::from_digits(&digits)
    }

    pub(crate) fn cell(&self, i: usize, j: usize) -> Cell {
        self.cells[i * GRID_WIDTH + j]
    }

    #[allow(unused)]
    pub(crate) fn set_cell(&mut self, i: usize, j: usize, cell: Cell) {
        self.cells[i * GRID_WIDTH + j] = cell;
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..GRID_WIDTH {
            for j in 0..GRID_WIDTH {
                match self.cell(i, j) {
                    Cell::Known(x) => write!(f, "{}", x)?,
                    Cell::Unknown => write!(f, ".")?,
                }
                if j < GRID_WIDTH - 1 {
                    write!(f, " ")?;
                    if (j + 1) % 3 == 0 {
                        write!(f, "| ")?;
                    }
                }
            }
            if i < GRID_WIDTH - 1 {
                write!(f, "\n")?;
                if (i + 1) % 3 == 0 {
                    write!(f, "{}\n", "-".repeat(21))?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_from_digits() {
        #[rustfmt::skip]
        let digits = vec![
            0, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
        ];

        let grid = Grid::from_digits(&digits).unwrap();

        assert_eq!(grid.cells[0], Cell::Unknown);
        for (cell, i) in (&grid.cells).iter().zip((1..=9).cycle()).skip(1) {
            assert_eq!(*cell, Cell::Known(i));
        }
    }

    #[test]
    fn grid_from_digits_wrong_size() {
        #[rustfmt::skip]
        let digits = vec![
            0, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            1, 2, 3, 4, 5, 6, 7, 8,
        ];

        let grid = Grid::from_digits(&digits);

        assert_eq!(
            grid,
            Err(GridError::IncorrectSize {
                expected: 81,
                actual: 80
            })
        );
    }

    #[test]
    fn grid_from_str() {
        let chars = ".23456789\
             123456789\
             123456789\
             123456789\
             123456789\
             123456789\
             123456789\
             123456789\
             123456789";

        let grid = Grid::from_str(&chars).unwrap();

        assert_eq!(grid.cells[0], Cell::Unknown);
        for (cell, i) in (&grid.cells).iter().zip((1..=9).cycle()).skip(1) {
            assert_eq!(*cell, Cell::Known(i));
        }
    }

    #[test]
    fn grid_from_str_with_whitespace() {
        let chars = ". 2 3 4    56789
             123456789\
             123456789
             123456789\
             123456789
             123456 \t  \n789\
             12345678\t 9\
             12345 6 7 89\
             12     345678 9";

        let grid = Grid::from_str(&chars).unwrap();

        assert_eq!(grid.cells[0], Cell::Unknown);
        for (cell, i) in (&grid.cells).iter().zip((1..=9).cycle()).skip(1) {
            assert_eq!(*cell, Cell::Known(i));
        }
    }

    #[test]
    fn grid_from_str_invalid_char() {
        let chars = ".23r56789\
             123456789\
             123456789\
             123456789\
             123456789\
             123456789\
             123456789\
             123456789\
             123456789";

        let grid = Grid::from_str(&chars);

        assert_eq!(grid, Err(GridError::InvalidCharacter('r')));
    }
}
