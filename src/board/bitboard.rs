use super::internal::*;
use Square::*;

#[derive(Clone)]
pub struct BitBoard {
	sets: [u64; 13],
}

impl Board for BitBoard {
	fn new() -> Self {
		Self::new()
	}

	fn at(&self, pos: Pos) -> Square {
		self.at(pos)
	}

	fn set(&mut self, pos: Pos, sq: Square) {
		self.set(pos, sq)
	}


	fn all_moves(&self, player: Color) -> SmVec<Move> {
		self.all_moves(player)
	}

	fn with_move(&self, mv: Move) -> Self {
		self.with_move(mv)
	}
}

impl BitBoard {

	fn new() -> Self {
		let mut pieces = [0; 13];
		pieces[Empty.index()] = !0;
		Self { sets: pieces }
	}

	fn at(&self, pos: Pos) -> Square {
		let mask = 1 << pos.index();
		for sq in Square::ALL_SQUARES {
			if self.sets[sq.index()] & mask != 0 {
				return sq;
			}
		}
		unreachable!()
	}

	fn set(&mut self, pos: Pos, sq: Square) {
		debug_assert!(pos.is_valid());
		let pos = pos.index() as u8;
		self.clear(pos);
		self.sets[sq.index()] |= 1 << pos;
	}

	fn with_move(&self, mv: Move) -> Self {
		let mut b = self.clone();
		b.set(mv.to, b.at(mv.from));
		b.set(mv.from, Square::Empty);
		b
	}

	fn all_moves(&self, player: Color) -> SmVec<Move> {
		SmVec::new()
	}

	fn clear(&mut self, pos: u8) {
		let mask = !(1 << pos);
		for sq in Square::ALL_SQUARES {
			self.sets[sq.index()] &= mask;
		}
	}

	#[inline]
	fn bits(&self, sq: Square) -> u64 {
		self.sets[sq.index()]
	}

	/// All white pieces.
	#[inline]
	pub fn white(&self) -> u64 {
		self.bits(WPawn)
			| self.bits(WRook)
			| self.bits(WKnight)
			| self.bits(WBisshop)
			| self.bits(WQueen)
			| self.bits(WKing)
	}

	/// All black pieces.
	#[inline]
	pub fn black(&self) -> u64 {
		self.bits(BPawn)
			| self.bits(BRook)
			| self.bits(BKnight)
			| self.bits(BBisshop)
			| self.bits(BQueen)
			| self.bits(BKing)
	}

	pub fn w_pawn_move(&self) -> u64 {
		self.w_pawn_push() | self.w_pawn_capture()
	}

	pub fn w_pawn_push(&self) -> u64 {
		let empty = self.bits(Empty);
		let pawns = self.bits(WPawn);

		// 1 square forward
		let push1 = sh_n(pawns) & empty;

		// 2 squares forward on first move
		let push2 = sh_n(sh_n(pawns & ROW1) & empty) & empty;

		push1 | push2
	}

	pub fn w_pawn_capture(&self) -> u64 {
		self.w_pawn_capture_ne() | self.w_pawn_capture_nw()
	}

	pub fn w_pawn_capture_ne(&self) -> u64 {
		sh_ne(self.bits(WPawn)) & self.black()
	}

	pub fn w_pawn_capture_nw(&self) -> u64 {
		sh_nw(self.bits(WPawn)) & self.black()
	}
}

const ROW0: u64 = 0b_11111111;
const ROW1: u64 = sh_n(ROW0);

const COL0: u64 = 0b_00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
const COL7: u64 = COL0 << 7;

/// Shift one row north.
#[inline]
const fn sh_n(set: u64) -> u64 {
	set << 8
}

/// Shift one row north east.
#[inline]
const fn sh_ne(set: u64) -> u64 {
	(set & !COL7) << 9
}

/// Shift one row north west.
#[inline]
const fn sh_nw(set: u64) -> u64 {
	(set & !COL0) << 7
}

impl FromStr for BitBoard {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self> {
		parse_board(s)
	}
}

impl fmt::Debug for BitBoard {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for sq in Square::ALL_SQUARES {
			write!(f, "{:?}:\n{}\n", sq, fmt_bits(self.sets[sq.index()]))?;
		}
		Ok(())
	}
}

fn fmt_bits(bits: u64) -> String {
	let mut str = String::with_capacity(2 * 64 + 8);
	for r in (0..8).rev() {
		for c in 0..8 {
			str.push(if bit_at(bits, pos(r, c)) { '1' } else { '0' });
			str.push(' ');
		}
		str.push('\n');
	}
	str
}

pub fn bit_at(set: u64, pos: Pos) -> bool {
	let mask = 1 << pos.index();
	(set & mask) != 0
}
