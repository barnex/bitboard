use super::internal::*;

/// 1-ply lookahead (i.e greedy search).
pub struct Lookahead1<F>(pub F)
where
	F: Fn(&BitBoard, Color) -> i32;

impl<F> Engine for Lookahead1<F>
where
	F: Fn(&BitBoard, Color) -> i32,
{
	fn do_move(&self, rng: &mut StdRng, board: &BitBoard, player: Color) -> Option<Move> {
		search_with_tiebreak(rng, board, player, |board, player| (self.0)(board, player))
	}
}
