use super::internal::*;
use std::convert::TryFrom;
use std::ops::Index;
use std::ops::IndexMut;
use std::str::FromStr;
use Square::*;

/// A straightforward board implementation used for testing BitBoard.
#[derive(Eq, PartialEq, Clone)]
pub struct Mailbox {
	// Layout using 0x88 indexing (https://en.wikipedia.org/wiki/0x88),
	// and fully surrounded by `Offboard` Squares so that indexing can never go out of bounds.
	board: [Square; 256],
}

impl Mailbox {
	/// Empty board.
	pub fn new() -> Self {
		let mut board = [OffBoard; 256];
		for i in 0..64 {
			board[Pos::from_index64(i).unwrap().index256()] = Empty;
		}
		Self { board }
	}

	pub fn starting_position() -> Self {
		starting_position::<Self>()
	}

	#[must_use]
	pub fn with_move(&self, mv: Move) -> Self {
		debug_assert!(mv.is_valid());
		let mut b = self.clone();
		b[mv.to] = b[mv.from];
		b[mv.from] = Empty;
		b
	}

	/// TODO: don't iter over offboard squares.
	pub fn iter<'s>(&'s self) -> impl Iterator<Item = (Pos, Square)> + 's {
		self.board
			.iter()
			.enumerate()
			.map(|(i, piece)| (Pos::from(i), *piece))
			.filter(|(pos, _)| pos.is_valid())
	}

	/// TODO: take color, not mask
	pub fn all_moves(&self, player: Color) -> SmVec<Move> {
		let mask = player.mask();
		debug_assert!(mask == WHITE || mask == BLACK);

		let mut moves = SmVec::new();
		for r in 0..8 {
			for c in 0..8 {
				let pos = pos(r, c);
				if self[pos].has_bit(mask) {
					moves.extend(self.dests_for(pos).iter().map(|&dst| Move::new(pos, dst)))
				}
			}
		}
		moves
	}

	pub fn moves_for(&self, pos: Pos) -> SmVec<Move> {
		self.dests_for(pos).into_iter().map(|dst| Move::new(pos, dst)).collect()
	}

	pub fn dests_for(&self, pos: Pos) -> SmVec<Pos> {
		debug_assert!(pos.is_valid());

		let mut dest = SmVec::new();
		let dst = &mut dest;

		match self[pos] {
			Empty => (),
			WPawn => self.w_pawn_moves(dst, pos),
			BPawn => self.b_pawn_moves(dst, pos),
			WRook => self.rook_moves(dst, BLACK, pos),
			BRook => self.rook_moves(dst, WHITE, pos),
			WBisshop => self.bisshop_moves(dst, BLACK, pos),
			BBisshop => self.bisshop_moves(dst, WHITE, pos),
			WQueen => self.queen_moves(dst, BLACK, pos),
			BQueen => self.queen_moves(dst, WHITE, pos),
			WKnight => self.w_knight_moves(dst, pos),
			BKnight => self.b_knight_moves(dst, pos),
			WKing => self.w_king_moves(dst, pos),
			BKing => self.b_king_moves(dst, pos),
			_ => (),
		}

		dest
	}

	fn w_king_moves(&self, dests: &mut SmVec<Pos>, pos: Pos) {
		self.jump(dests, pos, Self::KING_JUMPS, EMPTY | BLACK)
	}

	fn b_king_moves(&self, dests: &mut SmVec<Pos>, pos: Pos) {
		self.jump(dests, pos, Self::KING_JUMPS, EMPTY | WHITE)
	}

	fn w_knight_moves(&self, dests: &mut SmVec<Pos>, pos: Pos) {
		self.jump(dests, pos, Self::KNIGHT_JUMPS, EMPTY | BLACK)
	}

	fn b_knight_moves(&self, dests: &mut SmVec<Pos>, pos: Pos) {
		self.jump(dests, pos, Self::KNIGHT_JUMPS, EMPTY | WHITE)
	}

	fn queen_moves(&self, dests: &mut SmVec<Pos>, allowed: u8, pos: Pos) {
		self.rook_moves(dests, allowed, pos);
		self.bisshop_moves(dests, allowed, pos);
	}

	fn bisshop_moves(&self, dests: &mut SmVec<Pos>, allowed: u8, pos: Pos) {
		self.march(dests, allowed, pos, NORTH_EAST);
		self.march(dests, allowed, pos, NORTH_WEST);
		self.march(dests, allowed, pos, SOUTH_EAST);
		self.march(dests, allowed, pos, SOUTH_WEST);
	}

	fn rook_moves(&self, dests: &mut SmVec<Pos>, allowed: u8, pos: Pos) {
		self.march(dests, allowed, pos, NORTH);
		self.march(dests, allowed, pos, EAST);
		self.march(dests, allowed, pos, SOUTH);
		self.march(dests, allowed, pos, WEST);
	}

	fn w_pawn_moves(&self, dests: &mut SmVec<Pos>, pos: Pos) {
		self.pawn_captures(dests, BLACK, pos, delta(1, -1), delta(1, 1));
		self.pawn_pushes(dests, pos, delta(1, 0), 2);
	}

	fn b_pawn_moves(&self, dests: &mut SmVec<Pos>, pos: Pos) {
		self.pawn_captures(dests, WHITE, pos, delta(-1, -1), delta(-1, 1));
		self.pawn_pushes(dests, pos, delta(-1, 0), 5);
	}

	fn pawn_pushes(&self, dests: &mut SmVec<Pos>, pos: Pos, delta: u8, first_row: u8) {
		// one forward
		let pos = pos + delta;
		if self[pos].has_bit(EMPTY) {
			dests.push(pos);
			// another one forward
			if pos.row() == first_row {
				let pos = pos + delta;
				if self[pos].has_bit(EMPTY) {
					dests.push(pos)
				}
			}
		}
	}

	fn pawn_captures(&self, dests: &mut SmVec<Pos>, allowed: u8, pos: Pos, left: u8, right: u8) {
		for delta in [left, right] {
			let pos = pos + delta;
			if self[pos].opt_color() == allowed {
				dests.push(pos)
			}
		}
	}

	#[inline]
	fn march(&self, dests: &mut SmVec<Pos>, capture_color: u8, pos: Pos, dir: u8) {
		let mut pos = pos;

		for _ in 0..8 {
			pos = pos + dir;
			match self[pos] {
				Empty => dests.push(pos),
				square => {
					if square.opt_color() == capture_color {
						dests.push(pos);
					}
					return;
				}
			}
		}
	}

	#[inline]
	fn jump<const N: usize>(&self, dests: &mut SmVec<Pos>, pos: Pos, delta: [u8; N], allowed: u8) {
		for delta in delta {
			let pos = pos + delta;
			if self[pos].has_bit(allowed) {
				dests.push(pos)
			}
		}
	}

	const KING_JUMPS: [u8; 8] = [
		delta(-1, -1), //
		delta(-1, 0),
		delta(-1, 1),
		delta(0, -1),
		delta(0, 1),
		delta(1, -1),
		delta(1, 0),
		delta(1, 1),
	];

	const KNIGHT_JUMPS: [u8; 8] = [
		delta(-2, -1), //
		delta(-2, 1),
		delta(-1, -2),
		delta(-1, 2),
		delta(2, -1),
		delta(2, 1),
		delta(1, -2),
		delta(1, 2),
	];
}

