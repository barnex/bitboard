use super::internal::*;

/// Greedily takes material with not lookahead or position value.
pub struct Greedy<F: Fn(&Board, Color) -> i32>(pub F);

impl<F: Fn(&Board, Color) -> i32> Engine for Greedy<F> {
	fn eval_moves(&self, board: &Board, player: Color) -> SmVec<(Move, i32)> {
		board
			.iter_moves(player)
			.map(|mv| (mv, board.with_move(mv)))
			.filter(|(_, board)| !board.is_check(player))
			.map(|(mv, board)| (mv, (self.0)(&board, player)))
			.collect()
	}
}
