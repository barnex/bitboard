use super::internal::*;

/// Greedily takes material with not lookahead or position value.
pub struct Greedy {
	rng: StdRng,
}

impl Greedy {
	pub fn new(seed: u64) -> Self {
		Self {
			rng: StdRng::seed_from_u64(seed),
		}
	}

	pub fn do_move(&mut self, board: &BitBoard, player: Color) -> Option<Move> {
		// move-value pairs
		let mut move_value = board
			.collect_moves(player)
			.into_iter()
			.map(|mv| (mv, board.with_move(mv)))
			.filter(|(_, board)| !board.is_check(player))
			.map(|(mv, board)| (mv, material_value(&board, player)))
			.collect::<SmVec<_>>();

		// sort in descending value
		move_value.sort_by_key(|(_, value)| -*value);

		// single-out all moves with equal to best value.
		let best_value = move_value.get(0)?.1;
		let equal_value = move_value //
			.into_iter()
			.filter(|(_, value)| *value == best_value)
			.collect::<SmVec<_>>();

		// randomly pick from all moves with best value
		Some(equal_value[self.rng.gen_range(0..equal_value.len())].0)
	}
}

impl Engine for Greedy {
	fn do_move(&mut self, board: &BitBoard, player: Color) -> Option<Move> {
		self.do_move(board, player)
	}
}
