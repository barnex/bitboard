use super::internal::*;

pub trait Engine {
	fn evaluate(&self, board: &BitBoard, player: Color) -> i32;

	fn do_move(&self, rng: &mut StdRng, board: &BitBoard, player: Color) -> Option<Move> {
		search_with_tiebreak(rng, board, player, |board, player| self.evaluate(board, player))
	}
}
