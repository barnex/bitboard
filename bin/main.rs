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
	let mut board: BitBoard = starting_position();

	print_ansi(&board, &Set::default());

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

fn take_turn<B: Board>(board: B, player: Color) -> B {
	let start = Instant::now();
	let mv_value = evaluate_moves(&board, player, DEPTH);
	let elapsed = start.elapsed();

	//print_options(&board, player, &mv_value);

	let mv = pick_move(&mv_value);

	println!(
		"{:?} plays {} in {}ms",
		player,
		annotate_move(&board, mv),
		elapsed.as_millis()
	);
	let board = board.with_move(mv);
	let mark = [mv.from, mv.to].iter().copied().collect();
	print_ansi(&board, &mark);
	println!();
	board
}

/// Full chess notation of move `mv`. E.g.:
///   p b2c3 xn +
pub fn annotate_move(board: &impl Board, mv: Move) -> String {
	// piece...
	let mut str = board.at(mv.from).to_string().to_ascii_uppercase();

	// ...moves to
	str += &mv.to_string();

	// ... captures?
	if !board.at(mv.to).has_bit(EMPTY) {
		str += "x";
		str += &board.at(mv.to).to_string().to_ascii_uppercase();
	}

	if let Some(player) = board.at(mv.from).color() {
		// ... checkmate?
		if is_mate(&board.with_move(mv), player.opposite()) {
			str += "#";
		// ...or just check?
		} else if is_check(&board.with_move(mv), player.opposite()) {
			str += "+";
		}
	}
	str
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

fn print_options(board: &impl Board, player: Color, mv_value: &[(Move, i32)]) {
	let options = mv_value
		.iter()
		.map(|(mv, value)| format!("{} ({})", annotate_move(board, *mv), value))
		.collect::<Vec<_>>()
		.join(", ");
	println!("{:?} has options {}", player, options);
}
