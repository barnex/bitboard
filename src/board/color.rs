use super::internal::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
	White,
	Black,
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
}
