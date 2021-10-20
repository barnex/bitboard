use super::internal::*;
use std::ops::Add;

/// Row, Column in 2x3bit format and guaranteed to be a valid board position.
/// For potentially invalid (out-of-board) positions, use `IPos`.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Pos(u8);

/// Shorthand for `Pos::new`.
#[inline]
pub fn pos(row: u8, col: u8) -> Pos {
    Pos::new(row, col)
}

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

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.row(), self.col())
    }
}

impl Add<(i8, i8)> for Pos {
    type Output = Option<Pos>;

    fn add(self, (row, col): (i8, i8)) -> Self::Output {
        let (row, col) = (self.row() as i8 + row, self.col() as i8 + col);
        if ((row | col) & 0b1111000) == 0 {
            Some(Pos::new(row as u8, col as u8))
        } else {
            None
        }
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

impl Into<(i8, i8)> for Pos {
    #[inline]
    fn into(self) -> (i8, i8) {
        (self.row() as i8, self.col() as i8)
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
        assert_eq!(pos(0, 1).index(), 1);
        assert_eq!(pos(1, 0).index(), 8);
        assert_eq!(pos(7, 7).index(), 63);
    }

    #[test]
    fn add() {
        assert_eq!(pos(1, 2) + (3, 4), Some(pos(4, 6)));
        assert_eq!(pos(7, 7) + (1, 0), None);
        assert_eq!(pos(7, 7) + (0, 1), None);
        assert_eq!(pos(1, 1) + (-2, 0), None);
        assert_eq!(pos(1, 0) + (0, -1), None);
    }
}
