use super::internal::*;

pub trait Engine {
	fn do_move(&self, rng: &mut StdRng, board: &BitBoard, player: Color) -> Option<Move>;
}
