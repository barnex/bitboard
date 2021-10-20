use super::internal::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move {
    from: Pos,
    to: Pos,
}

pub type Moves = SmallVec<Move>;

impl Move {
    #[inline]
    pub fn new(from: Pos, to: Pos) -> Self {
        debug_assert!(from != to);
        Self { from, to }
    }
}

impl From<(Pos, Pos)> for Move {
    fn from((from, to): (Pos, Pos)) -> Self {
        Self::new(from, to)
    }
}
