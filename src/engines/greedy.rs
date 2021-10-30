use std::ptr::eq;

use super::internal::*;

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
		let mut move_value = board
			.all_moves(player)
			.into_iter()
			.map(|mv| (mv, board.with_move(mv)))
			.filter(|(_, board)| !board.is_check(player))
			.map(|(mv, board)| (mv, material(&board, player)))
			.collect::<SmVec<_>>();

		move_value.sort_by_key(|(_, value)| *value);

		let best_value = match move_value.len() {
			0 => return None,
			_ => move_value[0].1,
		};

		let equal_value = move_value.into_iter().filter(|(_, value)| *value == best_value).collect::<SmVec<_>>();

		Some(equal_value[self.rng.gen_range(0..equal_value.len())].0)
	}
}

impl Engine for Greedy {
	fn do_move(&mut self, board: &BitBoard, player: Color) -> Option<Move> {
		self.do_move(board, player)
	}
}
