use super::internal::*;

#[derive(Clone, Default)]
pub struct BitBoard {}

impl Board for BitBoard {
	fn at(&self, pos: Pos) -> Square {
		Square::Empty // TODO
	}

	fn set(&mut self, pos: Pos, sq: Square) {
		// TODO
	}

	fn all_moves(&self, player: Color) -> SmVec<Move> {
		SmVec::new() // TODO
	}

	fn with_move(&self, mv: Move) -> Self {
		self.clone() //TODO
	}
}
