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

pub const EMPTY: u8 = 0b_10_00_000;
const OCC: u8 = 0b_01_00_000;

pub const WHITE: u8 = 0b_00_10_000;
pub const BLACK: u8 = 0b_00_01_000;
const COLOR_MASK: u8 = 0b_00_11_000;

pub const PAWN: u8 = 0;
pub const ROOK: u8 = 1;
pub const KNIGHT: u8 = 2;
pub const BISSHOP: u8 = 3;
pub const QUEEN: u8 = 4;
pub const KING: u8 = 5;
pub const KIND_MASK: u8 = 0b111;

use Square::*;

impl Square {
	pub const ALL: [Square; 12] = [
		WPawn, WRook, WKnight, WBisshop, WQueen, WKing, BPawn, BRook, BKnight, BBisshop, BQueen, BKing,
	];
	const ASCII: [char; 12] = ['P', 'R', 'N', 'B', 'Q', 'K', 'p', 'r', 'n', 'b', 'q', 'k'];
	const UNICODE: [char; 12] = ['♙', '♖', '♘', '♗', '♕', '♔', '♟', '♜', '♞', '♝', '♛', '♚'];

	/// Piece representation following https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation.
	/// `None` is represented as '.', `OffBoard` as `?`.
	pub fn to_char(self) -> char {
		self.into()
	}

	pub fn has_bit(self, mask: u8) -> bool {
		(self as u8) & mask != 0
	}

	pub fn mask(self, mask: u8) -> u8 {
		(self as u8) & mask
	}

	// pub fn is_empty(self) -> bool {
	// 	(self as u8 & OCC) == 0
	// }

	pub fn is_valid(self) -> bool {
		self != Square::OffBoard
	}

	pub fn opt_color(self) -> u8 {
		(self as u8) & COLOR_MASK
	}

	pub fn color(self) -> Option<Color> {
		use Color::*;
		match self.opt_color() {
			WHITE => Some(White),
			BLACK => Some(Black),
			_ => None,
		}
	}

	pub fn is_white(self) -> bool {
		(self as u8 & WHITE) != 0
	}

	pub fn is_black(self) -> bool {
		(self as u8 & BLACK) != 0
	}
	pub fn unicode(self) -> char {
		match self {
			Empty => ' ',
			OffBoard => '?',
			valid => match valid.opt_color() {
				WHITE => Self::UNICODE[(self as u8 & KIND_MASK) as usize],
				BLACK => Self::UNICODE[(self as u8 & KIND_MASK) as usize + 6],
				_ => unreachable!(),
			},
		}
	}

}

impl fmt::Display for Square {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_char(self.to_char())
	}
}

impl Into<char> for Square {
	fn into(self) -> char {
		use Square::*;
		match self {
			Empty => '.',
			OffBoard => '?',
			valid => match valid.opt_color() {
				WHITE => Self::ASCII[(self as u8 & KIND_MASK) as usize],
				BLACK => Self::ASCII[(self as u8 & KIND_MASK) as usize + 6],
				_ => unreachable!(),
			},
		}
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
