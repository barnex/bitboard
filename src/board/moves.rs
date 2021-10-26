use std::io::Empty;

use super::internal::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Move {
	// TODO: piece
	pub from: Pos,
	pub to: Pos,
}

impl Move {
	#[inline]
	pub fn new(from: Pos, to: Pos) -> Self {
		debug_assert!(from != to);
		Self { from, to }
	}

	pub fn with_piece(piece: Square, from: Pos, to: Pos) -> Self {
		debug_assert!(from != to);
		debug_assert!(!piece.is_empty());
		Self { from, to }
	}

	pub fn is_valid(self) -> bool {
		self.from.is_valid() && self.to.is_valid() && self.from != self.to
	}
}

//impl From<(Pos, Pos)> for Move {
//	fn from((from, to): (Pos, Pos)) -> Self {
//		Self::new(from, to)
//	}
//}

impl fmt::Display for Move {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}{}", self.from, self.to)
	}
}

impl fmt::Debug for Move {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self)
	}
}

impl FromStr for Move {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self> {
		let bytes = s.as_bytes();
		let _piece = Square::try_from(bytes[0] as char)?;
		let from = Pos::try_from(&bytes[1..3])?;
		let to = Pos::try_from(&bytes[3..5])?;
		Ok(Move::new(from, to))
		//let capture = Square::from_str(chars[0])
	}
}

pub struct Moves {
	inner: SmVec<Move>,
}

impl Moves {
	pub fn push(&mut self, mv: Move) {
		self.inner.push(mv)
	}

	pub fn to_set(self) -> Set<Move> {
		self.inner.into_iter().collect()
	}
}
