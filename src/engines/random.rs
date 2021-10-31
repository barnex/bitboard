use super::internal::*;

/// Makes random valid moves.
pub struct Random();

impl Engine for Random {
	fn do_move(&self, rng: &mut StdRng, board: &BitBoard, player: Color) -> Option<Move> {
		let moves = board
			.collect_moves(player)
			.into_iter()
			.filter(|&mv| !board.with_move(mv).is_check(player))
			.collect::<SmVec<_>>();

		match moves.len() {
			0 => None,
			_ => Some(moves[rng.gen_range(0..moves.len())]),
		}
	}
}
