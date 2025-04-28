use crate::board::{Board, BoardError};
use crate::disc::Disc;
use crate::player::Player;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameError {
    InvalidMove,
    BoardError(BoardError),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameOutcome {
    Tie,
    Winner(Disc)
}

pub struct Game {
    board: Board,
    black: Box<dyn Player>,
    white: Box<dyn Player>,
    current: Disc,
}

impl Game {
    pub fn new(black: Box<dyn Player>, white: Box<dyn Player>) -> Self {
        Game {
            board: Board::new(),
            black,
            white,
            current: Disc::Black,
        }
    }

    pub fn current_disc(&self) -> Disc {
        self.current
    }

    pub fn current_player(&self) -> &dyn Player {
        match self.current_disc() {
            Disc::Black => self.black.as_ref(),
            Disc::White => self.white.as_ref(),
        }
    }

    pub fn available_moves(&self) -> Vec<usize> {
        self.board.valid_moves(self.current).into_iter().collect()
    }

    pub fn forced_pass(&self) -> bool {
        self.available_moves().is_empty()
    }

    fn apply_current(&mut self, choice: usize) -> Result<(), GameError> {
        let legal = self.board.valid_moves(self.current);
        if !legal.contains(&choice) {
            return Err(GameError::InvalidMove);
        }
        self.board
            .apply_move(choice, self.current)
            .map_err(GameError::BoardError)?;
        Ok(())
    }

    fn advance_turn(&mut self) {
        self.current = self.current.opposite();
    }

    pub fn is_over(&self) -> bool {
        self.board.valid_moves(Disc::Black).is_empty() && self.board.valid_moves(Disc::White).is_empty()
    }

    pub fn outcome(&self) -> Option<GameOutcome> {
        if !self.is_over() {
            return None;
        }
        let b = self.board.count_discs(Disc::Black);
        let w = self.board.count_discs(Disc::White);
        match b.cmp(&w) {
            std::cmp::Ordering::Greater => Some(GameOutcome::Winner(Disc::Black)),
            std::cmp::Ordering::Less => Some(GameOutcome::Winner(Disc::White)),
            std::cmp::Ordering::Equal => Some(GameOutcome::Tie),
        }
    }

    pub fn run(&mut self) -> Option<GameOutcome> {
        while !self.is_over() {
            if self.forced_pass() {
                self.advance_turn();
            } else {
                let player = self.current_player();
                let choice = player.select_move(&self.board, self.current);
                if self.board.is_valid_move(choice, self.current) {
                    let _ = self.apply_current(choice);
                    self.advance_turn();
                }
            }
        }
        self.outcome()
    }

    pub fn board(&self) -> &Board {
        &self.board
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::Player;

    struct DummyPlayer;
    impl Player for DummyPlayer {
        fn select_move(&self, _board: &Board, _disc: Disc) -> usize {
            0
        }
    }

    struct ValidPlayer;
    impl Player for ValidPlayer {
        fn select_move(&self, board: &Board, disc: Disc) -> usize {
            board.valid_moves(disc)[0]
        }
    }

    #[test]
    fn test_current_disc_initial_and_after_turn() {
        let mut game = Game::new(
            Box::new(DummyPlayer),
            Box::new(DummyPlayer),
        );
        assert_eq!(game.current_disc(), Disc::Black);
        game.advance_turn();
        assert_eq!(game.current_disc(), Disc::White);
    }

    #[test]
    fn test_forced_pass_detection() {
        let moves = [19, 18, 17, 9, 37, 16, 0, 2];
        let mut game = Game::new(
            Box::new(DummyPlayer),
            Box::new(DummyPlayer),
        );
        for &mv in &moves {
            assert_eq!(game.apply_current(mv), Ok(()));
            game.advance_turn();
        }
        assert!(game.forced_pass());
    }

    #[test]
    fn test_apply_current_legality() {
        let mut game = Game::new(
            Box::new(DummyPlayer),
            Box::new(DummyPlayer),
        );
        assert_eq!(game.apply_current(19), Ok(()));
        assert_eq!(game.board().get_field(19).unwrap(), Some(Disc::Black));
        assert_eq!(game.apply_current(0), Err(GameError::InvalidMove));
    }

    #[test]
    fn test_premature_outcome() {
        let game = Game::new(
            Box::new(DummyPlayer),
            Box::new(DummyPlayer),
        );
        assert_eq!(game.outcome(), None);
    }

    #[test]
    fn test_winner() {
        let moves = [44, 29, 20, 45, 38, 43, 52, 37, 34];
        let mut game = Game::new(
            Box::new(DummyPlayer),
            Box::new(DummyPlayer),
        );
        for &mv in &moves {
            assert_eq!(game.apply_current(mv), Ok(()));
            game.advance_turn();
        }
        assert_eq!(game.outcome(), Some(GameOutcome::Winner(Disc::Black)));
    }

    #[test]
    fn test_tie() {
        let moves = [37, 29, 18, 45, 54, 53, 21, 55, 61, 9, 47, 52, 63, 20, 51, 22, 13, 5, 0, 34];
        let mut game = Game::new(
            Box::new(DummyPlayer),
            Box::new(DummyPlayer),
        );
        for &mv in &moves {
            assert_eq!(game.apply_current(mv), Ok(()));
            game.advance_turn();
        }
        assert_eq!(game.outcome(), Some(GameOutcome::Tie));
    }

    #[test]
    fn test_run_eventually_ends() {
        let mut game = Game::new(
            Box::new(ValidPlayer),
            Box::new(ValidPlayer),
        );
        let winner = game.run();
        assert!(game.is_over());
        assert!(winner.is_some());
        assert!(game.board().valid_moves(Disc::Black).is_empty());
        assert!(game.board().valid_moves(Disc::White).is_empty());
    }
}
