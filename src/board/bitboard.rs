use super::internal::*;

#[derive(Clone, Default)]
pub struct BitBoard {
	pieces: [u64; 13],
}

impl Board for BitBoard {
	fn at(&self, pos: Pos) -> Square {
		let mask = 1 << pos.must_index64();
		for i in 0..self.pieces.len() {
			if self.pieces[i] & mask != 0 {
				return Self::idx2piece(i);
			}
		}
		unreachable!()
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

const I_EMPTY: usize = 0;
const W_PAWN: usize = 1;
const W_ROOK: usize = 2;
const W_KNIGHT: usize = 3;
const W_BISSHOP: usize = 4;
const W_QUEEN: usize = 5;
const W_KING: usize = 6;
const B_PAWN: usize = 7;
const B_ROOK: usize = 8;
const B_KNIGHT: usize = 9;
const B_BISSHOP: usize = 10;
const B_QUEEN: usize = 11;
const B_KING: usize = 12;

impl BitBoard {
	pub fn new() -> Self {
		Self::default()
	}

	fn w_pawn_pushes(&self) -> u64 {
		//let pawns = self.pieces[W_PAWN_I];
		//let push1 = (pawns << 8 & self.pieces[EMPTY_I]);

		//push1
		0
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
			Empty => I_EMPTY,
			WPawn => W_PAWN,
			WRook => W_ROOK,
			WKnight => W_KNIGHT,
			WBisshop => W_BISSHOP,
			WQueen => W_QUEEN,
			WKing => W_KING,
			BPawn => B_PAWN,
			BRook => B_ROOK,
			BKnight => B_KNIGHT,
			BBisshop => B_BISSHOP,
			BQueen => B_QUEEN,
			BKing => B_KING,
			_ => unreachable!(),
		}
	}

	fn idx2piece(idx: usize) -> Square {
		use Square::*;
		match idx {
			I_EMPTY => Empty,
			W_PAWN => WPawn,
			W_ROOK => WRook,
			W_KNIGHT => WKnight,
			W_BISSHOP => WBisshop,
			W_QUEEN => WQueen,
			W_KING => WKing,
			B_PAWN => BPawn,
			B_ROOK => BRook,
			B_KNIGHT => BKnight,
			B_BISSHOP => BBisshop,
			B_QUEEN => BQueen,
			B_KING => BKing,
			_ => unreachable!(),
		}
	}
}
