pub use super::internal::*;

pub trait Board {
	fn all_moves(&self, player: Color) -> SmVec<Move>;
	fn with_move(&self, mv: Move) -> Self;
	fn at(&self, pos: Pos) -> Square;
}

// argh, cannot be a defaoult impl, size of Self.
pub fn is_mate(board: &impl Board, victim: Color) -> bool {
	for mv in board.all_moves(victim) {
		if !is_check(&board.with_move(mv), victim) {
			return false;
		}
	}
	true
}

pub fn is_check(board: &impl Board, victim: Color) -> bool {
	let attacter = victim.opposite();
	for mv in board.all_moves(attacter) {
		if board.at(mv.to).mask(KIND_MASK) == KING {
			return true;
		}
	}
	false
}
