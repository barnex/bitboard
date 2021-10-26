use super::internal::*;

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
pub fn parse_board<B: Board>(s: &str) -> Result<B> {
	let mut board = B::new();
	let chars = parse_charboard(s)?;
	for (i, &chr) in chars.iter().enumerate() {
		let piece = Square::try_from(chr)?;
		if i >= 64 {
			return Err(format_err!("line too long"));
		}
		let pos = Pos::from_index(i);
		board.set(pos, piece);
	}
	Ok(board)
}

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
			board[pos(7 - i, j).index()] = chr;
		}
	}
	if max_line != 7 {
		return Err(format_err!("not enough lines"));
	}
	Ok(board)
}

/// Parse positions marked by `x`.
/// (whitespace optional)
///
/// r n b q k b n r
/// p p p p p p p p
/// x . . . . . . .
/// . x . . . . . x
/// . . x . . . x .
/// . . . x P x . .
/// P P P P x P P P
/// R N B Q K B N R
///
pub fn parse_positions(s: &str) -> Set<Pos> {
	parse_charboard(s)
		.unwrap()
		.iter()
		.enumerate()
		.filter(|(_, &chr)| chr == 'x')
		.map(|(i, _)| Pos::from_index(i))
		.collect()
}
