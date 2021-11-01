use super::internal::*;
use Color::*;
use Square::*;

#[derive(Clone)]
pub struct Board {
	bitfields: [u64; 13],
}

impl Board {
	/// Empty board
	pub fn new() -> Self {
		let mut pieces = [0; 13];
		pieces[Empty.index()] = !0;
		Self { bitfields: pieces }
	}

	pub fn starting_position() -> Self {
		use Square::*;
		let mut b = Self::new();
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

	/// Piece at position.
	/// Slow linear search only intended outputting the board state,
	/// not for evaluation.
	pub fn at(&self, pos: Pos) -> Square {
		let mask = 1 << pos.index();
		for sq in Square::ALL_SQUARES {
			if self.bitfields[sq.index()] & mask != 0 {
				return sq;
			}
		}
		unreachable!()
	}

	/// Bitset for all pieces of type `piece`.
	#[inline]
	pub fn bits(&self, piece: Square) -> u64 {
		self.bitfields[piece.index()]
	}

	/// Set position to piece.
	pub fn set(&mut self, pos: Pos, piece: Square) {
		debug_assert!(pos.is_valid());
		let pos = pos.index() as u8;
		self.clear(pos);
		self.bitfields[piece.index()] |= 1 << pos;
	}

	fn clear(&mut self, pos: u8) {
		let mask = !(1 << pos);
		for sq in Square::ALL_SQUARES {
			self.bitfields[sq.index()] &= mask;
		}
	}

	#[inline]
	pub fn with_move(&self, mv: Move) -> Self {
		let mut b = self.clone();
		debug_assert!(self.at(mv.from) == mv.piece);

		// clear to and from in one go
		let from = 1 << mv.from.index();
		let to = 1 << mv.to.index();
		let clear = !(from | to);

		for sq in Square::ALL_SQUARES {
			b.bitfields[sq.index()] &= clear;
		}

		// set `from` and `to` squares.
		b.bitfields[Empty.index()] |= from;
		b.bitfields[mv.piece.index()] |= to;

		b
	}

	/// All moves for `player`.
	pub fn collect_moves(&self, player: Color) -> SmVec<Move> {
		let mut moves = SmVec::new();
		match player {
			White => self.all_w_moves(&mut moves),
			Black => self.all_b_moves(&mut moves),
		}
		moves
	}

	pub fn iter_moves(&self, player: Color) -> impl Iterator<Item = Move> {
		self.collect_moves(player).into_iter()
	}

	/// All moves for white.
	fn all_w_moves(&self, buf: &mut SmVec<Move>) {
		let white = self.white();

		Self::unpack_pawn(WPawn, self.w_pawn_push1(), delta(-1, 0), buf);
		Self::unpack_pawn(WPawn, self.w_pawn_push2(), delta(-2, 0), buf);
		Self::unpack_pawn(WPawn, self.w_pawn_attack_ne(), delta(-1, -1), buf);
		Self::unpack_pawn(WPawn, self.w_pawn_attack_nw(), delta(-1, 1), buf);

		self.unpack(WKing, |s, b| s.king_moves(b, white), buf);
		self.unpack(WKnight, |s, b| s.knight_moves(b, white), buf);
		self.unpack(WRook, |s, b| s.rook_moves(b, white), buf);
		self.unpack(WBisshop, |s, b| s.bisshop_moves(b, white), buf);
		self.unpack(WQueen, |s, b| s.queen_moves(b, white), buf);
	}

	/// All moves for black.
	#[inline]
	fn all_b_moves(&self, buf: &mut SmVec<Move>) {
		let black = self.black();

		Self::unpack_pawn(BPawn, self.b_pawn_push1(), delta(1, 0), buf);
		Self::unpack_pawn(BPawn, self.b_pawn_push2(), delta(2, 0), buf);
		Self::unpack_pawn(BPawn, self.b_pawn_attack_se(), delta(1, -1), buf);
		Self::unpack_pawn(BPawn, self.b_pawn_attack_sw(), delta(1, 1), buf);

		self.unpack(BKing, |s, b| s.king_moves(b, black), buf);
		self.unpack(BKnight, |s, b| s.knight_moves(b, black), buf);
		self.unpack(BRook, |s, b| s.rook_moves(b, black), buf);
		self.unpack(BBisshop, |s, b| s.bisshop_moves(b, black), buf);
		self.unpack(BQueen, |s, b| s.queen_moves(b, black), buf);
	}

	pub fn attack_vectors(&self) -> AttacVector {
		let bitfields = [
			0, // Square::Empty
			self.w_pawn_attack(),
			self.rook_vector(self.bits(WRook)),
			self.knight_vector(self.bits(WKnight)),
			self.bisshop_vector(self.bits(WBisshop)),
			self.queen_vector(self.bits(WQueen)),
			self.king_vector(self.bits(WKing)),
			self.b_pawn_attack(),
			self.rook_vector(self.bits(BRook)),
			self.knight_vector(self.bits(BKnight)),
			self.bisshop_vector(self.bits(BBisshop)),
			self.queen_vector(self.bits(BQueen)),
			self.king_vector(self.bits(BKing)),
		];
		AttacVector {
			all: [
				bitfields[WPawn.index()]
					| bitfields[WRook.index()]
					| bitfields[WBisshop.index()]
					| bitfields[WQueen.index()]
					| bitfields[WKing.index()],
				bitfields[BPawn.index()]
					| bitfields[BRook.index()]
					| bitfields[BBisshop.index()]
					| bitfields[BQueen.index()]
					| bitfields[BKing.index()],
			],
			bitfields,
		}
	}

	#[inline]
	pub fn attack_vector(&self, player: Color) -> u64 {
		match player {
			White => self.w_attack_vector(),
			Black => self.b_attack_vector(),
		}
	}

	#[inline]
	pub fn w_attack_vector(&self) -> u64 {
		self.king_vector(self.bits(WKing))
			| self.bisshop_vector(self.bits(WQueen) | self.bits(WBisshop))
			| self.rook_vector(self.bits(WQueen) | self.bits(WRook))
			| self.knight_vector(self.bits(WKnight))
			| self.w_pawn_reach()
	}

	#[inline]
	pub fn b_attack_vector(&self) -> u64 {
		self.king_vector(self.bits(BKing))
			| self.bisshop_vector(self.bits(BQueen) | self.bits(BBisshop))
			| self.rook_vector(self.bits(BQueen) | self.bits(BRook))
			| self.knight_vector(self.bits(BKnight))
			| self.b_pawn_reach()
	}

	fn unpack<F>(&self, piece: Square, f: F, buf: &mut SmVec<Move>)
	where
		F: Fn(&Self, u64) -> u64,
	{
		let bits = self.bits(piece);
		for i in iter_bitfield(bits) {
			let from = Pos::from_index(i as usize);
			let bits = 1 << i;
			let moves = (&f)(self, bits);

			for j in iter_bitfield(moves) {
				let to = Pos::from_index(j as usize);
				buf.push(Move::new(piece, from, to))
			}
		}
	}

	#[inline]
	fn unpack_pawn(piece: Square, bits: u64, delta: u8, moves: &mut SmVec<Move>) {
		// TODO: use count trailing zeros
		for i in iter_bitfield(bits) {
			let pos = Pos::from_index(i as usize);
			let from = pos + delta;
			moves.push(Move::new(piece, from, pos));
		}
	}

	#[inline]
	pub fn w_king_moves(&self) -> u64 {
		self.king_moves(self.bits(WKing), self.white())
	}

	#[inline]
	pub fn b_king_moves(&self) -> u64 {
		self.king_moves(self.bits(BKing), self.black())
	}

	#[inline]
	fn king_moves(&self, king: u64, player: u64) -> u64 {
		self.king_vector(king) & !player
	}

	#[inline]
	pub fn king_position(&self, player: Color) -> Pos {
		Pos::from_index(self.bits(player.king()).trailing_zeros() as usize)
	}

	// NOTE: includes self.
	#[inline]
	fn king_vector(&self, king: u64) -> u64 {
		let mut acc = king;
		acc |= sh_n(acc);
		acc |= sh_s(acc);
		acc |= sh_e(acc);
		acc |= sh_w(acc);
		acc
	}

	#[inline]
	pub fn w_knight_moves(&self) -> u64 {
		self.knight_moves(self.bits(WKnight), self.white())
	}

	#[inline]
	pub fn b_knight_moves(&self) -> u64 {
		self.knight_moves(self.bits(BKnight), self.black())
	}

	#[inline]
	fn knight_moves(&self, knights: u64, player: u64) -> u64 {
		self.knight_vector(knights) & !player
	}

	#[inline]
	fn knight_vector(&self, knights: u64) -> u64 {
		let e = sh_e(knights);
		let w = sh_w(knights);
		let ee = sh_e(e);
		let ww = sh_w(w);
		sh_n(sh_n(e | w) | ee | ww) | sh_s(sh_s(e | w) | ee | ww)
	}

	#[inline]
	pub fn queen_moves(&self, queen: u64, player: u64) -> u64 {
		self.queen_vector(queen) & !player
	}

	#[inline]
	pub fn w_bisshop_moves(&self) -> u64 {
		self.bisshop_moves(self.bits(WBisshop), self.white())
	}

	#[inline]
	pub fn b_bisshop_moves(&self) -> u64 {
		self.bisshop_moves(self.bits(BBisshop), self.black())
	}

	#[inline]
	pub fn bisshop_moves(&self, bisshops: u64, player: u64) -> u64 {
		self.bisshop_vector(bisshops) & !player
	}

	#[inline]
	pub fn bisshop_vector(&self, bits: u64) -> u64 {
		self.slide(bits, sh_ne) | self.slide(bits, sh_se) | self.slide(bits, sh_sw) | self.slide(bits, sh_nw)
	}

	#[inline]
	pub fn queen_vector(&self, queen: u64) -> u64 {
		self.bisshop_vector(queen) | self.rook_vector(queen)
	}

	#[inline]
	pub fn w_rook_moves(&self) -> u64 {
		self.rook_moves(self.bits(WRook), self.white())
	}

	#[inline]
	pub fn b_rook_moves(&self) -> u64 {
		self.rook_moves(self.bits(BRook), self.black())
	}

	#[inline]
	fn rook_moves(&self, rooks: u64, player: u64) -> u64 {
		self.rook_vector(rooks) & !player
	}

	#[inline]
	pub fn rook_vector(&self, bits: u64) -> u64 {
		self.slide(bits, sh_n) | self.slide(bits, sh_e) | self.slide(bits, sh_s) | self.slide(bits, sh_w)
	}

	#[inline]
	pub fn w_pawn_move(&self) -> u64 {
		self.w_pawn_push() | self.w_pawn_attack()
	}

	#[inline]
	pub fn b_pawn_move(&self) -> u64 {
		self.b_pawn_push() | self.b_pawn_attack()
	}

	#[inline]
	pub fn w_pawn_push(&self) -> u64 {
		self.w_pawn_push1() | self.w_pawn_push2()
	}

	#[inline]
	pub fn w_pawn_push1(&self) -> u64 {
		sh_n(self.bits(WPawn)) & self.empty()
	}

	#[inline]
	pub fn w_pawn_push2(&self) -> u64 {
		sh_n(sh_n(self.bits(WPawn) & ROW1) & self.empty()) & self.empty()
	}

	#[inline]
	pub fn b_pawn_push(&self) -> u64 {
		self.b_pawn_push1() | self.b_pawn_push2()
	}

	#[inline]
	pub fn b_pawn_push1(&self) -> u64 {
		sh_s(self.bits(BPawn)) & self.empty()
	}

	#[inline]
	pub fn b_pawn_push2(&self) -> u64 {
		sh_s(sh_s(self.bits(BPawn) & ROW6) & self.empty()) & self.empty()
	}

	#[inline]
	pub fn w_pawn_attack(&self) -> u64 {
		self.w_pawn_attack_ne() | self.w_pawn_attack_nw()
	}

	#[inline]
	pub fn b_pawn_attack(&self) -> u64 {
		self.b_pawn_attack_se() | self.b_pawn_attack_sw()
	}

	#[inline]
	pub fn w_pawn_attack_ne(&self) -> u64 {
		sh_ne(self.bits(WPawn)) & self.black()
	}

	#[inline]
	pub fn b_pawn_attack_se(&self) -> u64 {
		sh_se(self.bits(BPawn)) & self.white()
	}

	#[inline]
	pub fn w_pawn_attack_nw(&self) -> u64 {
		sh_nw(self.bits(WPawn)) & self.black()
	}

	#[inline]
	pub fn b_pawn_attack_sw(&self) -> u64 {
		sh_sw(self.bits(BPawn)) & self.white()
	}

	#[inline]
	fn w_pawn_reach(&self) -> u64 {
		let pawns = self.bits(WPawn);
		self.w_pawn_push() | sh_ne(pawns) | sh_nw(pawns)
	}

	#[inline]
	fn b_pawn_reach(&self) -> u64 {
		let pawns = self.bits(BPawn);
		self.b_pawn_push() | sh_se(pawns) | sh_sw(pawns)
	}

	#[inline]
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

	/// All white pieces.
	#[inline]
	pub fn white(&self) -> u64 {
		self.bits(WPawn) | self.bits(WRook) | self.bits(WKnight) | self.bits(WBisshop) | self.bits(WQueen) | self.bits(WKing)
	}

	/// All black pieces.
	#[inline]
	pub fn black(&self) -> u64 {
		self.bits(BPawn) | self.bits(BRook) | self.bits(BKnight) | self.bits(BBisshop) | self.bits(BQueen) | self.bits(BKing)
	}

	#[inline]
	pub fn all_pieces(&self, player: Color) -> u64 {
		match player {
			White => self.white(),
			Black => self.black(),
		}
	}

	/// All empty squares.
	#[inline]
	pub fn empty(&self) -> u64 {
		self.bits(Empty)
	}

	#[inline]
	pub fn piece_count(&self, piece: Square) -> u32 {
		self.bits(piece).count_ones()
	}

	/// Iterate over non-empty positions
	#[inline]
	pub fn iter(bits: u64) -> impl Iterator<Item = Pos> {
		(0..64).filter(move |i| bits & (1 << i) != 0).map(|i| Pos::from_index(i))
	}

	pub fn is_check(&self, player: Color) -> bool {
		(self.attack_vector(player.opposite()) & self.bits(player.king())) != 0
	}

	pub fn has_king(&self, player: Color) -> bool {
		self.bits(player.king()) != 0
	}
}

// ___________________________________________________________ bit fiddling

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

#[inline]
pub fn bit_at(set: u64, pos: Pos) -> bool {
	let mask = 1 << pos.index();
	(set & mask) != 0
}

pub fn iter_bitfield(set: u64) -> impl Iterator<Item = u8> {
	BitfieldIter(set)
}

struct BitfieldIter(u64);

impl Iterator for BitfieldIter {
	type Item = u8;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		if self.0 == 0 {
			return None;
		}
		let i = self.0.trailing_zeros();
		self.0 ^= 1 << i;
		Some(i as u8)
	}
}

