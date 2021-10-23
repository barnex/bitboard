use std::str::FromStr;

use bitboard::*;
use Color::*;

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

	fn play(board: Mailbox, player: Color) -> Mailbox {
		let mv = search(&board, player, 1);
		let piece = board[mv.from];
		let capture = board[mv.to];
		let capture = if capture.is(EMPTY) { "".into() } else { format!(" x {}", capture) };
		println!("\n{:?} plays {} {} {}", player, piece, mv, capture);

		let board = board.with_move(mv);

		print_ansi(&board);

		board
	}

	for turn in 0..20 {
		println!("\n=======================turn {}", turn+1);
		board = play(board, White);
		board = play(board, Black);
	}
}
