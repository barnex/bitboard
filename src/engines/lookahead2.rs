use super::internal::*;

/// Play with 2-ply lookahead
/// (i.e.: pick best move with 1-ply lookahead evaluation).
pub struct Lookahead2<F>(pub F)
where
	F: Fn(&BitBoard, Color) -> i32;

impl<F> Lookahead2<F>
where
	F: Fn(&BitBoard, Color) -> i32,
{
	// essentially 1-ply negamax.
	fn evaluate(&self, board: &BitBoard, player: Color) -> i32 {
		let opp = player.opposite();
		-board
			.iter_moves(opp)
			.map(|mv| board.with_move(mv))
			.filter(|board| !board.is_check(opp))
			.map(|board| (self.0)(&board, opp))
			.max()
			.unwrap_or(-INF)
	}
}
impl<F> Engine for Lookahead2<F>
where
	F: Fn(&BitBoard, Color) -> i32,
{
	fn do_move(&self, rng: &mut StdRng, board: &BitBoard, player: Color) -> Option<Move> {
		search_with_tiebreak(rng, board, player, |board, player| self.evaluate(board, player))
	}
}
