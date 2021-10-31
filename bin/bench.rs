use bitboard::*;
use std::time::Duration;
use std::time::Instant;
use structopt::*;

#[derive(StructOpt)]
pub struct Opts {
	/// Random seed
	#[structopt(short, long, default_value = "1234567")]
	pub seed: u64,

	/// Player A
	#[structopt(short = "e", long)]
	pub engines: Vec<String>,

	/// Approximate seconds to benchmark for.
	#[structopt(short = "t", long, default_value = "5")]
	pub time: f32,
}

fn main() -> Result<()> {
	let opts = Opts::from_args();
	let boards = random_boards(1024);
	let mut rng = StdRng::seed_from_u64(opts.seed);

	let engines = opts.engines.iter().map(|name| parse_engine(name)).collect::<Result<Vec<_>>>()?;

	for (i, engine) in engines.into_iter().enumerate() {
		let name = &opts.engines[i];
		let bench_time = Duration::from_secs_f32(opts.time);

		let start = Instant::now();
		let mut evals = 0;

		while start.elapsed() < bench_time {
			for board in &boards {
				engine.do_move(&mut rng, board, White);
				engine.do_move(&mut rng, board, Black);
				evals += 2;
			}
		}

		let secs = start.elapsed().as_secs_f32();
		println!(
			"{:<8}:  {:>2.3} s/eval, {:>8.1} evals/s",
			name,
			(secs / evals as f32),
			(evals as f32 / secs)
		);
	}

	Ok(())
}

fn random_boards(n: usize) -> Vec<BitBoard> {
	let seed = 12345;
	let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
	let mut boards = Vec::with_capacity(n);

	for _ in 0..n {
		let mut board = BitBoard::starting_position();

		// remove some random pieces (not kings)
		for _ in 0..rng.gen_range(10..100) {
			let r = rng.gen_range(0..8) as u8;
			let c = rng.gen_range(0..8) as u8;
			let pos = pos(r, c);
			if !board.at(pos).is_king() {
				board.set(pos, Square::Empty)
			}
		}

		for mv in 0..rng.gen_range(0..100) {
			let player = if mv % 2 == 0 { Color::White } else { Color::Black };
			let moves = board.collect_moves(player);
			if moves.len() != 0 {
				let mv = moves[rng.gen_range(0..moves.len())];
				if !board.at(mv.to).is_king() {
					board = board.with_move(mv)
				}
			}
		}

		boards.push(board);
	}

	boards
}
