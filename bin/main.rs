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

	for turn in 0..20 {
		println!("\n=======================turn {}", turn + 1);
		board = play(board, White);
		board = play(board, Black);
	}
}

fn play(board: Mailbox, player: Color) -> Mailbox {
	print_options(&board, player);
	let mv = search(&board, player, 1);
	println!("{:?} plays {}", player, board.annotate_move(mv));
	let board = board.with_move(mv);
	let mark = [mv.from, mv.to].iter().copied().collect();
	print_ansi(&board, &mark);
	println!();
	board
}

fn print_options(board: &Mailbox, player: Color) {
	println!(
		"{:?} has options {}",
		player,
		board
			.all_moves(player.mask())
			.iter()
			.map(|&mv| board.annotate_move(mv))
			.collect::<Vec::<_>>()
			.join(", ")
	);
}
