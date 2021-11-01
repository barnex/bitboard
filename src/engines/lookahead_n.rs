use super::internal::*;

/// Play with N-ply lookahead
/// (i.e.: pick best move with N-1 ply lookahead evaluation).
pub struct Lookahead<F>
where
	F: Fn(&BitBoard, Color) -> i32,
{
	leaf_value: F,
	depth: u32,
}

impl<F> Lookahead<F>
where
	F: Fn(&BitBoard, Color) -> i32,
{
	pub fn new(leaf_value: F, depth: u32) -> Self {
		Self { leaf_value, depth }
	}

	fn eval_with_depth(&self, board: &BitBoard, player: Color, depth: u32) -> i32 {
		if depth == 0 {
			(self.leaf_value)(board, player)
		} else {
			let opp = player.opposite();
			-board
				.iter_moves(opp)
				.map(|mv| board.with_move(mv))
				.filter(|board| !board.is_check(opp))
				.map(|board| self.eval_with_depth(&board, opp, depth - 1))
				.max()
				.unwrap_or(-INF)
		}
	}
}

impl<F> Engine for Lookahead<F>
where
	F: Fn(&BitBoard, Color) -> i32,
{
	fn do_move(&self, rng: &mut StdRng, board: &BitBoard, player: Color) -> Option<Move> {
		search_with_tiebreak(rng, board, player, |board, player| self.eval_with_depth(board, player, self.depth))
	}
}
