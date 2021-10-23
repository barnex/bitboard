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

const DEPTH: u32 = 0;

fn play(board: Mailbox, player: Color) -> Mailbox {
	let mv_value = evaluate_moves(&board, player, DEPTH);
	print_options(&board, player, &mv_value);

	let mv = mv_value.get(0).expect("at least one possible move").0;

	println!("{:?} plays {}", player, board.annotate_move(mv));
	let board = board.with_move(mv);
	let mark = [mv.from, mv.to].iter().copied().collect();
	print_ansi(&board, &mark);
	println!();
	board
}

fn print_options(board: &Mailbox, player: Color, mv_value: &[(Move, i32)]) {
	let options = mv_value.iter().map(|(mv, value)| format!("{} ({})", board.annotate_move(*mv), value)).collect::<Vec::<_>>().join(", ");
	println!("{:?} has options {}", player, options);
}
