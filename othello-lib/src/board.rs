use crate::disc::Disc;
use arrayvec::ArrayVec;
use std::fmt::{self, Debug};

#[derive(Debug, PartialEq, Eq)]
pub enum BoardError {
    OutOfBounds,
    SquareOccupied,
    InvalidMove,
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}
impl Direction {
    fn delta_row_col(self) -> (isize, isize) {
        match self {
            Self::North => (-1, 0),
            Self::NorthEast => (-1, 1),
            Self::East => (0, 1),
            Self::SouthEast => (1, 1),
            Self::South => (1, 0),
            Self::SouthWest => (1, -1),
            Self::West => (0, -1),
            Self::NorthWest => (-1, -1),
        }
    }
    const ALL: [Direction; 8] = [
        Self::North,
        Self::NorthEast,
        Self::East,
        Self::SouthEast,
        Self::South,
        Self::SouthWest,
        Self::West,
        Self::NorthWest,
    ];
}

#[derive(Clone, PartialEq, Eq)]
pub struct Board {
    squares: [Option<Disc>; Board::BOARD_SURFACE],
}

impl Board {
    const BOARD_WIDTH: usize = 8;
    const BOARD_HEIGHT: usize = 8;
    const BOARD_MAX_DIM: usize = 8; // should be equal to the max of WIDTH and HEIGHT
    const BOARD_SURFACE: usize = Board::BOARD_WIDTH * Board::BOARD_HEIGHT;

    pub fn new() -> Self {
        let mut board = Self {
            squares: [None; Self::BOARD_SURFACE],
        };

        let mid_row = Self::BOARD_HEIGHT / 2;
        let mid_col = Self::BOARD_WIDTH / 2;

        let init = [
            (mid_row, mid_col, Disc::White),
            (mid_row - 1, mid_col, Disc::Black),
            (mid_row, mid_col - 1, Disc::Black),
            (mid_row - 1, mid_col - 1, Disc::White),
        ];

        for &(r, c, disc) in &init {
            let idx = board.index(r, c).expect("center coords should be valid");
            board
                .set_field(idx, disc)
                .expect("setting initial disc cannot fail");
        }

        board
    }

    pub fn index(&self, row: usize, col: usize) -> Result<usize, BoardError> {
        match (row, col) {
            (Board::BOARD_HEIGHT.., _) => Err(BoardError::OutOfBounds),
            (_, Board::BOARD_WIDTH..) => Err(BoardError::OutOfBounds),
            (row, col) => Ok(Board::BOARD_WIDTH * row + col),
        }
    }

    pub fn row_col(&self, index: usize) -> Result<(usize, usize), BoardError> {
        match index {
            Board::BOARD_SURFACE.. => Err(BoardError::OutOfBounds),
            index => Ok((index / Board::BOARD_WIDTH, index % Board::BOARD_WIDTH)),
        }
    }

    fn step_row(&self, row: usize, delta: isize) -> Option<usize> {
        Self::step_coord(row, delta, Self::BOARD_HEIGHT)
    }

    fn step_col(&self, col: usize, delta: isize) -> Option<usize> {
        Self::step_coord(col, delta, Self::BOARD_WIDTH)
    }

    const fn step_coord(coord: usize, delta: isize, limit: usize) -> Option<usize> {
        let next = coord as isize + delta;
        if next < 0 || next >= limit as isize {
            None
        } else {
            Some(next as usize)
        }
    }

    fn next_index(&self, index: usize, direction: Direction) -> Option<usize> {
        let (row, col) = self.row_col(index).ok()?;
        let (dr, dc) = direction.delta_row_col();

        let next_row = self.step_row(row, dr)?;
        let next_col = self.step_col(col, dc)?;

        let next_index = self.index(next_row, next_col).ok()?;
        Some(next_index)
    }

    pub fn get_field(&self, index: usize) -> Result<Option<Disc>, BoardError> {
        self.squares
            .get(index)
            .copied()
            .ok_or(BoardError::OutOfBounds)
    }

    fn set_field(&mut self, index: usize, disc: Disc) -> Result<(), BoardError> {
        let square: &mut Option<Disc> = self.squares.get_mut(index).ok_or(BoardError::OutOfBounds)?;
        *square = Some(disc);
        Ok(())
    }

