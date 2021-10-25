use std::io::Empty;

use super::internal::*;

#[derive(Clone, Default)]
pub struct BitBoard {
	pieces: [u64; 12],
}

impl Board for BitBoard {
	fn at(&self, pos: Pos) -> Square {
		let mask = 1 << pos.must_index64();
		for i in 0..self.pieces.len() {
			if self.pieces[i] & mask != 0 {
				return Self::idx2piece(i);
			}
		}
		Square::Empty
	}

	fn set(&mut self, pos: Pos, sq: Square) {
		debug_assert!(pos.is_valid());

		let pos = pos.must_index64();
		self.clear(pos);

		let idx = Self::piece_idx(sq);
		if idx != 255 {
			self.pieces[idx] |= 1 << pos;
		}
	}

	// fn set_i(&mut self, pos: u8, : Square) {
	// 	//let idx = Self::set_idx(sq);
	// 	//if idx != 255{
	// 	//    self.sets[i]
	// 	//}
	// }

	fn all_moves(&self, player: Color) -> SmVec<Move> {
		SmVec::new()
	}

	fn with_move(&self, mv: Move) -> Self {
		let mut b = self.clone();
		b.set(mv.to, b.at(mv.from));
		b.set(mv.from, Square::Empty);
		b
	}
}

impl BitBoard {
	pub fn new() -> Self {
		Self::default()
	}

	fn clear(&mut self, pos: u8) {
		let mask = !(1 << pos);
		for i in 0..self.pieces.len() {
			self.pieces[i] &= mask;
		}
	}

	fn piece_idx(sq: Square) -> usize {
		use Square::*;
		match sq {
			WPawn => 0,
			WRook => 1,
			WKnight => 2,
			WBisshop => 3,
			WQueen => 4,
			WKing => 5,
			BPawn => 6,
			BRook => 7,
			BKnight => 8,
			BBisshop => 9,
			BQueen => 10,
			BKing => 11,
			_ => 255,
		}
	}

	fn idx2piece(idx: usize) -> Square {
		use Square::*;
		match idx {
			0 => WPawn,
			1 => WRook,
			2 => WKnight,
			3 => WBisshop,
			4 => WQueen,
			5 => WKing,
			6 => BPawn,
			7 => BRook,
			8 => BKnight,
			9 => BBisshop,
			10 => BQueen,
			11 => BKing,
			_ => unreachable!(),
		}
	}
}