// ___________________________________________________________ trait implementations

impl FromStr for Board {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self> {
		parse_board(s)
	}
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut pieces = Vec::new();
		for r in 0..8 {
			for c in 0..8 {
				pieces.push(self.at(pos(r, c)));
			}
		}
		f.write_str(&format_board(pieces.iter()))
	}
}

impl fmt::Debug for Board {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
		for sq in Square::ALL_SQUARES {
			write!(f, "{:?}:\n{}\n", sq, fmt_bits(self.bitfields[sq.index()]))?;
		}
		Ok(())
	}
}

// ___________________________________________________________ util

/// Check if player is checkmate.
/// This is a slow but general implementation
/// intended to determine the winner of a game,
/// not to be used in a value computation.
pub fn is_mate(board: &Board, player: Color) -> bool {
	for mv in board.iter_moves(player) {
		if !board.with_move(mv).is_check(player) {
			return false;
		}
	}
	true
}

// impl Board for BitBoard {
// 	fn new() -> Self {
// 		Self::new()
// 	}
//
// 	fn at(&self, pos: Pos) -> Square {
// 		self.at(pos)
// 	}
//
// 	fn set(&mut self, pos: Pos, sq: Square) {
// 		self.set(pos, sq)
// 	}
//
// 	fn collect_moves(&self, player: Color) -> SmVec<Move> {
// 		self.collect_moves(player)
// 	}
//
// 	fn with_move(&self, mv: Move) -> Self {
// 		self.with_move_(mv)
// 	}
//
// }
