use super::internal::*;

/// A straightforward board implementation used solely for testing BitBoard.
#[derive(Eq, PartialEq)]
pub struct ArrayBoard {
    board: [Piece; 64],
}

impl ArrayBoard {
    /// Empty board.
    pub fn new() -> Self {
        Self { board: [Empty; 64] }
    }

    /// Piece at position `pos`. E.g:
    /// 	board.at(Pos(1,2));
    /// 	board.at((1,2));
    pub fn at<P>(&self, pos: P) -> Piece
    where
        P: Into<Pos>,
    {
        let pos: Pos = pos.into();
        self.board[pos.index()]
    }

    pub fn set<P>(&mut self, pos: P, piece: Piece)
    where
        P: Into<Pos>,
    {
        let pos: Pos = pos.into();
        self.board[pos.index()] = piece;
    }
}

impl ToString for ArrayBoard {
    fn to_string(&self) -> String {
        let mut str = String::from("\n");
        for r in (0..8).rev() {
            str += &format!("{}", r + 1);
            for c in 0..8 {
                str.push(' ');
                str.push(Piece::ascii(self.at((r, c))));
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

    #[test]
    fn set() {
        let mut b = ArrayBoard::new();
        b.set((1, 2), WPawn);

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
