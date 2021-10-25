use super::internal::*;
use std::{fmt::Write, ops::Add};

/// Board position in 2x4bit format (https://en.wikipedia.org/wiki/0x88).
#[derive(Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Pos(u8);

/// Construct a (valid) position.
#[inline]
pub fn pos(row: u8, col: u8) -> Pos {
	debug_assert!(row < 8 && col < 8);
	Pos::new(row, col)
}

/// Construct a delta, to be added to a position. E.g.:
///   pos(7,5) + delta(-2,0)
pub const fn delta(row: i8, col: i8) -> u8 {
	((row as u8) << 4).wrapping_add(col as u8)
}

pub const NORTH: u8 = delta(1, 0);
pub const NORTH_EAST: u8 = delta(1, 1);
pub const EAST: u8 = delta(0, 1);
pub const SOUTH_EAST: u8 = delta(-1, 1);
pub const SOUTH: u8 = delta(-1, 0);
pub const SOUTH_WEST: u8 = delta(-1, -1);
pub const WEST: u8 = delta(0, -1);
pub const NORTH_WEST: u8 = delta(1, -1);

impl Pos {
	#[inline]
	const fn new(row: u8, col: u8) -> Self {
		Self(row << 4 | col)
	}

	/// Linear index (row-major).
	/// In range 0..256, not necessarily valid.
	#[inline]
	pub const fn index256(self) -> usize {
		self.0 as usize
	}

	/// Linear index (row-major) in range 0..64,
	/// or None for invalid positions.
	pub fn index64(self) -> Option<usize> {
		match self.is_valid() {
			true => Some((self.row() << 3 | self.col()) as usize),
			false => None,
		}
	}

	/// Linear index (row-major) in range 0..64,
	/// assumes position is valid.
	pub fn must_index64(self) -> u8 {
		debug_assert!(self.is_valid());
		(self.row() << 3 | self.col()) as u8
	}

	/// Convert row-major index (in range 0..64) to a position.
	pub fn from_index64(index: usize) -> Result<Self> {
		if index < 64 {
			let row = (index >> 3) as u8;
			let col = (index & 0b111) as u8;
			Ok(pos(row, col))
		} else {
			Err(format_err!("position index out of bounds: {}", index))
		}
	}

	/// Row. In range 0..16, not necessarily valid.
	#[inline]
	pub const fn row(self) -> u8 {
		self.0 >> 4
	}

	/// Column. In range 0..16, not necessarily valid.
	#[inline]
	pub const fn col(self) -> u8 {
		self.0 & 0b1111
	}

	/// Is this position on the board?
	#[inline]
	pub fn is_valid(self) -> bool {
		(self.0 & 0x88) == 0
	}
}

impl fmt::Debug for Pos {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "0x{:02x}", self.0)
	}
}

impl fmt::Display for Pos {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_char(b"abcdefgh"[self.col() as usize] as char)?;
		f.write_char(b"12345678"[self.row() as usize] as char)
	}
}

impl Add<u8> for Pos {
	type Output = Pos;

	fn add(self, rhs: u8) -> Self {
		Self(self.0.wrapping_add(rhs))
	}
}

impl From<usize> for Pos {
	fn from(i: usize) -> Self {
		Self(i as u8)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn row_col() {
		assert_eq!(pos(3, 4).row(), 3);
		assert_eq!(pos(3, 4).col(), 4);
	}

	#[test]
	fn index() {
		assert_eq!(pos(0, 1).index256(), 1);
		assert_eq!(pos(1, 0).index256(), 16);
		assert_eq!(pos(7, 7).index256(), 119);
	}

	#[test]
	fn add() {
		assert_eq!(pos(3, 5) + delta(1, 0), pos(4, 5));
		assert_eq!(pos(2, 6) + delta(0, 1), pos(2, 7));
		assert_eq!(pos(2, 6) + delta(0, -1), pos(2, 5));
		assert_eq!(pos(3, 7) + delta(0, -7), pos(3, 0));
		assert_eq!(pos(2, 6) + delta(-1, 0), pos(1, 6));
		assert_eq!(pos(7, 4) + delta(-7, 0), pos(0, 4));
		assert_eq!(pos(4, 5) + delta(-1, -3), pos(3, 2));
		assert_eq!(pos(7, 7) + delta(-7, -7), pos(0, 0));
	}

	#[test]
	fn is_valid() {
		assert_eq!(pos(0, 0).is_valid(), true);
		assert_eq!(pos(0, 7).is_valid(), true);
		assert_eq!(pos(7, 0).is_valid(), true);
		assert_eq!(pos(7, 7).is_valid(), true);

		assert_eq!((pos(0, 0) + delta(1, 2)).is_valid(), true);
		assert_eq!((pos(0, 0) + delta(-1, 0)).is_valid(), false);
		assert_eq!((pos(0, 0) + delta(0, -1)).is_valid(), false);
		assert_eq!((pos(0, 0) + delta(-1, -1)).is_valid(), false);
		assert_eq!((pos(7, 2) + delta(0, -3)).is_valid(), false);
		assert_eq!((pos(3, 6) + delta(0, 2)).is_valid(), false);
		assert_eq!((pos(4, 2) + delta(4, 0)).is_valid(), false);
	}
}
