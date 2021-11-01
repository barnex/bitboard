use super::internal::*;
use rayon::prelude::*;

/// Pick the move with highest `value` for `player`. Breaks ties randomly.
pub fn search_with_tiebreak<F>(rng: &mut StdRng, board: &BitBoard, player: Color, value: F) -> Option<Move>
where
	F: Fn(&BitBoard, Color) -> i32,
{
	// move-value pairs
	let move_value = board
		.collect_moves(player)
		.into_iter()
		.map(|mv| (mv, board.with_move(mv)))
		.filter(|(_, board)| !board.is_check(player))
		.map(|(mv, board)| (mv, value(&board, player)))
		.collect::<SmVec<_>>();

	// highest value, or return None
	let best_value = move_value //
		.iter()
		.map(|(_, value)| *value)
		.max()?;

	// all moves with value equal to best
	let best_moves = move_value //
		.into_iter()
		.filter(|(_, value)| *value == best_value)
		.map(|(mv, _)| mv)
		.collect::<SmVec<_>>();

	// randomly pick from all moves with best value
	let random = rng.gen_range(0..best_moves.len());
	Some(best_moves[random])
}

/// Pick the move with highest `value` for `player`. Breaks ties randomly.
pub fn psearch_with_tiebreak<F>(rng: &mut StdRng, board: &BitBoard, player: Color, value: F) -> Option<Move>
where
	F: Fn(&BitBoard, Color) -> i32 + Sync,
{
	// move-value pairs
	let move_value = board
		.collect_moves(player)
		.into_par_iter()
		.copied()
		.map(|mv| (mv, board.with_move(mv)))
		.filter(|(_, board)| !board.is_check(player))
		.map(|(mv, board)| (mv, value(&board, player)))
		.collect::<Vec<_>>();

	// highest value, or return None
	let best_value = move_value //
		.iter()
		.map(|(_, value)| *value)
		.max()?;

	// all moves with value equal to best
	let best_moves = move_value //
		.into_iter()
		.filter(|(_, value)| *value == best_value)
		.map(|(mv, _)| mv)
		.collect::<SmVec<_>>();

	// randomly pick from all moves with best value
	let random = rng.gen_range(0..best_moves.len());
	Some(best_moves[random])
}
