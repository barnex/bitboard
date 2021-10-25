use super::internal::*;

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
	for (i, line) in s.lines().map(str::trim).filter(|v| !v.is_empty()).enumerate() {
		if i >= 8 {
			return Err(format_err!("too many lines: {}", line));
		}
		max_line = i;
		let i = i as u8;
		for (j, chr) in line.chars().filter(|chr| !chr.is_whitespace()).enumerate() {
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
