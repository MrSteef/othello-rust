use othello_lib::{board::Board, disc::Disc, player::Player};
use std::io::{self, Write};

/// Reads moves from stdin for a human player.
pub struct HumanPlayer;

impl HumanPlayer {
    pub fn new() -> Self {
        HumanPlayer
    }
}

impl Player for HumanPlayer {
    fn select_move(&self, board: &Board, disc: Disc) -> usize {
        loop {
            println!("{}", board);

            // let moves = board.all_flips(0, disc).unwrap_or_default(); // placeholder
            print!("Enter move for {:?}: ", disc);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                continue;
            }
            if let Ok(idx) = input.trim().parse::<usize>() {
                if board.is_valid_move(idx, disc) {
                    return idx;
                }
            }
            println!("Invalid move, try again.");
        }
    }
}