use super::internal::*;

pub struct NegaMax<F: Fn(&Board, Color) -> i32> {
	depth: u32,
	leaf_value: F,
}

impl<F: Fn(&Board, Color) -> i32> Engine for NegaMax<F> {
	fn eval_moves(&self, board: &Board, player: Color) -> SmVec<(Move, i32)> {
		board
			.iter_moves(player)
			.map(|mv| (mv, board.with_move(mv)))
			.filter(|(_, board)| !board.is_check(player))
			.map(|(mv, board)| (mv, self.negamax(&board, player, self.depth)))
			.collect()
	}
}

impl<F: Fn(&Board, Color) -> i32> NegaMax<F> {
	pub fn new(depth: u32, leaf_value: F) -> Self {
		Self { depth, leaf_value }
	}
	fn negamax(&self, board: &Board, has_played: Color, depth: u32) -> i32 {
		if depth == 0 {
			(self.leaf_value)(board, has_played)
		} else {
			let will_play = has_played.opposite();
			-board
				.iter_moves(will_play)
				.map(|mv| board.with_move(mv))
				.filter(|board| !board.is_check(will_play))
				.map(|board| self.negamax(&board, will_play, depth - 1))
				.max()
				.unwrap_or(-INF)
		}
	}
}
