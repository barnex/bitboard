use super::internal::*;

/// From a list of (Move, value) pairs, return the move with highest value
/// or None if the list was empty.
/// In case of a tie, pick one at random.
pub fn pick_best_move(rng: &mut StdRng, mv_value: SmVec<(Move, i32)>) -> Option<Move> {
	let mut mv_value = mv_value;
	mv_value.sort_by_key(|(_, v)| -v);

	match mv_value.get(0) {
		None => None,
		Some((_, best_value)) => {
			let equal_value = mv_value.iter().filter(|(_, v)| v == best_value).collect::<Vec<_>>();
			Some(equal_value[rng.gen_range(0..equal_value.len())].0)
		}
	}
}

/// List all `player`'s moves and their value.
/// TODO: pass valuation function.
pub fn evaluate_moves<B, F>(board: &B, player: Color, root_eval: &F) -> SmVec<(Move, i32)>
where
	B: Board,
	F: Fn(&B, Color) -> i32,
{
	board
		.all_moves(player)
		.iter()
		.copied()
		.map(|mv| (mv, board.with_move(mv)))
		.filter(|(_, board)| !board.is_check(player)) // TODO: remove, or assert not check
		//.map(|(mv, board)| (mv, -negamax(&board, player.opposite(), &material, depth)))
		.map(|(mv, board)| (mv, -root_eval(&board, player.opposite())))
		.collect::<SmVec<_>>()
}
