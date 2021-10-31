use super::internal::*;

/// Search without lookahead.
pub struct Lookahead0<F>(pub F)
where
	F: Fn(&BitBoard, Color) -> i32;

impl<F> Engine for Lookahead0<F>
where
	F: Fn(&BitBoard, Color) -> i32,
{
	fn do_move(&self, rng: &mut StdRng, board: &BitBoard, player: Color) -> Option<Move> {
		search(rng, board, player, &self.0)
	}
}
