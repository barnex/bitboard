use super::internal::*;
use std::ops::Add;

pub fn material_value_white(board: &Mailbox) -> i32 {
	board
		.iter()
		.map(|(_, sq)| piece_value(sq))
		.map(|(w, b)| w - b)
		.reduce(i32::add)
		.unwrap_or(0)
}

/// https://en.wikipedia.org/wiki/Chess_piece_relative_value.
pub fn piece_value(piece: Square) -> (i32, i32) {
	use Square::*;
	match piece {
		WPawn => (1, 0),
		WKnight => (3, 0),
		WBisshop => (3, 0),
		WRook => (5, 0),
		WQueen => (9, 0),
		WKing => (1000, 0), // TODO: remove

		BPawn => (0, 1),
		BKnight => (0, 3),
		BBisshop => (0, 3),
		BRook => (0, 5),
		BQueen => (0, 9),
		BKing => (0, 1000), // TODO: remove

		_ => (0, 0),
	}
}
