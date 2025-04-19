use crate::disc::Disc;
use std::{fmt, path::Iter};

#[derive(Debug, PartialEq, Eq)]
pub enum BoardError {
    OutOfBoundsError,
    SquareOccupiedError,
}

pub struct Board {
    squares: [Disc; 64],
}

impl Board {
    pub fn new() -> Self {
        Self {
            squares: [Disc::Empty; 64],
        }
    }

    pub fn index(row: u8, col: u8) -> Result<u8, BoardError> {
        match (row, col) {
            (8.., _) => Err(BoardError::OutOfBoundsError),
            (_, 8..) => Err(BoardError::OutOfBoundsError),
            (row, col) => Ok(8 * row + col),
        }
    }

    pub fn row(index: u8) -> Result<u8, BoardError> {
        match index {
            64.. => Err(BoardError::OutOfBoundsError),
            index => Ok(index / 8),
        }
    }

    pub fn col(index: u8) -> Result<u8, BoardError> {
        match index {
            64.. => Err(BoardError::OutOfBoundsError),
            index => Ok(index % 8),
        }
    }

    pub fn get_field(self, index: usize) -> Result<Disc, BoardError> {
        self.squares
            .get(index)
            .copied()
            .ok_or(BoardError::OutOfBoundsError)
    }

    pub fn count_discs(self, color: Disc) -> usize {
        self.squares.iter().filter(|sq| **sq == color).count()
    }

    fn set_field(&mut self, index: usize, disc: Disc) -> Result<(), BoardError> {
        let square: &mut Disc = self
            .squares
            .get_mut(index)
            .ok_or(BoardError::OutOfBoundsError)?;
        *square = disc;
        Ok(())
    }

    pub fn play_move(&mut self, index: usize, disc: Disc) -> Result<(), BoardError> {
        let square: &mut Disc = self
            .squares
            .get_mut(index)
            .ok_or(BoardError::OutOfBoundsError)?;
        if *square != Disc::Empty {
            return Err(BoardError::SquareOccupiedError);
        }
        todo!("perform flipping logic");
        self.set_field(index, disc);
        Ok(())
    }

    pub fn empty_squares(&self) -> impl Iterator<Item = usize> + '_ {
        self.squares
            .iter() // Iterator<Item = &Disc>
            .copied() // Iterator<Item = Disc>
            .enumerate() // Iterator<Item = (usize, Disc)>
            .filter_map(
                |(idx, disc)| {
                    if disc == Disc::Empty {
                        Some(idx)
                    } else {
                        None
                    }
                },
            )
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..8 {
            for col in 0..8 {
                let sym = match self.squares[row * 8 + col] {
                    Disc::Black => '●',
                    Disc::White => '○',
                    Disc::Empty => '.',
                };
                write!(f, "{} ", sym)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_from_valid_row_col() {
        assert_eq!(Board::index(1, 0), Ok(8));
        assert_eq!(Board::index(2, 0), Ok(16));
        assert_eq!(Board::index(3, 0), Ok(24));
        assert_eq!(Board::index(4, 0), Ok(32));
        assert_eq!(Board::index(5, 0), Ok(40));
        assert_eq!(Board::index(6, 0), Ok(48));
        assert_eq!(Board::index(7, 0), Ok(56));

        assert_eq!(Board::index(0, 1), Ok(1));
        assert_eq!(Board::index(0, 2), Ok(2));
        assert_eq!(Board::index(0, 3), Ok(3));
        assert_eq!(Board::index(0, 4), Ok(4));
        assert_eq!(Board::index(0, 5), Ok(5));
        assert_eq!(Board::index(0, 6), Ok(6));
        assert_eq!(Board::index(0, 7), Ok(7));

        assert_eq!(Board::index(0, 0), Ok(0));
        assert_eq!(Board::index(1, 1), Ok(9));
        assert_eq!(Board::index(2, 2), Ok(18));
        assert_eq!(Board::index(3, 3), Ok(27));
        assert_eq!(Board::index(4, 4), Ok(36));
        assert_eq!(Board::index(5, 5), Ok(45));
        assert_eq!(Board::index(6, 6), Ok(54));
        assert_eq!(Board::index(7, 7), Ok(63));
    }

    #[test]
    fn index_from_invalid_row_col() {
        assert_eq!(Board::index(8, 0), Err(BoardError::OutOfBoundsError));
        assert_eq!(Board::index(0, 8), Err(BoardError::OutOfBoundsError));
        assert_eq!(Board::index(8, 8), Err(BoardError::OutOfBoundsError));
    }
}
