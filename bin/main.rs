use std::time::Instant;
use std::time::SystemTime;

use bitboard::*;
use Color::*;

const DEPTH: u32 = 2;

fn main() {
	match play_game() {
		Some(winner) => println!("{} wins", winner),
		None => println!("stalemate"),
	}
}

fn play_game() -> Option<Color> {
	let mut board = Mailbox::starting_position();

	let mut player = White;
	for ply in 0..100 {
		println!("Ply {}", ply + 1);
		board = take_turn(board, player);

		if let Some(winner) = winner(&board) {
			return Some(winner);
		}

		if is_check(&board, player) {
			println!("{} checked their self", player);
			return Some(player.opposite());
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

	let mv = pick_move(&mv_value);

	println!(
		"{:?} plays {} in {}ms",
		player,
		board.annotate_move(mv),
		elapsed.as_millis()
	);
	let board = board.with_move(mv);
	let mark = [mv.from, mv.to].iter().copied().collect();
	print_ansi(&board, &mark);
	println!();
	board
}

fn pick_move(mv_value: &[(Move, i32)]) -> Move {
	let best_value = mv_value.get(0).expect("at least one possible move").1;
	let equal_value = mv_value.iter().filter(|(_, v)| *v == best_value).collect::<Vec<_>>();
	let rand = SystemTime::now()
		.duration_since(SystemTime::UNIX_EPOCH)
		.unwrap()
		.as_nanos() as usize;
	equal_value[rand % equal_value.len()].0
}

fn winner(board: &impl Board) -> Option<Color> {
	for player in [White, Black] {
		if is_mate(board, player) {
			return Some(player.opposite());
		}
	}
	None
}

fn print_options(board: &Mailbox, player: Color, mv_value: &[(Move, i32)]) {
	let options = mv_value
		.iter()
		.map(|(mv, value)| format!("{} ({})", board.annotate_move(*mv), value))
		.collect::<Vec<_>>()
		.join(", ");
	println!("{:?} has options {}", player, options);
}
