use bitboard::*;
use std::time::{Duration, Instant};
use structopt::*;

#[derive(StructOpt)]
pub struct Opts {
	/// Random seed
	#[structopt(short, long, default_value = "1234567")]
	pub seed: u64,

	/// Maximum number of turns
	#[structopt(short = "t", long, default_value = "70")]
	pub max_turns: u32,

	/// Number of games to play per match
	#[structopt(short, long, default_value = "100")]
	pub num_games: u32,

	/// Verbosity level
	#[structopt(short, long, default_value = "1")]
	pub verbosity: u32,

	/// Player A
	#[structopt(short = "a", long, default_value = "random")]
	pub engine_a: String,

	/// Player B
	#[structopt(short = "b", long, default_value = "random")]
	pub engine_b: String,
}

impl Opts {
	// is the verbosity at least `level`?
	fn v(&self, level: u32) -> bool {
		self.verbosity >= level
	}
}
fn main() {
	if let Err(e) = main_result() {
		eprintln!("Error {}", e);
		std::process::exit(1);
	}
}

fn main_result() -> Result<()> {
	let opts = Opts::from_args();

	let mut a = parse_engine(&opts.engine_a, opts.seed)?;
	let mut b = parse_engine(&opts.engine_b, opts.seed + 1)?;

	let mut stats = play_match(&opts, &mut [a.as_mut(), b.as_mut()]);
	stats.engine_names = [opts.engine_a.to_owned(), opts.engine_b.to_owned()];
	println!("{}", stats);

	Ok(())
}

fn parse_engine(name: &str, seed: u64) -> Result<Box<dyn Engine>> {
	match name {
		"random" => Ok(Box::new(Random::new(seed))),
		unknown => Err(format_err!("unknown engine: {}", unknown)),
	}
}

#[derive(Default, Debug)]
struct MatchStats {
	engine_names: [String; 2],
	wins: [u32; 2],
	draws: u32,
}

impl MatchStats {
	fn add(&mut self, g: GameStats) {
		match g.winner {
			Some(player) => self.wins[player.index()] += 1,
			None => self.draws += 1,
		}
	}
}

impl fmt::Display for MatchStats {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"wins: A({}): {}, B({}): {}, draws: {}",
			self.engine_names[0], self.wins[0], self.engine_names[1], self.wins[1], self.draws
		)?;

		Ok(())
	}
}

#[derive(Default, Debug)]
struct GameStats {
	winner: Option<Color>,
	plies: u32,
}

fn play_match(opts: &Opts, engines: &mut [&mut dyn Engine; 2]) -> MatchStats {
	let mut match_stats = MatchStats::default();
	for i in 0..opts.num_games {
		let game_stats = play_game(opts, engines);
		if opts.v(2) {
			println!("game {}: {:?}", i, &game_stats)
		}
		match_stats.add(game_stats);
	}
	match_stats
}

fn play_game(opts: &Opts, engines: &mut [&mut dyn Engine; 2]) -> GameStats {
	let mut stats = GameStats::default();
	let mut board: BitBoard = starting_position();

	if opts.v(3) {}

	let mut player = White;

	for ply in 0..(2 * opts.max_turns) {
		if opts.v(3) {
			println!("Turn {} (ply {})", (ply / 2) + 1, ply + 1);
		}

		let mv = match engines[player.index()].do_move(&board, player) {
			None => {
				stats.winner = Some(player.opposite());
				return stats;
			}
			Some(mv) => mv,
		};

		board = board.with_move(mv);

		if let Some(winner) = winner(&board) {
			stats.winner = Some(winner);
			return stats;
		}

		if board.is_check(player) {
			panic!("{} checked their self", player);
		}

		player = player.opposite();
		stats.plies += 1;
	}

	stats
}

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


fn print_options(board: &impl Board, player: Color, mv_value: &[(Move, i32)]) {
	let options = mv_value
		.iter()
		.map(|(mv, value)| format!("{} ({})", annotate_move(board, *mv), value))
		.collect::<Vec<_>>()
		.join(", ");
	println!("{:?} has options {}", player, options);
}
*/

fn winner(board: &impl Board) -> Option<Color> {
	for player in [White, Black] {
		if is_mate(board, player) {
			return Some(player.opposite());
		}
	}
	None
}
