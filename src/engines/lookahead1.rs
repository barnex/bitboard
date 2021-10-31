use super::internal::*;

/// Greedily takes material with not lookahead or position value.
pub struct Lookahead1<F>
where
	F: Fn(&BitBoard, Color) -> i32,
{
	rng: StdRng,
	leaf_value: F,
}

pub fn l1_material(seed: u64) -> impl Engine {
	Lookahead1::new(seed, material_value)
}

impl<F> Lookahead1<F>
where
	F: Fn(&BitBoard, Color) -> i32,
{
	pub fn new(seed: u64, leaf_value: F) -> Self {
		Self {
			rng: StdRng::seed_from_u64(seed),
			leaf_value,
		}
	}

	pub fn do_move(&mut self, board: &BitBoard, player: Color) -> Option<Move> {
		// move-value pairs
		let mut move_value = board
			.all_moves(player)
			.into_iter()
			.map(|mv| (mv, board.with_move(mv)))
			.filter(|(_, board)| !board.is_check(player))
			.map(|(mv, board)| (mv, -(self.l1_value(&board, player.opposite()))))
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

	fn l1_value(&self, board: &BitBoard, player: Color) -> i32 {
		board
			.all_moves(player)
			.into_iter()
			.map(|mv| board.with_move(mv))
			.filter(|board| !board.is_check(player))
			.map(|board| (self.leaf_value)(&board, player))
			.max()
			.unwrap_or(-INF)
	}
}

impl<F> Engine for Lookahead1<F>
where
	F: Fn(&BitBoard, Color) -> i32,
{
	fn do_move(&mut self, board: &BitBoard, player: Color) -> Option<Move> {
		self.do_move(board, player)
	}
}
