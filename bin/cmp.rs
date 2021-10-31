use bitboard::*;
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

#[derive(Default, Debug)]
struct MatchStats {
	engine_names: [String; 2],
	wins: [u32; 2],
	draws: u32,
	total_games: u32,
	total_plies: u32,
	total_final_material: i32,
}

impl MatchStats {
	fn add(&mut self, g: &GameStats) {
		match g.winner {
			Some(player) => self.wins[player.index()] += 1,
			None => self.draws += 1,
		}
		self.total_games += 1;
		self.total_plies += g.plies;
		self.total_final_material += material_value(&g.board, White);
	}

	fn avg_plies(&self) -> f32 {
		(self.total_plies as f32) / (self.total_games as f32)
	}

	fn avg_material(&self) -> f32 {
		(self.total_final_material as f32) / (self.total_games as f32)
	}
}

impl fmt::Display for MatchStats {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"wins: A({}): {}, B({}): {}, draws: {}, avg plies: {:.1}, avg material: {:+.1}",
			self.engine_names[0],
			self.wins[0],
			self.engine_names[1],
			self.wins[1],
			self.draws,
			self.avg_plies(),
			self.avg_material(),
		)?;

		Ok(())
	}
}

struct GameStats {
	winner: Option<Color>,
	plies: u32,
	board: BitBoard,
}

fn play_match(opts: &Opts, engines: &mut [&mut dyn Engine; 2]) -> MatchStats {
	let mut match_stats = MatchStats::default();
	for i in 0..opts.num_games {
		let game_stats = play_game(opts, engines);
		match_stats.add(&game_stats);

		if opts.v(1) {
			println!(
				"game {}: {} plies, {} wins, final material: {:+}",
				i,
				game_stats.plies,
				game_stats.winner.map_or("nobody".to_owned(), |c| c.to_string()),
				material_value(&game_stats.board, White)
			)
		}
		if opts.v(2) {
			print_ansi(&game_stats.board, &Set::default())
		}
	}
	match_stats
}

fn play_game(opts: &Opts, engines: &mut [&mut dyn Engine; 2]) -> GameStats {
	let mut board: BitBoard = starting_position();

	let mut player = White;

	let max_plies = 2 * opts.max_turns;
	for ply in 0..=max_plies {
		let mv = match engines[player.index()].do_move(&board, player) {
			None => {
				// player has not valid moves or resigns.
				return GameStats {
					winner: Some(player.opposite()),
					plies: ply,
					board,
				};
			}
			Some(mv) => mv,
		};

		board = board.with_move(mv);

		if let Some(winner) = winner(&board) {
			return GameStats {
				winner: Some(winner),
				board,
				plies: ply,
			};
		}

		if board.is_check(player) {
			panic!("{} checked their self", player);
		}

		player = player.opposite();
	}

	// too many moves
	GameStats {
		winner: None,
		board,
		plies: max_plies,
	}
}

fn winner(board: &impl Board) -> Option<Color> {
	for player in [White, Black] {
		if is_mate(board, player) {
			return Some(player.opposite());
		}
	}
	None
}

fn parse_engine(name: &str, seed: u64) -> Result<Box<dyn Engine>> {
	match name {
		"random" => Ok(Box::new(Random::new(seed))),
		"greedy" => Ok(Box::new(Greedy::new(seed))),
		"greedy_k_dist" => Ok(Box::new(greedy_with_king_dist(seed))),
		unknown => Err(format_err!("unknown engine: {}", unknown)),
	}
}
