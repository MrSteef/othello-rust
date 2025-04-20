#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Disc {
    Black,
    White,
    Empty,
}

impl Disc {
    pub fn opposite(&self) -> Option<Disc> {
        match self {
            Self::Black => Some(Self::White),
            Self::White => Some(Self::Black),
            Self::Empty => None,
        }
    }
}
