use super::internal::*;

pub trait Engine {
	fn do_move(&mut self, board: &BitBoard, player: Color) -> Option<Move>;
}
