use super::internal::*;

pub fn material(b: &impl Board, player: Color) -> i32 {
	player.sign() * material_value(b)
}

// TODO: hide
pub fn material_value(board: &impl Board) -> i32 {
	let mut value = 0;
	for r in 0..8 {
		for c in 0..8 {
			value += piece_value(board.at(pos(r, c)))
		}
	}
	value
}

/// https://en.wikipedia.org/wiki/Chess_piece_relative_value.
pub fn piece_value(sq: Square) -> i32 {
	use Square::*;
	match sq {
		WPawn => 1,
		WKnight => 3,
		WBisshop => 3,
		WRook => 5,
		WQueen => 9,
		WKing => 0,
		BPawn => -1,
		BKnight => -3,
		BBisshop => -3,
		BRook => -5,
		BQueen => -9,
		BKing => 0,
		_ => 0,
	}
}
