use std::str::FromStr;

use bitboard::*;

fn main() {
	let mut board = Mailbox::from_str(
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

	println!("{}", &board);
}