impl Board for Mailbox {
	fn at(&self, pos: Pos) -> Square {
		self[pos]
	}

	fn all_moves(&self, player: Color) -> SmVec<Move> {
		self.all_moves(player)
	}

	fn with_move(&self, mv: Move) -> Self {
		self.with_move(mv)
	}

	fn set(&mut self, pos: Pos, sq: Square) {
		self[pos] = sq
	}
}

impl Index<Pos> for Mailbox {
	type Output = Square;

	#[inline]
	fn index(&self, pos: Pos) -> &Self::Output {
		&self.board[pos.index256()]
	}
}

impl IndexMut<Pos> for Mailbox {
	#[inline]
	fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
		&mut self.board[pos.index256()]
	}
}

impl FromStr for Mailbox {
	type Err = anyhow::Error;

	/// Parse a chess board from the following notation:
	/// (whitespace optional)
	///
	/// r n b q k b n r
	/// p p p p p p p p
	/// . . . . . . . .
	/// . . . . . . . .
	/// . . . . . . . .
	/// . . . . . . . .
	/// P P P P P P P P
	/// R N B Q K B N R
	///
	fn from_str(s: &str) -> Result<Self> {
		let mut board = Mailbox::new();
		let chars = parse_charboard(s)?;
		for (i, &chr) in chars.iter().enumerate() {
			let piece = Square::try_from(chr)?;
			let pos = Pos::from_index64(i)?;
			board[pos] = piece;
		}
		Ok(board)
	}
}

impl Default for Mailbox {
	fn default() -> Self {
		Self::new()
	}
}

impl fmt::Debug for Mailbox {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&format_board(self.iter().map(|(_, piece)| piece)))
	}
}

impl fmt::Display for Mailbox {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&format_board(self.iter().map(|(_, piece)| piece)))
	}
}
