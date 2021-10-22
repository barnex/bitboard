use super::internal::*;
use std::convert::TryFrom;
use std::ops::Index;
use std::ops::IndexMut;
use std::str::FromStr;
use Color::*;
use Square::*;

/// A straightforward board implementation used for testing BitBoard.
#[derive(Eq, PartialEq)]
pub struct Mailbox {
	// Layout using 0x88 indexing (https://en.wikipedia.org/wiki/0x88),
	// and fully surrounded by `Offboard` Squares.
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

	pub fn iter<'s>(&'s self) -> impl Iterator<Item = (Pos, Square)> + 's {
		self.board
			.iter()
			.enumerate()
			.map(|(i, piece)| (Pos::from(i), *piece))
			.filter(|(pos, _)| pos.is_valid())
	}

	pub fn moves_for(&self, pos: Pos) -> SmVec<Pos> {
		debug_assert!(pos.is_valid());

		let mut dest = SmVec::new();
		let dst = &mut dest;

		match self[pos] {
			Empty => (),
			WPawn => self.wpawn_moves(dst, pos),
			BPawn => self.bpawn_moves(dst, pos),
			WRook => self.rook_moves(dst, White, pos),
			BRook => self.rook_moves(dst, Black, pos),
			WBisshop => self.bisshop_moves(dst, White, pos),
			BBisshop => self.bisshop_moves(dst, Black, pos),
			WQueen => self.queen_moves(dst, White, pos),
			BQueen => self.queen_moves(dst, Black, pos),
			WKnight => self.wknight_moves(dst, pos),
			BKnight => self.bknight_moves(dst, pos),
			WKing => self.wking_moves(dst, pos),
			BKing => self.bking_moves(dst, pos),
			_ => (),
		}

		dest
	}

	fn wking_moves(&self, dests: &mut SmVec<Pos>, pos: Pos) {
		self.wjump(dests, pos, Self::KING_JUMPS)
	}

	fn bking_moves(&self, dests: &mut SmVec<Pos>, pos: Pos) {
		self.bjump(dests, pos, Self::KING_JUMPS)
	}

	fn wknight_moves(&self, dests: &mut SmVec<Pos>, pos: Pos) {
		self.wjump(dests, pos, Self::KNIGHT_JUMPS)
	}

	fn bknight_moves(&self, dests: &mut SmVec<Pos>, pos: Pos) {
		self.bjump(dests, pos, Self::KNIGHT_JUMPS)
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

	fn wjump<const N: usize>(&self, dests: &mut SmVec<Pos>, pos: Pos, delta: [u8; N]) {
		for delta in delta {
			let pos = pos + delta;
			match self[pos].color() {
				None | Some(Black) => {
					if pos.is_valid() {
						dests.push(pos)
					}
				}
				Some(White) => (),
			}
		}
	}

	fn bjump<const N: usize>(&self, dests: &mut SmVec<Pos>, pos: Pos, delta: [u8; N]) {
		for delta in delta {
			let pos = pos + delta;
			match self[pos].color() {
				None | Some(White) => {
					if pos.is_valid() {
						dests.push(pos)
					}
				}
				Some(Black) => (),
			}
		}
	}

	fn queen_moves(&self, dests: &mut SmVec<Pos>, color: Color, pos: Pos) {
		self.rook_moves(dests, color, pos);
		self.bisshop_moves(dests, color, pos);
	}

	fn bisshop_moves(&self, dests: &mut SmVec<Pos>, color: Color, pos: Pos) {
		self.march(dests, color, pos, NorthEast);
		self.march(dests, color, pos, NorthWest);
		self.march(dests, color, pos, SouthEast);
		self.march(dests, color, pos, SouthWest);
	}

	fn rook_moves(&self, dests: &mut SmVec<Pos>, color: Color, pos: Pos) {
		self.march(dests, color, pos, North);
		self.march(dests, color, pos, East);
		self.march(dests, color, pos, South);
		self.march(dests, color, pos, West);
	}

	fn march(&self, dests: &mut SmVec<Pos>, my_color: Color, pos: Pos, dir: u8) {
		let mut pos = pos;

		for _ in 0..8 {
			pos = pos + dir;
			match self[pos] {
				Empty => dests.push(pos),
				piece => {
					if piece.color() == Some(my_color.opposite()) {
						dests.push(pos);
					}
					return;
				}
			}
		}
	}

	fn wpawn_moves(&self, dests: &mut SmVec<Pos>, pos: Pos) {
		self.pawn_captures(dests, White, pos, delta(1, -1), delta(1, 1));
		self.pawn_pushes(dests, pos, delta(1, 0), 2);
	}

	fn bpawn_moves(&self, dests: &mut SmVec<Pos>, pos: Pos) {
		self.pawn_captures(dests, Black, pos, delta(-1, -1), delta(-1, 1));
		self.pawn_pushes(dests, pos, delta(-1, 0), 5);
	}

	fn pawn_pushes(&self, dests: &mut SmVec<Pos>, pos: Pos, delta: u8, first_row: u8) {
		// one forward
		let pos = pos + delta;
		if self[pos].is_empty() {
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

	fn pawn_captures(&self, dests: &mut SmVec<Pos>, my_color: Color, pos: Pos, left: u8, right: u8) {
		for delta in [left, right] {
			let pos = pos + delta;
			if self[pos].color() == Some(my_color.opposite()) {
				dests.push(pos)
			}
		}
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
