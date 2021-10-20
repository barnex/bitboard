use super::internal::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Empty = 0,
    WPawn = 1,
    WRook = 2,
    WKnight = 3,
    WBisshop = 4,
    WQueen = 5,
    WKing = 6,
    BPawn = 7,
    BRook = 8,
    BKnight = 9,
    BBisshop = 10,
    BQueen = 11,
    BKing = 12,
}

use Piece::*;

impl Piece {
    pub const ALL: [Piece; 12] = [
        WPawn, WRook, WKnight, WBisshop, WQueen, WKing, BPawn, BRook, BKnight, BBisshop, BQueen,
        BKing,
    ];
    const ASCII: [char; 13] = [
        '.', 'P', 'R', 'N', 'B', 'Q', 'K', 'p', 'r', 'n', 'b', 'q', 'k',
    ];
    const UNICODE: [char; 13] = [
        '.', '♙', '♖', '♘', '♗', '♕', '♔', '♟', '♜', '♞', '♝', '♛', '♚',
    ];

    /// Piece representation following https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation.
    /// `None` is represented as '.'.
    pub fn ascii(self) -> char {
        Self::ASCII[self.index()]
    }

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn is_empty(self) -> bool {
        self == Piece::Empty
    }

    pub fn color(self) -> Option<Color> {
        match self as usize {
            0 => None,
            1..=6 => Some(Color::White),
            7..=12 => Some(Color::Black),
            _ => unreachable!(),
        }
    }
}

impl Default for Piece {
    fn default() -> Self {
        Piece::Empty
    }
}
