use super::internal::*;
use std::convert::TryFrom;
use std::ops::Index;
use std::ops::IndexMut;
use std::str::FromStr;
use Piece::*;

/// A straightforward board implementation used for testing BitBoard.
#[derive(Eq, PartialEq)]
pub struct Mailbox {
    // Layout using 0x88 indexing (https://en.wikipedia.org/wiki/0x88),
    // and fully surrounded by `Offboard` Pieces.
    board: [Piece; 256],
}

impl Mailbox {
    /// Empty board.
    pub fn new() -> Self {
        let mut board = [OffBoard; 256];
        for i in 0..64 {
            board[Pos::from_index64(i).unwrap().index256()] = Empty;
        }
        Self { board }
    }

    pub fn iter<'s>(&'s self) -> impl Iterator<Item = (Pos, Piece)> + 's {
        self.board
            .iter()
            .enumerate()
            .map(|(i, piece)| (Pos::from(i), *piece))
            .filter(|(pos, _)| pos.is_valid())
    }

    pub fn moves_for(&self, pos: Pos) -> SmallVec<Pos> {
        let mut dest = SmallVec::new();

        match self[pos] {
            Piece::Empty => (),
            Piece::WPawn => self.wpawn_moves(&mut dest, pos),
            _ => (),
        }

        dest
    }

    fn wpawn_moves(&self, dests: &mut SmallVec<Pos>, pos: Pos) {
        self.wpawn_captures(dests, pos);
        self.wpawn_pushes(dests, pos);
    }

    fn wpawn_pushes(&self, dests: &mut SmallVec<Pos>, pos: Pos) {
        // one forward
        let pos = pos + delta(1, 0);
        if self[pos].is_empty() {
            dests.push(pos); 
            // another one forward
            if pos.row() == 2 {
                let pos = pos + delta(1, 0);
                if self[pos].is_empty() {
                    dests.push(pos)
                }
            }
        }
    }

    fn wpawn_captures(&self, dests: &mut SmallVec<Pos>, pos: Pos) {
        for i in [-1, 1] {
            let pos = pos + delta(1, i);
            if self[pos].is_black() {
                dests.push(pos)
            }
        }
    }
}

impl Index<Pos> for Mailbox {
    type Output = Piece;

    #[inline]
    fn index(&self, pos: Pos) -> &Self::Output {
        &self.board[pos.index256()]
    }
}

impl IndexMut<Pos> for Mailbox {
    #[inline]
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        &mut self.board[pos.index256()]
    }
}

impl FromStr for Mailbox {
    type Err = anyhow::Error;

    /// Parse a chess board from the following notation:
    /// (whitespace optional)
    ///
    /// r n b q k b n r
    /// p p p p p p p p
    /// . . . . . . . .
    /// . . . . . . . .
    /// . . . . . . . .
    /// . . . . . . . .
    /// P P P P P P P P
    /// R N B Q K B N R
    ///
    fn from_str(s: &str) -> Result<Self> {
        let mut board = Mailbox::new();
        let chars = parse_charboard(s)?;
        for (i, &chr) in chars.iter().enumerate() {
            let piece = Piece::try_from(chr)?;
            let pos = Pos::from_index64(i)?;
            board[pos] = piece;
        }
        Ok(board)
    }
}

impl fmt::Debug for Mailbox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format_board(self.iter().map(|(_, piece)| piece)))
    }
}

impl fmt::Display for Mailbox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format_board(self.iter().map(|(_, piece)| piece)))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    // check the moves for all pieces of given type,
    // by comparing to a stringified board where destinations are marked with `x`.
    fn check_all_moves(piece: Piece, board: &str, want: &str) {
        let board: Mailbox = board.parse().unwrap();
        let mut have = Set::default();
        for (pos, p) in board.iter() {
            if p == piece {
                have.extend(board.moves_for(pos))
            }
        }
        let who = piece.to_string();
        check_moves_(&who, &board, have, want)
    }

    // check the moves for piece at `pos`,
    // by comparing to a stringified board where destinations are marked with `x`.
    fn check_moves(pos: Pos, board: &str, want: &str) {
        let board: Mailbox = board.parse().unwrap();
        let have: Set<Pos> = board.moves_for(pos).iter().copied().collect();
        let who = format!("{} @ {}", board[pos], pos);
        check_moves_(&who, &board, have, want)
    }

    fn check_moves_(who: &str, board: &Mailbox, have: Set<Pos>, want: &str) {
        let want: Set<Pos> = parse_charboard(want)
            .unwrap()
            .iter()
            .enumerate()
            .filter(|(_, &chr)| chr == 'x')
            .map(|(i, _)| Pos::from_index64(i).unwrap())
            .collect();

        if have != want {
            println!(
                "moves for {}\ngot: {}\nwant:{}",
                who,
                mark_moves(&board, have),
                mark_moves(&board, want)
            );
            panic!("test failed")
        }
    }

    // render `board` as text, but mark destinations as `x`.
    fn mark_moves(board: &Mailbox, dests: Set<Pos>) -> String {
        let marks = board.iter().map(|(pos, piece)| {
            if dests.contains(&pos) {
                'x'
            } else {
                piece.to_char()
            }
        });
        format_board(marks)
    }

    #[test]
    fn wpawn_moves() {
        check_moves(
            pos(1, 2),
            r"
. . . . . . . .
. . . . . . . .
. . . . . . . .
. . . . . . . .
. . . . . . . .
. . . r . . . .
. . P . . . . .
. . . . . . . .
",
            r"
. . . . . . . .
. . . . . . . .
. . . . . . . .
. . . . . . . .
. . x . . . . .
. . x x . . . .
. . P . . . . .
. . . . . . . .
",
        );
        check_all_moves(
            WPawn,
            r"
. . . . . . . P
. . . . . . . .
. . . R . . . .
. . . P r . . .
r . . . . P . .
P . r . . . . .
. . P . . . . P
. . . . . . . .
",
            r"
. . . . . . . P
. . . . . . . .
. . . R . . . .
. . . P x x . .
r . . . . P . x
P . r . . . . x
. . P . . . . P
. . . . . . . .
",
        );
    }

    #[test]
    fn set() {
        let mut b = Mailbox::new();
        b[pos(1, 2)] = WPawn;

        assert_eq!(
            b.to_string(),
            r"
8 . . . . . . . .
7 . . . . . . . .
6 . . . . . . . .
5 . . . . . . . .
4 . . . . . . . .
3 . . . . . . . .
2 . . P . . . . .
1 . . . . . . . .
  a b c d e f g h"
        );
    }

    #[test]
    fn from_str() {
        let b = Mailbox::from_str(
            r"
rnbqkbnr
pppppppp
........
........
........
........
PPPPPPPP
RNBQKBNR
",
        )
        .unwrap();

        assert_eq!(b[pos(0, 7)], WRook);
        assert_eq!(b[pos(1, 0)], WPawn);
        assert_eq!(b[pos(7, 3)], BQueen);
    }

    #[test]
    fn from_str_err() {
        let b = Mailbox::from_str(
            r"
rnbqkbnr
pppppppp
........
........
........
PPPPPPPP
RNBQKBNR
",
        );
        assert!(b.is_err())
    }
}
