pub use super::internal::*;

pub trait Board {
	fn all_moves(&self, player: Color) -> SmVec<Move>;
	fn with_move(&self, mv: Move) -> Self;
	fn at(&self, pos: Pos) -> Square;
	fn is_mate(&self, victim: Color) -> bool;

	fn material_value(&self) -> i32;

	fn is_check(&self, victim: Color) -> bool {
		let attacter = victim.opposite();
		for mv in self.all_moves(attacter) {
			if self.at(mv.to).mask(KIND_MASK) == KING {
				return true;
			}
		}
		false
	}

}
