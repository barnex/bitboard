use std::io;
use std::io::prelude::*;
use std::io::BufRead;
use std::time::Instant;

use bitboard::*;
use structopt::*;

#[derive(StructOpt)]
pub struct Opts {
	/// Random seed
	#[structopt(short, long, default_value = "1234567")]
	pub seed: u64,

	/// Verbosity level
	#[structopt(short, long, default_value = "0")]
	pub verbosity: u32,

	/// Search depth
	#[structopt(short, long, default_value = "4")]
	pub depth: u32,
}

impl Opts {
	// is the verbosity at least `level`?
	fn v(&self, level: u32) -> bool {
		self.verbosity >= level
	}
}
fn main() {
	let opts = Opts::from_args();

	let mut board = Board::starting_position();

	let mut rng = StdRng::seed_from_u64(opts.seed);

	let engine = ParAlphaBeta::new(opts.depth, heuristic1);

	print_ansi(&board, &Set::default());
	loop {
		let mv = play_human(&board, White).expect("White resigns");
		board = board.with_move(mv);
		print_ansi(&board, &[mv.from, mv.to].into_iter().collect());

		if let Some(winner) = winner(&board) {
			println!("{} wins", winner);
			break;
		}

		let start = Instant::now();
		let mv = play_machine(&mut rng, &engine, &board, Black).expect("Black resigns");
		let ms = start.elapsed().as_secs_f32() * 1e3;
		println!("Black> {} ({:.1}ms)", annotate_move(&board, mv), ms);
		board = board.with_move(mv);
		print_ansi(&board, &[mv.from, mv.to].into_iter().collect());

		if let Some(winner) = winner(&board) {
			println!("{} wins", winner);
			break;
		}
	}
}

fn play_human(board: &Board, color: Color) -> Option<Move> {
	loop {
		print!("{}> ", color);
		io::stdout().flush().expect("stdio error");
		let allowed = board //
			.iter_moves(color)
			.collect::<Vec<_>>();

		let mut line = String::new();
		io::stdin().read_line(&mut line).expect("read from stdin");
		let line = line.trim();

		let have = allowed //
			.iter()
			.copied()
			.filter(|mv| mv.to_string().contains(line))
			.collect::<Vec<_>>();

		match &have[..] {
			&[] => println!("invalid move: {}, options: {:?}", &line, &allowed),
			&[mv] => return Some(mv),
			ambigous => println!("ambiguous move: {}, options: {:?}", &line, ambigous),
		}
	}
}

fn play_machine(rng: &mut StdRng, engine: &dyn Engine, board: &Board, color: Color) -> Option<Move> {
	pick_best_with_tiebreak(rng, &engine.eval_moves(board, color))
}
/// Full chess notation of move `mv`. E.g.:
///   p b2c3 xn +
pub fn annotate_move(board: &Board, mv: Move) -> String {
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

fn winner(board: &Board) -> Option<Color> {
	for player in [White, Black] {
		if is_mate(board, player) {
			return Some(player.opposite());
		}
	}
	None
}
