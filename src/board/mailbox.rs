use super::internal::*;
use std::ops::Index;
use std::ops::IndexMut;
use Color::*;
use Square::*;

/// A straightforward board implementation used for testing BitBoard.
#[derive(Eq, PartialEq, Clone)]
pub struct Mailbox {
	board: [Square; 64],
}

impl Mailbox {
	pub fn new() -> Self {
		Self { board: [Empty; 64] }
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

	pub fn at(&self, pos: Pos) -> Square {
		self[pos]
	}

	pub fn set(&mut self, pos: Pos, sq: Square) {
		self[pos] = sq
	}

	pub fn do_move(&mut self, mv: Move) {
		self[mv.to] = self[mv.from];
		self[mv.from] = Empty;
	}

	pub fn iter<'s>(&'s self) -> impl Iterator<Item = (Pos, Square)> + 's {
		self.board
			.iter()
			.enumerate()
			.map(|(i, piece)| (Pos::from_index(i), *piece))
			.filter(|(pos, _)| pos.is_valid())
	}

	pub fn collect_moves(&self, player: Color) -> SmVec<Move> {
		let mut moves = SmVec::new();
		for r in 0..8 {
			for c in 0..8 {
				let pos = pos(r, c);
				if self[pos].is_color(player) {
					moves.extend(
						self.dests_for(pos) //
							.iter()
							.map(|&dst| Move::new(self.at(pos), pos, dst)),
					)
				}
			}
		}
		moves
	}

	pub fn dests_for(&self, pos: Pos) -> SmVec<Pos> {
		debug_assert!(pos.is_valid());

		let mut dest = SmVec::new();
		let dst = &mut dest;

		match self[pos] {
			Empty => (),
			WPawn => self.w_pawn_moves(dst, pos),
			BPawn => self.b_pawn_moves(dst, pos),
			WRook => self.rook_moves(dst, White, pos),
			BRook => self.rook_moves(dst, Black, pos),
			WBisshop => self.bisshop_moves(dst, White, pos),
			BBisshop => self.bisshop_moves(dst, Black, pos),
			WQueen => self.queen_moves(dst, White, pos),
			BQueen => self.queen_moves(dst, Black, pos),
			WKnight => self.w_knight_moves(dst, pos),
			BKnight => self.b_knight_moves(dst, pos),
			WKing => self.w_king_moves(dst, pos),
			BKing => self.b_king_moves(dst, pos),
		}

		dest
	}

	fn w_king_moves(&self, dests: &mut SmVec<Pos>, pos: Pos) {
		self.jump(dests, pos, Self::KING_JUMPS, White)
	}

	fn b_king_moves(&self, dests: &mut SmVec<Pos>, pos: Pos) {
		self.jump(dests, pos, Self::KING_JUMPS, Black)
	}

	fn w_knight_moves(&self, dests: &mut SmVec<Pos>, pos: Pos) {
		self.jump(dests, pos, Self::KNIGHT_JUMPS, White)
	}

	fn b_knight_moves(&self, dests: &mut SmVec<Pos>, pos: Pos) {
		self.jump(dests, pos, Self::KNIGHT_JUMPS, Black)
	}

	fn queen_moves(&self, dests: &mut SmVec<Pos>, player: Color, pos: Pos) {
		self.rook_moves(dests, player, pos);
		self.bisshop_moves(dests, player, pos);
	}

	fn bisshop_moves(&self, dests: &mut SmVec<Pos>, player: Color, pos: Pos) {
		self.march(dests, player, pos, NORTH_EAST);
		self.march(dests, player, pos, NORTH_WEST);
		self.march(dests, player, pos, SOUTH_EAST);
		self.march(dests, player, pos, SOUTH_WEST);
	}

	fn rook_moves(&self, dests: &mut SmVec<Pos>, player: Color, pos: Pos) {
		self.march(dests, player, pos, NORTH);
		self.march(dests, player, pos, EAST);
		self.march(dests, player, pos, SOUTH);
		self.march(dests, player, pos, WEST);
	}

	fn w_pawn_moves(&self, dests: &mut SmVec<Pos>, pos: Pos) {
		self.pawn_captures(dests, White, pos, delta(1, -1), delta(1, 1));
		self.pawn_pushes(dests, pos, delta(1, 0), 2);
	}

	fn b_pawn_moves(&self, dests: &mut SmVec<Pos>, pos: Pos) {
		self.pawn_captures(dests, Black, pos, delta(-1, -1), delta(-1, 1));
		self.pawn_pushes(dests, pos, delta(-1, 0), 5);
	}

	fn pawn_pushes(&self, dests: &mut SmVec<Pos>, pos: Pos, delta: u8, first_row: u8) {
		// one forward
		let pos = pos + delta;
		if pos.is_valid() && self[pos].is_empty() {
			dests.push(pos);
			// another one forward
			if pos.row() == first_row {
				let pos = pos + delta;
				if self[pos].is_empty() {
					dests.push(pos)
				}
			}
		}
	}

	fn pawn_captures(&self, dests: &mut SmVec<Pos>, player: Color, pos: Pos, left: u8, right: u8) {
		for delta in [left, right] {
			let pos = pos + delta;
			if pos.is_valid() && self[pos].is_color(player.opposite()) {
				dests.push(pos)
			}
		}
	}

	#[inline]
	fn march(&self, dests: &mut SmVec<Pos>, player: Color, pos: Pos, dir: u8) {
		let mut pos = pos + dir;

		while pos.is_valid() {
			match self[pos] {
				Empty => dests.push(pos),
				square => {
					if square.is_color(player.opposite()) {
						dests.push(pos);
					}
					return;
				}
			}
			pos = pos + dir;
		}
	}

	#[inline]
	fn jump<const N: usize>(&self, dests: &mut SmVec<Pos>, pos: Pos, delta: [u8; N], player: Color) {
		for delta in delta {
			let pos = pos + delta;
			if pos.is_valid() && !self[pos].is_color(player) {
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

impl Index<Pos> for Mailbox {
	type Output = Square;

	#[inline]
	fn index(&self, pos: Pos) -> &Self::Output {
		&self.board[pos.index()]
	}
}

impl IndexMut<Pos> for Mailbox {
	#[inline]
	fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
		&mut self.board[pos.index()]
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
