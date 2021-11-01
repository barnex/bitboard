use super::internal::*;

pub trait Engine {
	fn eval_moves(&self, board: &Board, player: Color) -> SmVec<(Move, i32)>;
}
