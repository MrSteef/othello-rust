#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Disc {
    Black,
    White
}

impl Disc {
    pub fn opposite(&self) -> Disc {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black
        }
    }
}
