pub use super::internal::*;

// TODO: remove
pub trait Board {
	fn new() -> Self;
	fn at(&self, pos: Pos) -> Square;
	fn set(&mut self, pos: Pos, sq: Square);
	fn collect_moves(&self, player: Color) -> SmVec<Move>;
	fn with_move(&self, mv: Move) -> Self;

	// TODO: optimize for bitboard
	fn has_king(&self, player: Color) -> bool {
		let king = match player {
			Color::White => Square::WKing,
			Color::Black => Square::BKing,
		};
		for r in 0..8 {
			for c in 0..8 {
				if self.at(pos(r, c)) == king {
					return true;
				}
			}
		}
		false
	}

	fn is_check(&self, player: Color) -> bool {
		let attacter = player.opposite();
		for mv in self.collect_moves(attacter) {
			if self.at(mv.to).is_king() {
				return true;
			}
		}
		false
	}
}

/// Check if player is checkmate.
/// This is a slow but general implementation
/// intended to determine the winner of a game,
/// not to be used in a value computation.
pub fn is_mate(board: &impl Board, player: Color) -> bool {
	for mv in board.collect_moves(player) {
		if !board.with_move(mv).is_check(player) {
			return false;
		}
	}
	true
}

pub fn starting_position<B: Board>() -> B {
	use Square::*;
	let mut b = B::new();
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
