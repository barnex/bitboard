use std::time::Instant;

use bitboard::*;
use Color::*;

const DEPTH: u32 = 4;

fn main() {
	match play_game(){
		Some(winner) => println!("{} wins", winner),
		None=>println!("stalemate"),
	}
}


fn play_game() -> Option<Color>{
	let mut board = Mailbox::starting_position();

	let mut player = White;
	for ply in 0..100 {
		println!("Ply {}", ply + 1);
		board = take_turn(board, player);

		if let Some(winner) = winner(&board){
			return Some(winner)
		}

		if board.is_check(player){
			println!("{} checked their self", player);
			return Some(player.opposite())
		}

		player = player.opposite();
	}
	None
}

fn take_turn(board: Mailbox, player: Color) -> Mailbox {

	let start = Instant::now();
	let mv_value = evaluate_moves(&board, player, DEPTH);
	let elapsed = start.elapsed();

	print_options(&board, player, &mv_value);

	let mv = mv_value.get(0).expect("at least one possible move").0;

	println!("{:?} plays {} in {}ms", player, board.annotate_move(mv), elapsed.as_millis());
	let board = board.with_move(mv);
	let mark = [mv.from, mv.to].iter().copied().collect();
	print_ansi(&board, &mark);
	println!();
	board
}

fn winner(board: &Mailbox)-> Option<Color>{
	use Square::*;
	let w_king = board.iter().find(|(_,p)|*p==WKing).is_some();
	let b_king = board.iter().find(|(_,p)|*p==BKing).is_some();
	match (w_king, b_king) {
		(true, true)	 => None,
		(true, false)	 => Some(White),
		(false, true)	 => Some(Black),
		(false, false)	 => unreachable!()
	}
}

fn print_options(board: &Mailbox, player: Color, mv_value: &[(Move, i32)]) {
	let options = mv_value.iter().map(|(mv, value)| format!("{} ({})", board.annotate_move(*mv), value)).collect::<Vec::<_>>().join(", ");
	println!("{:?} has options {}", player, options);
}