    fn flips_in_direction(
        &self,
        start: usize,
        disc: Disc,
        dir: Direction,
    ) -> Option<ArrayVec<usize, { Board::BOARD_MAX_DIM }>> {
        let opponent = disc.opposite();
        let mut flips = ArrayVec::<usize, { Board::BOARD_MAX_DIM }>::new();
        let mut index = self.next_index(start, dir)?;
        if self.get_field(index).ok()? != Some(opponent) {
            return None;
        }
        flips.push(index);
        while let Some(next) = self.next_index(index, dir) {
            index = next;
            match self.get_field(index).ok()? {
                Some(d) if d == opponent => flips.push(index),
                Some(d) if d == disc => return Some(flips),
                _ => return None,
            }
        }
        None
    }

    fn all_flips(
        &self,
        start: usize,
        disc: Disc,
    ) -> Option<ArrayVec<usize, { Board::BOARD_SURFACE }>> {
        let mut all = ArrayVec::<usize, { Board::BOARD_SURFACE }>::new();
        for &dir in Direction::ALL.iter() {
            if let Some(flips) = self.flips_in_direction(start, disc, dir) {
                all.try_extend_from_slice(&flips).ok()?;
            }
        }
        if all.is_empty() {
            None
        } else {
            Some(all)
        }
    }

    pub fn apply_move(&mut self, start: usize, disc: Disc) -> Result<(), BoardError> {
        match self.get_field(start) {
            Ok(None) => {}
            Ok(_) => return Err(BoardError::SquareOccupied),
            Err(_) => return Err(BoardError::OutOfBounds),
        }
        let flips = self.all_flips(start, disc).ok_or(BoardError::InvalidMove)?;
        self.set_field(start, disc)?;
        for index in flips {
            self.set_field(index, disc)?
        }
        Ok(())
    }

    pub fn is_valid_move(&self, start: usize, disc: Disc) -> bool {
        let Ok(None) = self.get_field(start) else {
            return false;
        };
        self.all_flips(start, disc).is_some()
    }

