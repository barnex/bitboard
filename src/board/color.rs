use super::internal::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
	White = 0,
	Black = 1,
}
use Color::*;

impl Color {
	pub const fn opposite(self) -> Self {
		match self {
			White => Black,
			Black => White,
		}
	}
}
