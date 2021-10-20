use super::internal::*;

/// Row, Column.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Pos(u8);

impl Pos {
    #[inline]
    pub fn new(row: u8, col: u8) -> Self {
        debug_assert!(row < 8 && col < 8);
        Self(row << 3 | col)
    }

    /// Index in row-major (in range 0..64)
    #[inline]
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub const fn row(self) -> u8 {
        self.0 >> 3
    }

    #[inline]
    pub const fn col(self) -> u8 {
        self.0 & 0b111
    }
}

impl From<(u8, u8)> for Pos {
    #[inline]
    fn from((row, col): (u8, u8)) -> Self {
        Self::new(row, col)
    }
}

impl Into<(u8, u8)> for Pos {
    #[inline]
    fn into(self) -> (u8, u8) {
        (self.row(), self.col())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_into() {
        assert_eq!(Pos::from((3, 4)).row(), 3);
        assert_eq!(Pos::from((3, 4)).col(), 4);
    }
}
