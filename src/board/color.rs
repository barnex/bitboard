use super::internal::*;

#[repr(i8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
	White = 1,
	Black = -1,
}
use Color::*;

impl Color {
	pub fn mask(self) -> u8 {
		match self {
			White => WHITE,
			Black => BLACK,
		}
	}

	pub fn opposite(self) -> Self {
		match self {
			White => Black,
			Black => White,
		}
	}

	pub fn sign(self) -> i32 {
		self as i32
	}
}

impl fmt::Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}
