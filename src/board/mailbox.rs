use super::internal::*;
use std::convert::TryFrom;
use std::ops::Index;
use std::ops::IndexMut;
use std::str::FromStr;
use Color::*;
use Piece::*;

/// A straightforward board implementation used for testing BitBoard.
#[derive(Eq, PartialEq)]
pub struct Mailbox {
	// Layout using 0x88 indexing (https://en.wikipedia.org/wiki/0x88),
	// and fully surrounded by `Offboard` Pieces.
	board: [Piece; 256],
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

	pub fn iter<'s>(&'s self) -> impl Iterator<Item = (Pos, Piece)> + 's {
		self.board.iter().enumerate().map(|(i, piece)| (Pos::from(i), *piece)).filter(|(pos, _)| pos.is_valid())
	}

	pub fn moves_for(&self, pos: Pos) -> SmallVec<Pos> {
		let mut dest = SmallVec::new();

		match self[pos] {
			Piece::Empty => (),
			Piece::WPawn => self.wpawn_moves(&mut dest, pos),
			Piece::BPawn => self.bpawn_moves(&mut dest, pos),
			_ => (),
		}

		dest
	}

	fn wpawn_moves(&self, dests: &mut SmallVec<Pos>, pos: Pos) {
		self.pawn_captures(dests, White, pos, delta(1, -1), delta(1, 1));
		self.pawn_pushes(dests, pos, delta(1, 0), 2);
	}

	fn bpawn_moves(&self, dests: &mut SmallVec<Pos>, pos: Pos) {
		self.pawn_captures(dests, Black, pos, delta(-1, -1), delta(-1, 1));
		self.pawn_pushes(dests, pos, delta(-1, 0), 5);
	}

	fn pawn_pushes(&self, dests: &mut SmallVec<Pos>, pos: Pos, delta: u8, first_row: u8) {
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

	fn pawn_captures(&self, dests: &mut SmallVec<Pos>, my_color: Color, pos: Pos, left: u8, right: u8) {
		for delta in [left, right] {
			let pos = pos + delta;
			if self[pos].color() == Some(my_color.opposite()) {
				dests.push(pos)
			}
		}
	}
}

impl Index<Pos> for Mailbox {
	type Output = Piece;

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
			let piece = Piece::try_from(chr)?;
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
