use super::internal::*;

/// Just returns valid moves, all valuated at 0.
/// Results in random play.
pub struct Valid();

impl Engine for Valid {
	fn eval_moves(&self, board: &Board, player: Color) -> SmVec<(Move, i32)> {
		board
			.iter_moves(player)
			.filter(|&mv| !board.with_move(mv).is_check(player))
			.map(|mv| (mv, 0))
			.collect()
	}
}
