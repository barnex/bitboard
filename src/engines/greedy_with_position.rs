use std::str::EncodeUtf16;

use super::internal::*;

/// Greedily takes material with not lookahead or position value.
pub struct GreedyWith<F>
where
	F: Fn(&BitBoard, Color) -> i32,
{
	rng: StdRng,
	value: F,
}

/// Greedy, with additional position value for distance to the opponent's king.
pub fn greedy_with_king_dist(seed: u64) -> impl Engine {
	GreedyWith::new(seed, |board, player| {
		//let king = board.king_position(player.opposite());
		//let
		// - dist_to_king;
		1000 * material_value(board, player)
	})
}

impl<F> GreedyWith<F>
where
	F: Fn(&BitBoard, Color) -> i32,
{
	pub fn new(seed: u64, value: F) -> Self {
		Self {
			rng: StdRng::seed_from_u64(seed),
			value,
		}
	}

	pub fn do_move(&mut self, board: &BitBoard, player: Color) -> Option<Move> {
		// move-value pairs
		let mut move_value = board
			.all_moves(player)
			.into_iter()
			.map(|mv| (mv, board.with_move(mv)))
			.filter(|(_, board)| !board.is_check(player))
			.map(|(mv, board)| (mv, (self.value)(&board, player)))
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

impl<F> Engine for GreedyWith<F>
where
	F: Fn(&BitBoard, Color) -> i32,
{
	fn do_move(&mut self, board: &BitBoard, player: Color) -> Option<Move> {
		self.do_move(board, player)
	}
}
