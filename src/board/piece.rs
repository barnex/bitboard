use std::{convert::TryFrom, fmt::Write};

use super::internal::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    OffBoard = 13,
}

use Piece::*;

impl Piece {
    pub const ALL: [Piece; 12] = [
        WPawn, WRook, WKnight, WBisshop, WQueen, WKing, BPawn, BRook, BKnight, BBisshop, BQueen,
        BKing,
    ];
    const ASCII: [char; 14] = [
        '.', 'P', 'R', 'N', 'B', 'Q', 'K', 'p', 'r', 'n', 'b', 'q', 'k', '?',
    ];
    const UNICODE: [char; 14] = [
        '.', '♙', '♖', '♘', '♗', '♕', '♔', '♟', '♜', '♞', '♝', '♛', '♚', '?',
    ];

    /// Piece representation following https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation.
    /// `None` is represented as '.', `OffBoard` as `?`.
    pub fn to_char(self) -> char {
        self.into()
    }

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn is_empty(self) -> bool {
        self == Piece::Empty
    }

    pub fn is_valid(self) -> bool {
        self != Piece::OffBoard
    }

    pub fn color(self) -> Option<Color> {
        match self as usize {
            0 => None,
            1..=6 => Some(Color::White),
            7..=12 => Some(Color::Black),
            13 => None,
            _ => unreachable!(),
        }
    }

    pub fn is_white(self) -> bool {
        self.color() == Some(Color::White)
    }

    pub fn is_black(self) -> bool {
        self.color() == Some(Color::Black)
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char(self.to_char())
    }
}

impl Into<char> for Piece {
    fn into(self) -> char {
        Piece::ASCII[self as usize]
    }
}

impl TryFrom<char> for Piece {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        use Piece::*;
        Ok(match value {
            '.' => Empty,
            'P' => WPawn,
            'R' => WRook,
            'N' => WKnight,
            'B' => WBisshop,
            'Q' => WQueen,
            'K' => WKing,
            'p' => BPawn,
            'r' => BRook,
            'n' => BKnight,
            'b' => BBisshop,
            'q' => BQueen,
            'k' => BKing,
            '?' => OffBoard,
            invalid => return Err(format_err!("invalid piece: {}", invalid)),
        })
    }
}

impl Default for Piece {
    fn default() -> Self {
        Piece::Empty
    }
}
