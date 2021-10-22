use super::internal::*;
use core::convert::TryFrom;

/// Parse a chess board from the following notation:
///
/// rnbqkbnr
/// pppppppp
/// ........
/// ........
/// ........
/// ........
/// PPPPPPPP
/// RNBQKBNR
pub fn parse_charboard(s: &str) -> Result<[char; 64]> {
    let mut board = [' '; 64];
    let mut max_line = 0;
    for (i, line) in s
        .lines()
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .enumerate()
    {
        if line.len() != 8 {
            return Err(format_err!("wrong line length: {}", line));
        }
        if i >= 8 {
            return Err(format_err!("too many lines: {}", line));
        }
        max_line = i;
        let i = i as u8;
        for (j, chr) in line.chars().enumerate() {
            if j >= 8 {
                return Err(format_err!("col out of range"));
            }
            let j = j as u8;
            board[pos(7 - i, j).index64().unwrap()] = chr;
        }
    }
    if max_line != 7 {
        return Err(format_err!("not enough lines"));
    }
    Ok(board)
}

pub fn format_board<I, T>(el: I) -> String
where
    I: Iterator<Item = T>,
    T: fmt::Display,
{
    let collect: Vec<_> = el.collect();

    let mut str = String::from("\n");
    for r in (0..8).rev() {
        str += &format!("{}", r + 1);
        for c in 0..8 {
            str.push(' ');
            str += &(collect[pos(r, c).index64().unwrap()]).to_string();
        }
        str.push('\n')
    }
    str + "  a b c d e f g h"
}
