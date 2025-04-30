use othello_lib::player::Player;

pub struct ComputerPlayer;

impl Player for ComputerPlayer {
    fn select_move(&self, board: &othello_lib::board::Board, disc: othello_lib::disc::Disc) -> usize {
        board.valid_moves(disc)[0]
    }
}