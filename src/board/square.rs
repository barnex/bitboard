use std::{convert::TryFrom, fmt::Write};

use super::internal::*;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Square {
	Empty = EMPTY,
	OffBoard = OCC,

	WPawn = OCC | WHITE | PAWN,
	WRook = OCC | WHITE | ROOK,
	WKnight = OCC | WHITE | KNIGHT,
	WBisshop = OCC | WHITE | BISSHOP,
	WQueen = OCC | WHITE | QUEEN,
	WKing = OCC | WHITE | KING,

	BPawn = OCC | BLACK | PAWN,
	BRook = OCC | BLACK | ROOK,
	BKnight = OCC | BLACK | KNIGHT,
	BBisshop = OCC | BLACK | BISSHOP,
	BQueen = OCC | BLACK | QUEEN,
	BKing = OCC | BLACK | KING,
}

const EMPTY: u8 = 0b_10_00_000;
const OCC: u8 = 0b_01_00_000;

const WHITE: u8 = 0b_00_10_000;
const BLACK: u8 = 0b_00_01_000;
const COLOR_MASK: u8 = 0b_00_11_000;

const PAWN: u8 = 0;
const ROOK: u8 = 1;
const KNIGHT: u8 = 2;
const BISSHOP: u8 = 3;
const QUEEN: u8 = 4;
const KING: u8 = 5;
const KIND_MASK: u8 = 0b111;

use Square::*;

impl Square {
	pub const ALL: [Square; 12] = [
		WPawn, WRook, WKnight, WBisshop, WQueen, WKing, BPawn, BRook, BKnight, BBisshop, BQueen, BKing,
	];
	const ASCII: [char; 14] = ['.', 'P', 'R', 'N', 'B', 'Q', 'K', 'p', 'r', 'n', 'b', 'q', 'k', '?'];
	const UNICODE: [char; 14] = [' ', '♙', '♖', '♘', '♗', '♕', '♔', '♟', '♜', '♞', '♝', '♛', '♚', '?'];

	/// Piece representation following https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation.
	/// `None` is represented as '.', `OffBoard` as `?`.
	pub fn to_char(self) -> char {
		self.into()
	}

	pub fn is_empty(self) -> bool {
		self == Empty
	}

	pub fn is_valid(self) -> bool {
		self != Square::OffBoard
	}

	pub fn color(self) -> Option<Color> {
		use Color::*;
		match self {
			WPawn | WRook | WKnight | WBisshop | WQueen | WKing => Some(White),
			BPawn | BRook | BKnight | BBisshop | BQueen | BKing => Some(Black),
			_ => None,
		}
	}

	pub fn index(self) -> usize {
		match self {
			Empty => 0,
			WPawn => 1,
			WRook => 2,
			WKnight => 3,
			WBisshop => 4,
			WQueen => 5,
			WKing => 6,
			BPawn => 7,
			BRook => 8,
			BKnight => 9,
			BBisshop => 10,
			BQueen => 11,
			BKing => 12,
			OffBoard => 13,
		}
	}

	pub fn is_color(self, color: Color) -> bool {
		self.color() == Some(color)
	}

	pub fn is_king(self) -> bool {
		match self {
			WKing | BKing => true,
			_ => false,
		}
	}

	pub fn unicode(self) -> char {
		Self::UNICODE[self.index()]
	}
}

impl fmt::Display for Square {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_char(self.to_char())
	}
}

impl Into<char> for Square {
	fn into(self) -> char {
		Self::ASCII[self.index()]
	}
}

impl TryFrom<char> for Square {
	type Error = anyhow::Error;

	fn try_from(value: char) -> Result<Self> {
		use Square::*;
		Ok(match value {
			'.' => Empty,
			'P' => WPawn,
			'R' => WRook,
			'N' => WKnight,
			'B' => WBisshop,
			'Q' => WQueen,
			'K' => WKing,
			'p' => BPawn,
			'r' => BRook,
			'n' => BKnight,
			'b' => BBisshop,
			'q' => BQueen,
			'k' => BKing,
			'?' => OffBoard,
			invalid => return Err(format_err!("invalid piece: {}", invalid)),
		})
	}
}

impl Default for Square {
	fn default() -> Self {
		Square::Empty
	}
}
