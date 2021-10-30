use bitboard::*;
use std::time::{Duration, Instant};
use structopt::*;

#[derive(StructOpt)]
pub struct Args {
	/// Random seed
	#[structopt(short, long, default_value = "1234567")]
	pub seed: u64,

	/// Maximum number of turns
	#[structopt(short, long, default_value = "70")]
	pub max_turns: u32,

	#[structopt(short, long, default_value = "100")]
	pub num_games: u32,
}

fn main() {
	let args = Args::from_args();

	let mut a = Random::new(args.seed);
	let mut b = Random::new(args.seed + 1);
	compare(&args, &mut a, &mut b)
}

//type Engine = fn(&BitBoard, Color) -> Option<Move>;

fn compare(args: &Args, a: &mut dyn Engine, b: &mut dyn Engine) {}

/*
fn play_game(args: &Args) -> Option<Color> {
	let mut board: BitBoard = starting_position();

	print_ansi(&board, &Set::default());

	let mut total_time = [Duration::ZERO, Duration::ZERO];

	let mut player = White;
	let mut rng = StdRng::seed_from_u64(args.seed);
	for ply in 0..(2 * args.max_turns) {
		println!("Ply {}", ply + 1);

		let start = Instant::now();
		board = match player {
			White => take_turn(&mut rng, board, player, args),
			Black => take_turn(&mut rng, board, player, args),
		};
		total_time[player.index()] += start.elapsed();

		if let Some(winner) = winner(&board) {
			return Some(winner);
		}
		if board.is_check(player) {
			println!("{} checked their self", player);
			return Some(player.opposite());
		}

		player = player.opposite();

		println!(
			"Wall time: White: {}ms, Black: {}ms",
			total_time[White.index()].as_millis(),
			total_time[Black.index()].as_millis()
		);
	}
	None
}

fn take_turn(rng: &mut StdRng, board: BitBoard, player: Color, args: &Args) -> BitBoard {
	let start = Instant::now();

	//let mv_value = evaluate_moves(&board, player, f);
	let depth = match player {
		White => args.w_depth,
		Black => args.b_depth,
	};
	let (mv, value) = alphabeta_(&board, player, &basic_value, -2*INF, 2*INF, depth);

	let elapsed = start.elapsed();
	let mv = mv.expect("at least one valid move");

	println!(
		"{:?} plays {} with value {} in {}ms",
		player,
		annotate_move(&board, mv),
		value,
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
*/
