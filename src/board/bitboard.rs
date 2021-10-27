use super::internal::*;
use Color::*;
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

	fn clear(&mut self, pos: u8) {
		let mask = !(1 << pos);
		for sq in Square::ALL_SQUARES {
			self.sets[sq.index()] &= mask;
		}
	}

	fn with_move(&self, mv: Move) -> Self {
		let mut b = self.clone();
		b.set(mv.to, b.at(mv.from));
		b.set(mv.from, Square::Empty);
		b
	}

	fn all_moves(&self, player: Color) -> SmVec<Move> {
		let mut moves = SmVec::new();
		match player {
			White => self.all_w_moves(&mut moves),
			Black => self.all_b_moves(&mut moves),
		}
		moves
	}

	fn all_w_moves(&self, moves: &mut SmVec<Move>) {
		Self::decode(WPawn, self.w_pawn_push1(), delta(-1, 0), moves);
		Self::decode(WPawn, self.w_pawn_push2(), delta(-2, 0), moves);
		Self::decode(WPawn, self.w_pawn_capture_ne(), delta(-1, -1), moves);
		Self::decode(WPawn, self.w_pawn_capture_nw(), delta(-1, 1), moves);
	}

	fn all_b_moves(&self, moves: &mut SmVec<Move>) {
		Self::decode(BPawn, self.b_pawn_push1(), delta(1, 0), moves);
		Self::decode(BPawn, self.b_pawn_push2(), delta(2, 0), moves);
		Self::decode(BPawn, self.b_pawn_capture_se(), delta(1, -1), moves);
		Self::decode(BPawn, self.b_pawn_capture_sw(), delta(1, 1), moves);
	}

	fn decode(piece: Square, bits: u64, delta: u8, moves: &mut SmVec<Move>) {
		// TODO: use bitscan intrinsic.
		for i in 0..64 {
			let pos = Pos::from_index(i);
			if bit_at(bits, pos) {
				let from = pos + delta;
				moves.push(Move::with_piece(piece, from, pos));
			}
		}
	}

	pub fn w_king_moves(&self) -> u64 {
		self.king_moves(self.bits(WKing), self.white())
	}

	pub fn b_king_moves(&self) -> u64 {
		self.king_moves(self.bits(BKing), self.black())
	}

	fn king_moves(&self, king: u64, player: u64) -> u64 {
		let mut acc = king;
		acc |= sh_n(acc);
		acc |= sh_s(acc);
		acc |= sh_e(acc);
		acc |= sh_w(acc);
		acc & !player
	}

	pub fn w_knight_moves(&self) -> u64 {
		self.knight_moves(self.bits(WKnight), self.white())
	}

	pub fn b_knight_moves(&self) -> u64 {
		self.knight_moves(self.bits(BKnight), self.black())
	}

	fn knight_moves(&self, knights: u64, player: u64) -> u64 {
		let e = sh_e(knights);
		let ee = sh_e(e);

		let w = sh_w(knights);
		let ww = sh_w(w);

		let n = sh_n(e | w);
		let nn = sh_n(n | ee | ww);

		let s = sh_s(e | w);
		let ss = sh_s(s | ee | ww);

		(nn | ss) & !player
	}

	pub fn w_bisshop_moves(&self) -> u64 {
		self.bisshop_reach(self.bits(WBisshop)) & !self.white()
	}

	pub fn b_bisshop_moves(&self) -> u64 {
		self.bisshop_reach(self.bits(BBisshop)) & !self.black()
	}

	pub fn bisshop_reach(&self, bits: u64) -> u64 {
		self.slide(bits, sh_ne) | self.slide(bits, sh_se) | self.slide(bits, sh_sw) | self.slide(bits, sh_nw)
	}

	pub fn w_rook_moves(&self) -> u64 {
		self.rook_reach(self.bits(WRook)) & !self.white()
	}

	pub fn b_rook_moves(&self) -> u64 {
		self.rook_reach(self.bits(BRook)) & !self.black()
	}

	pub fn rook_reach(&self, bits: u64) -> u64 {
		self.slide(bits, sh_n) | self.slide(bits, sh_e) | self.slide(bits, sh_s) | self.slide(bits, sh_w)
	}

	pub fn w_pawn_move(&self) -> u64 {
		self.w_pawn_push() | self.w_pawn_capture()
	}

	pub fn b_pawn_move(&self) -> u64 {
		self.b_pawn_push() | self.b_pawn_capture()
	}

	pub fn w_pawn_push(&self) -> u64 {
		self.w_pawn_push1() | self.w_pawn_push2()
	}

	pub fn w_pawn_push1(&self) -> u64 {
		sh_n(self.bits(WPawn)) & self.empty()
	}

	pub fn w_pawn_push2(&self) -> u64 {
		sh_n(sh_n(self.bits(WPawn) & ROW1) & self.empty()) & self.empty()
	}

	pub fn b_pawn_push(&self) -> u64 {
		self.b_pawn_push1() | self.b_pawn_push2()
	}

	pub fn b_pawn_push1(&self) -> u64 {
		sh_s(self.bits(BPawn)) & self.empty()
	}

	pub fn b_pawn_push2(&self) -> u64 {
		sh_s(sh_s(self.bits(BPawn) & ROW6) & self.empty()) & self.empty()
	}

	pub fn w_pawn_capture(&self) -> u64 {
		self.w_pawn_capture_ne() | self.w_pawn_capture_nw()
	}

	pub fn b_pawn_capture(&self) -> u64 {
		self.b_pawn_capture_se() | self.b_pawn_capture_sw()
	}

	pub fn w_pawn_capture_ne(&self) -> u64 {
		sh_ne(self.bits(WPawn)) & self.black()
	}

	pub fn b_pawn_capture_se(&self) -> u64 {
		sh_se(self.bits(BPawn)) & self.white()
	}

	pub fn w_pawn_capture_nw(&self) -> u64 {
		sh_nw(self.bits(WPawn)) & self.black()
	}

	pub fn b_pawn_capture_sw(&self) -> u64 {
		sh_sw(self.bits(BPawn)) & self.white()
	}

	pub fn slide<F: Fn(u64) -> u64>(&self, bits: u64, sh: F) -> u64 {
		let empty = self.empty();

		let mut cursor = sh(bits);
		let mut acc = cursor;

		// TODO: check if unrolled, unroll manually if needed.
		for _ in 0..6 {
			cursor &= empty;
			cursor = sh(cursor);
			acc |= cursor;
		}

		acc
	}

	#[inline]
	pub fn bits(&self, sq: Square) -> u64 {
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

	#[inline]
	fn empty(&self) -> u64 {
		self.bits(Empty)
	}
}

