use super::internal::*;

/// Looks ahead one ply
pub struct Lookahead1<F>(pub F)
where
	F: Fn(&BitBoard, Color) -> i32;

impl<F> Lookahead1<F>
where
	F: Fn(&BitBoard, Color) -> i32,
{
	fn l1_value(&self, board: &BitBoard, player: Color) -> i32 {
		board
			.collect_moves(player)
			.into_iter()
			.map(|mv| board.with_move(mv))
			.filter(|board| !board.is_check(player))
			.map(|board| (self.0)(&board, player))
			.max()
			.unwrap_or(-INF)
	}
}

impl<F> Engine for Lookahead1<F>
where
	F: Fn(&BitBoard, Color) -> i32,
{
	fn do_move(&self, rng: &mut StdRng, board: &BitBoard, player: Color) -> Option<Move> {
		search(rng, board, player, |board, player| self.l1_value(board, player))
	}
}
