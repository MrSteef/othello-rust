use othello_lib::game::Game;
use std::error::Error;

pub mod human;
pub mod computer;

/// Runs the Othello CLI game loop.
/// Returns an error if I/O or game logic fails.
pub fn run() -> Result<(), Box<dyn Error>> {
    let human = Box::new(human::HumanPlayer::new());
    let computer = Box::new(computer::ComputerPlayer);

    let mut game = Game::new(human, computer);
    game.run();
    Ok(())
}