use super::internal::*;

/// 1-ply lookahead (i.e greedy search).
pub struct Lookahead1<F>(pub F)
where
	F: Fn(&BitBoard, Color) -> i32;

impl<F> Engine for Lookahead1<F>
where
	F: Fn(&BitBoard, Color) -> i32,
{
	fn evaluate(&self, board: &BitBoard, player: Color) -> i32 {
		(self.0)(board, player)
	}
}
