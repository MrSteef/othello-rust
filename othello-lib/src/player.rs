use crate::board::Board;
use crate::disc::Disc;

pub trait Player {
    fn select_move(&self, board: &Board, disc: Disc) -> usize;
}