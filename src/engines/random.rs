use super::internal::*;

pub struct Random {
	rng: StdRng,
}

impl Random {
	pub fn new(seed: u64) -> Self {
		Self {
			rng: StdRng::seed_from_u64(seed),
		}
	}
	pub fn do_move(&mut self, board: &BitBoard, player: Color) -> Option<Move> {
		let moves = board.all_moves(player);
		match moves.len() {
			0 => None,
			_ => Some(moves[self.rng.gen_range(0..moves.len())]),
		}
	}
}

impl Engine for Random {
	fn do_move(&mut self, board: &BitBoard, player: Color) -> Option<Move> {
		self.do_move(board, player)
	}
}
