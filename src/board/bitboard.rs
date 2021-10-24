use super::internal::*;

pub struct BitBoard {}

impl Board for BitBoard{
    fn all_moves(&self, player: Color) -> SmVec<Move> {
        todo!()
    }

    fn with_move(&self, mv: Move) -> Self {
        todo!()
    }

    fn at(&self, pos: Pos) -> Square {
        todo!()
    }
}