const ROW0: u64 = 0b_11111111;
const ROW1: u64 = ROW0 << (1 * 8);
const ROW6: u64 = ROW0 << (6 * 8);
const COL0: u64 = 0x_01_01_01_01_01_01_01_01;
const COL7: u64 = COL0 << 7;

/// Shift one row north.
#[inline]
const fn sh_n(set: u64) -> u64 {
	set << 8
}

/// Shift one column east.
#[inline]
pub const fn sh_e(set: u64) -> u64 {
	(set << 1) & !COL0
}

/// Shift one column west.
#[inline]
pub const fn sh_w(set: u64) -> u64 {
	(set >> 1) & !COL7
}

/// Shift one row south.
#[inline]
pub const fn sh_s(set: u64) -> u64 {
	set >> 8
}

/// Shift one row north east.
#[inline]
pub const fn sh_ne(set: u64) -> u64 {
	(set & !COL7) << 9
}

/// Shift one row south east.
#[inline]
pub const fn sh_se(set: u64) -> u64 {
	(set & !COL7) >> 7
}

/// Shift one row south west.
#[inline]
pub const fn sh_sw(set: u64) -> u64 {
	(set & !COL0) >> 9
}

/// Shift one row north west.
#[inline]
pub const fn sh_nw(set: u64) -> u64 {
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
