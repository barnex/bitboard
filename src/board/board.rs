pub use super::internal::*;

pub trait Board: Default {
	fn at(&self, pos: Pos) -> Square;
	fn set(&mut self, pos: Pos, sq: Square);
	fn all_moves(&self, player: Color) -> SmVec<Move>;
	fn with_move(&self, mv: Move) -> Self;
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

pub fn starting_position<B: Board>() -> B {
	use Square::*;
	let mut b = B::default();
	b.set(pos(0, 0), WRook);
	b.set(pos(0, 1), WKnight);
	b.set(pos(0, 2), WBisshop);
	b.set(pos(0, 3), WKing);
	b.set(pos(0, 4), WQueen);
	b.set(pos(0, 5), WBisshop);
	b.set(pos(0, 6), WKnight);
	b.set(pos(0, 7), WRook);

	for c in 0..8 {
		b.set(pos(1, c), WPawn);
		b.set(pos(6, c), BPawn);
	}

	b.set(pos(7, 0), BRook);
	b.set(pos(7, 1), BKnight);
	b.set(pos(7, 2), BBisshop);
	b.set(pos(7, 3), BKing);
	b.set(pos(7, 4), BQueen);
	b.set(pos(7, 5), BBisshop);
	b.set(pos(7, 6), BKnight);
	b.set(pos(7, 7), BRook);

	b
}
