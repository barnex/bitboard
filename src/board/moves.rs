use super::internal::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    from: Pos,
    to: Pos,
}

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

pub struct Moves {
    inner: SmallVec<Move>,
}

impl Moves {
    pub fn push(&mut self, mv: Move) {
        self.inner.push(mv)
    }

    pub fn to_set(self) -> Set<Move> {
        self.inner.into_iter().collect()
    }
}
