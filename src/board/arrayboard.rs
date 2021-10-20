use super::internal::*;

/// A straightforward board implementation used solely for testing BitBoard.
#[derive(Eq, PartialEq)]
pub struct ArrayBoard {
    board: [Piece; 64],
}

impl ArrayBoard {
    /// Empty board.
    pub fn new() -> Self {
        Self {
            board: [Piece::Empty; 64],
        }
    }

    /// Piece at position `pos`. E.g:
    /// 	board.at(Pos(1,2));
    /// 	board.at((1,2));
    pub fn at(&self, pos: Pos) -> Piece {
        self.board[pos.index()]
    }

    pub fn set(&mut self, pos: Pos, piece: Piece) {
        self.board[pos.index()] = piece;
    }

    pub fn moves_for(&self, pos: Pos) -> Moves {
        let mut moves = SmallVec::new();

        match self.at(pos) {
            Piece::Empty => (),
            Piece::WPawn => self.wpawn_moves(&mut moves, pos),
            _ => (),
        }

        moves.into_iter().map(|dest| Move::new(pos, dest)).collect()
    }

    fn wpawn_moves(&self, dest: &mut SmallVec<Pos>, from: Pos) {
        // one row forward
        if let Some(dst) = from + (1, 0) {
            if self.at(dst).is_empty() {
                dest.push(dst)
            }
        }

        // two rows forward
        if let Some(dst) = from + (2, 0) {
            if dst.row() == 3 && self.at(dst).is_empty() {
                dest.push(dst)
            }
        }

        // capture left
        if let Some(dst) = from + (1, -1) {
            if self.at(dst).color() == Some(Color::Black) {
                dest.push(dst)
            }
        }

        // capture right
        if let Some(dst) = from + (1, 1) {
            if self.at(dst).color() == Some(Color::Black) {
                dest.push(dst)
            }
        }
    }
}

impl ToString for ArrayBoard {
    fn to_string(&self) -> String {
        let mut str = String::from("\n");
        for r in (0..8).rev() {
            str += &format!("{}", r + 1);
            for c in 0..8 {
                str.push(' ');
                str.push(Piece::ascii(self.at(pos(r, c))));
            }
            str.push('\n')
        }
        str += "  a b c d e f g h";
        str
    }
}

impl fmt::Debug for ArrayBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Piece::*;

    #[test]
    fn set() {
        let mut b = ArrayBoard::new();
        b.set(pos(1, 2), WPawn);

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
}
