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
	#[structopt(short, long, default_value = "0")]
	pub verbosity: u32,

	#[structopt()]
	pub engines: Vec<String>,
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

	let mut a = parse_engine(&opts.engines[0], opts.seed)?;
	let mut b = parse_engine(&opts.engines[1], opts.seed + 1)?;

	let mut stats = play_match(&opts, &mut [a.as_mut(), b.as_mut()]);

	stats.wins[White.index()].engine_name = opts.engines[0].clone();
	stats.wins[Black.index()].engine_name = opts.engines[1].clone();
	stats.wins[2].engine_name = "draw".into();

	println!("{}", stats);

	Ok(())
}

#[derive(Default)]
struct MatchStats {
	wins: [PlayerStats; 3], // white, black, draw
}

impl MatchStats {
	pub fn add(&mut self, game: &GameStats) {
		let idx = match game.winner {
			Some(player) => player.index(),
			None => 2,
		};
		self.wins[idx].add_win(game);
	}
}

#[derive(Default)]
struct PlayerStats {
	engine_name: String,
	total_wins: u32,
	total_plies: u32,
	final_material: i32,
}

impl PlayerStats {
	fn add_win(&mut self, game: &GameStats) {
		self.total_wins += 1;
		self.total_plies += game.plies;
		self.final_material += material_value(&game.board, White);
	}
	fn avg_plies(&self) -> f32 {
		(self.total_plies as f32) / (self.total_wins as f32)
	}

	fn avg_material(&self) -> f32 {
		(self.final_material as f32) / (self.total_wins as f32)
	}
}

impl fmt::Display for PlayerStats {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{:<16}: {:>4} games, {:>5.1} avg ply, {:>+5.1} avg material",
			self.engine_name,
			self.total_wins,
			self.avg_plies(),
			self.avg_material()
		)?;
		Ok(())
	}
}

impl fmt::Display for MatchStats {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		Ok(for player_stats in &self.wins {
			write!(f, "{}\n", player_stats)?;
		})
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
