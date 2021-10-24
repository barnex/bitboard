pub use super::internal::*;

pub trait Board {
	fn all_moves(&self, player: Color) -> SmVec<Move>;
	fn with_move(&self, mv: Move) -> Self;
	fn at(&self, pos: Pos) -> Square;
}

/// Check if player is checkmate.
/// This is a slow but general implementation
/// intended to determine the winner of a game,
/// not to be used in a value computation.
pub fn is_mate(board: &impl Board, player: Color) -> bool {
	for mv in board.all_moves(player) {
		if !is_check(&board.with_move(mv), player) {
			return false;
		}
	}
	true
}

/// Check if player is check.
/// This is a slow but general implementation
/// intended to annotate moves,
/// not to be used in a value computation.
pub fn is_check(board: &impl Board, player: Color) -> bool {
	let attacter = player.opposite();
	for mv in board.all_moves(attacter) {
		if board.at(mv.to).mask(KIND_MASK) == KING {
			return true;
		}
	}
	false
}
