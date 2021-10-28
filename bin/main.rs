use std::time::Instant;

use bitboard::*;
use rand::SeedableRng;
use Color::*;


fn main() {
	match play_game() {
		Some(winner) => println!("{} wins", winner),
		None => println!("stalemate"),
	}
}

fn play_game() -> Option<Color> {
	let mut board: BitBoard = starting_position();

	let eval_w = |board: &BitBoard, player| negamax(board, player, &material, 4);
	let eval_b = |board: &BitBoard, player| negamax(board, player, &material, 2); 
	print_ansi(&board, &Set::default());

	let mut player = White;
	let mut rng = StdRng::seed_from_u64(123456);
	for ply in 0..100 {
		println!("Ply {}", ply + 1);

		board = match player {
			White => take_turn(&mut rng, board, player, &eval_w),
			Black => take_turn(&mut rng, board, player, &eval_b),
		};

		if let Some(winner) = winner(&board) {
			return Some(winner);
		}
		if board.is_check(player) {
			println!("{} checked their self", player);
			return Some(player.opposite());
		}

		player = player.opposite();
	}
	None
}

fn take_turn<B, F>(rng: &mut StdRng, board: B, player: Color, f: &F) -> B
where
	B: Board,
	F: Fn(&B, Color) -> i32,
{
	let start = Instant::now();

	let mv_value = evaluate_moves(&board, player, f);

	let elapsed = start.elapsed();
	print_options(&board, player, &mv_value);
	let mv = pick_best_move(rng, mv_value).expect("at least one valid move");

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
	if !board.at(mv.to).is_empty() {
		str += "x";
		str += &board.at(mv.to).to_string().to_ascii_uppercase();
	}

	if let Some(player) = board.at(mv.from).color() {
		// ... checkmate?
		if is_mate(&board.with_move(mv), player.opposite()) {
			str += "#";
		// ...or just check?
		} else if board.with_move(mv).is_check(player.opposite()) {
			str += "+";
		}
	}
	str
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
