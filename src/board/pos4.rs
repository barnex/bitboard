use super::internal::*;

/// Row, Column.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Pos(pub u8, pub u8);

impl Pos {
    /// Index in row-major.
    pub fn index(self) -> usize {
        (self.0 << 3 | self.1) as usize
    }
}

impl From<(u8, u8)> for Pos {
    fn from((row, col): (u8, u8)) -> Self {
        Self(row, col)
    }
}