    pub fn count_discs(&self, disc: Disc) -> usize {
        self.squares
            .iter()
            .copied()
            .filter(|&s| s == Some(disc))
            .count()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..Board::BOARD_HEIGHT {
            for col in 0..Board::BOARD_WIDTH {
                let sym = match self.squares[row * Board::BOARD_WIDTH + col] {
                    Some(Disc::Black) => '○',
                    Some(Disc::White) => '●',
                    None => '.',
                };
                write!(f, "{} ", sym)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..Board::BOARD_HEIGHT {
            for col in 0..Board::BOARD_WIDTH {
                let sym = match self.squares[row * Board::BOARD_WIDTH + col] {
                    Some(Disc::Black) => '○',
                    Some(Disc::White) => '●',
                    None => '.',
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
    fn index_valid_coordinates() {
        let board = Board::new();
        assert_eq!(board.index(0, 0), Ok(0));
        assert_eq!(board.index(0, 7), Ok(7));
        assert_eq!(board.index(7, 0), Ok(56));
        assert_eq!(board.index(7, 7), Ok(63))
    }

    #[test]
    fn index_out_of_bounds() {
        let board = Board::new();
        assert_eq!(board.index(0, 8), Err(BoardError::OutOfBounds));
        assert_eq!(board.index(8, 0), Err(BoardError::OutOfBounds));
        assert_eq!(board.index(8, 8), Err(BoardError::OutOfBounds));
    }

    #[test]
    fn row_col_valid_indices() {
        let board = Board::new();
        assert_eq!(board.row_col(0), Ok((0, 0)));
        assert_eq!(board.row_col(7), Ok((0, 7)),);
        assert_eq!(board.row_col(56), Ok((7, 0)),);
        assert_eq!(board.row_col(63), Ok((7, 7)),);
    }

    #[test]
    fn row_col_out_of_bounds() {
        let board = Board::new();
        assert_eq!(board.row_col(64), Err(BoardError::OutOfBounds));
    }

    #[test]
    fn get_field_valid() {
        let board = Board::new();
        assert_eq!(board.get_field(0), Ok(None));
        assert_eq!(board.get_field(27), Ok(Some(Disc::White)));
        assert_eq!(board.get_field(28), Ok(Some(Disc::Black)));
        assert_eq!(board.get_field(35), Ok(Some(Disc::Black)));
        assert_eq!(board.get_field(36), Ok(Some(Disc::White)));
        assert_eq!(board.get_field(63), Ok(None));
    }

    #[test]
    fn get_field_out_of_bounds() {
        let board = Board::new();
        assert_eq!(board.get_field(64), Err(BoardError::OutOfBounds));
    }

    #[test]
    fn set_field_valid() {
        let mut board = Board::new();
        assert_eq!(board.get_field(0), Ok(None));
        assert_eq!(board.set_field(0, Disc::White), Ok(()));
        assert_eq!(board.get_field(0), Ok(Some(Disc::White)));
        assert_eq!(board.set_field(0, Disc::Black), Ok(()));
        assert_eq!(board.get_field(0), Ok(Some(Disc::Black)));
    }

    #[test]
    fn set_field_out_of_bounds() {
        let mut board = Board::new();
        assert_eq!(
            board.set_field(64, Disc::White),
            Err(BoardError::OutOfBounds)
        );
    }

    #[test]
    fn next_index_in_bounds() {
        let board = Board::new();
        assert_eq!(board.next_index(9, Direction::North), Some(1));
        assert_eq!(board.next_index(9, Direction::NorthEast), Some(2));
        assert_eq!(board.next_index(9, Direction::East), Some(10));
        assert_eq!(board.next_index(9, Direction::SouthEast), Some(18));
        assert_eq!(board.next_index(9, Direction::South), Some(17));
        assert_eq!(board.next_index(9, Direction::SouthWest), Some(16));
        assert_eq!(board.next_index(9, Direction::West), Some(8));
        assert_eq!(board.next_index(9, Direction::NorthWest), Some(0));

        assert_eq!(board.next_index(54, Direction::North), Some(46));
        assert_eq!(board.next_index(54, Direction::NorthEast), Some(47));
        assert_eq!(board.next_index(54, Direction::East), Some(55));
        assert_eq!(board.next_index(54, Direction::SouthEast), Some(63));
        assert_eq!(board.next_index(54, Direction::South), Some(62));
        assert_eq!(board.next_index(54, Direction::SouthWest), Some(61));
        assert_eq!(board.next_index(54, Direction::West), Some(53));
        assert_eq!(board.next_index(54, Direction::NorthWest), Some(45));
    }

    #[test]
    fn next_index_out_of_bounds() {
        let board = Board::new();
        assert_eq!(board.next_index(0, Direction::SouthWest), None);
        assert_eq!(board.next_index(0, Direction::West), None);
        assert_eq!(board.next_index(0, Direction::NorthWest), None);
        assert_eq!(board.next_index(0, Direction::North), None);
        assert_eq!(board.next_index(0, Direction::NorthEast), None);

        assert_eq!(board.next_index(7, Direction::NorthWest), None);
        assert_eq!(board.next_index(7, Direction::North), None);
        assert_eq!(board.next_index(7, Direction::NorthEast), None);
        assert_eq!(board.next_index(7, Direction::East), None);
        assert_eq!(board.next_index(7, Direction::SouthEast), None);

        assert_eq!(board.next_index(63, Direction::NorthEast), None);
        assert_eq!(board.next_index(63, Direction::East), None);
        assert_eq!(board.next_index(63, Direction::SouthEast), None);
        assert_eq!(board.next_index(63, Direction::South), None);
        assert_eq!(board.next_index(63, Direction::SouthWest), None);

        assert_eq!(board.next_index(56, Direction::SouthEast), None);
        assert_eq!(board.next_index(56, Direction::South), None);
        assert_eq!(board.next_index(56, Direction::SouthWest), None);
        assert_eq!(board.next_index(56, Direction::West), None);
        assert_eq!(board.next_index(56, Direction::NorthWest), None);
    }

    #[test]
    fn flips_in_direction_some() {
        let mut board = Board::new();
        assert!(board
            .flips_in_direction(44, Disc::Black, Direction::North)
            .is_some());
        assert!(board
            .flips_in_direction(37, Disc::Black, Direction::West)
            .is_some());
        assert!(board
            .flips_in_direction(20, Disc::White, Direction::South)
            .is_some());
        assert!(board
            .flips_in_direction(29, Disc::White, Direction::West)
            .is_some());
        board.set_field(18, Disc::Black).unwrap();
        assert!(board
            .flips_in_direction(45, Disc::Black, Direction::NorthWest)
            .is_some())
    }

    #[test]
    fn flips_in_direction_none() {
        let mut board = Board::new();
        assert!(board
            .flips_in_direction(44, Disc::White, Direction::North)
            .is_none());
        assert!(board
            .flips_in_direction(37, Disc::White, Direction::West)
            .is_none());
        assert!(board
            .flips_in_direction(20, Disc::Black, Direction::South)
            .is_none());
        assert!(board
            .flips_in_direction(29, Disc::Black, Direction::West)
            .is_none());
        board.set_field(36, Disc::Black).unwrap();
        assert!(board
            .flips_in_direction(44, Disc::Black, Direction::North)
            .is_none());
        assert!(board
            .flips_in_direction(20, Disc::White, Direction::South)
            .is_none());

        assert!(board
            .flips_in_direction(0, Disc::White, Direction::North)
            .is_none());
        assert!(board
            .flips_in_direction(0, Disc::White, Direction::North)
            .is_none());
        assert!(board
            .flips_in_direction(0, Disc::Black, Direction::South)
            .is_none());
        assert!(board
            .flips_in_direction(0, Disc::Black, Direction::South)
            .is_none());
    }

    #[test]
    fn all_flips_some() {
        let mut board = Board::new();
        assert!(board.all_flips(44, Disc::Black).is_some());
        assert!(board.all_flips(37, Disc::Black).is_some());
        assert!(board.all_flips(20, Disc::White).is_some());
        assert!(board.all_flips(29, Disc::White).is_some());
        board.set_field(18, Disc::Black).unwrap();
        assert!(board.all_flips(45, Disc::Black).is_some())
    }

    #[test]
    fn all_flips_none() {
        let mut board = Board::new();
        assert!(board.all_flips(44, Disc::White).is_none());
        assert!(board.all_flips(37, Disc::White).is_none());
        assert!(board.all_flips(20, Disc::Black).is_none());
        assert!(board.all_flips(29, Disc::Black).is_none());
        board.set_field(36, Disc::Black).unwrap();
        assert!(board.all_flips(44, Disc::Black).is_none());
        assert!(board.all_flips(20, Disc::White).is_none());

        assert!(board.all_flips(0, Disc::White).is_none());
        assert!(board.all_flips(0, Disc::White).is_none());
        assert!(board.all_flips(0, Disc::Black).is_none());
        assert!(board.all_flips(0, Disc::Black).is_none());
    }

    #[test]
    fn is_valid_move_valid() {
        let mut board = Board::new();
        assert_eq!(board.is_valid_move(44, Disc::Black), true);
        assert_eq!(board.is_valid_move(37, Disc::Black), true);
        assert_eq!(board.is_valid_move(20, Disc::White), true);
        assert_eq!(board.is_valid_move(29, Disc::White), true);
        board.set_field(18, Disc::Black).unwrap();
        assert_eq!(board.is_valid_move(45, Disc::Black), true);
    }

    #[test]
    fn is_valid_move_occupied() {
        let mut board = Board::new();
        assert_eq!(board.is_valid_move(36, Disc::Black), false);
        assert_eq!(board.is_valid_move(36, Disc::White), false);
        board.set_field(20, Disc::White).unwrap();
        assert_eq!(board.is_valid_move(36, Disc::Black), false);
        assert_eq!(board.is_valid_move(36, Disc::White), false);
        board.set_field(36, Disc::Black).unwrap();
        assert_eq!(board.is_valid_move(36, Disc::Black), false);
        assert_eq!(board.is_valid_move(36, Disc::White), false);
    }

    #[test]
    fn is_valid_move_out_of_bounds() {
        let board = Board::new();
        assert_eq!(board.is_valid_move(64, Disc::White), false);
        assert_eq!(board.is_valid_move(64, Disc::Black), false);
    }

    #[test]
    fn is_valid_move_invalid() {
        let mut board = Board::new();
        assert_eq!(board.is_valid_move(44, Disc::White), false);
        assert_eq!(board.is_valid_move(37, Disc::White), false);
        assert_eq!(board.is_valid_move(20, Disc::Black), false);
        assert_eq!(board.is_valid_move(29, Disc::Black), false);
        board.set_field(36, Disc::Black).unwrap();
        assert_eq!(board.is_valid_move(44, Disc::Black), false);
        assert_eq!(board.is_valid_move(20, Disc::White), false);

        assert_eq!(board.is_valid_move(0, Disc::White), false);
        assert_eq!(board.is_valid_move(0, Disc::White), false);
        assert_eq!(board.is_valid_move(0, Disc::Black), false);
        assert_eq!(board.is_valid_move(0, Disc::Black), false);
    }

    #[test]
    fn apply_move_valid() {
        let mut board = Board::new();

        assert_eq!(board.apply_move(44, Disc::Black), Ok(()));
        assert_eq!(board.get_field(44), Ok(Some(Disc::Black)));
        assert_eq!(board.get_field(36), Ok(Some(Disc::Black)));

        assert_eq!(board.apply_move(45, Disc::White), Ok(()));
        assert_eq!(board.get_field(45), Ok(Some(Disc::White)));
        assert_eq!(board.get_field(36), Ok(Some(Disc::White)));

        assert_eq!(board.apply_move(37, Disc::Black), Ok(()));
        assert_eq!(board.get_field(37), Ok(Some(Disc::Black)));
        assert_eq!(board.get_field(36), Ok(Some(Disc::Black)));

        assert_eq!(board.apply_move(43, Disc::White), Ok(()));
        assert_eq!(board.get_field(43), Ok(Some(Disc::White)));
        assert_eq!(board.get_field(35), Ok(Some(Disc::White)));
        assert_eq!(board.get_field(44), Ok(Some(Disc::White)));
    }

    #[test]
    fn apply_move_occupied() {
        let mut board = Board::new();
        let mut reference = Board::new();

        // does not have to be a BoardError::SquareOccupied, since BoardError::InvalidMove also applies, as this move would flip nothing
        assert!(board.apply_move(35, Disc::Black).is_err());
        assert_eq!(board, reference);
        board.set_field(19, Disc::Black).unwrap();
        reference.set_field(19, Disc::Black).unwrap();
        // this move would have flipped 27, so the only error that applies is BoardError::SquareOccupied, therefore we can check for an exact match
        assert_eq!(
            board.apply_move(35, Disc::Black),
            Err(BoardError::SquareOccupied)
        );
        assert_eq!(board, reference);
        board.set_field(35, Disc::Black).unwrap();
        reference.set_field(35, Disc::Black).unwrap();
        // this again does not have to be a BoardError::SquareOccupied, since BoardError::InvalidMove also applies, as this move would flip nothing
        assert!(board.apply_move(35, Disc::Black).is_err());
        assert_eq!(board, reference);
    }

    #[test]
    fn apply_move_out_of_bounds() {
        let mut board = Board::new();
        let mut reference = Board::new();

        // does not have to be a BoardError::OutOfBounds, since BoardError::InvalidMove also applies, as this move would flip nothing
        assert!(board.apply_move(64, Disc::Black).is_err());
        assert_eq!(board, reference);

        board.set_field(63, Disc::White).unwrap();
        reference.set_field(63, Disc::White).unwrap();
        board.set_field(62, Disc::Black).unwrap();
        reference.set_field(62, Disc::Black).unwrap();

        // this move would have flipped 63, so the only error that applies is BoardError::OutOfBounds, therefore we can check for an exact match
        assert_eq!(
            board.apply_move(64, Disc::Black),
            Err(BoardError::OutOfBounds)
        );
        assert_eq!(board, reference);
    }

    #[test]
    fn apply_move_invalid() {
        let mut board = Board::new();
        let mut reference = Board::new();

        assert_eq!(
            board.apply_move(0, Disc::Black),
            Err(BoardError::InvalidMove)
        );

        board.set_field(1, Disc::White).unwrap();
        reference.set_field(1, Disc::White).unwrap();
        board.set_field(2, Disc::Black).unwrap();
        reference.set_field(2, Disc::Black).unwrap();

        assert_eq!(board, reference);

        assert_eq!(board.apply_move(0, Disc::Black), Ok(()));

        assert_ne!(board, reference)
    }

    #[test]
    fn board_constructor() {
        let board = Board::new();

        for index in 0..64 {
            match index {
                27 => assert_eq!(board.get_field(index), Ok(Some(Disc::White))),
                28 => assert_eq!(board.get_field(index), Ok(Some(Disc::Black))),
                35 => assert_eq!(board.get_field(index), Ok(Some(Disc::Black))),
                36 => assert_eq!(board.get_field(index), Ok(Some(Disc::White))),
                _ => assert_eq!(board.get_field(index), Ok(None)),
            }
        }
    }

    fn assert_counts(board: &Board, black: usize, white: usize) {
        assert_eq!(board.count_discs(Disc::Black), black);
        assert_eq!(board.count_discs(Disc::White), white);
    }

    #[test]
    fn count_discs() {
        let mut board = Board::new();

        assert_counts(&board, 2, 2);

        // (square_to_play, color_to_play, expected_black, expected_white)
        let moves = [
            (19, Disc::Black, 4, 1),
            (18, Disc::White, 3, 3),
            (17, Disc::Black, 5, 2),
            (29, Disc::White, 4, 4),
        ];

        for &(pos, disc, exp_black, exp_white) in &moves {
            board.apply_move(pos, disc).unwrap();
            assert_counts(&board, exp_black, exp_white);
        }
    }
}
