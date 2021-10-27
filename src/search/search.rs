use super::internal::*;
use std::time::SystemTime;

pub fn pick_move(mv_value: &[(Move, i32)]) -> Move {
	let best_value = mv_value.get(0).expect("at least one possible move").1;
	let equal_value = mv_value.iter().filter(|(_, v)| *v == best_value).collect::<Vec<_>>();
	let rand = SystemTime::now()
		.duration_since(SystemTime::UNIX_EPOCH)
		.unwrap()
		.as_nanos() as usize;
	equal_value[rand % equal_value.len()].0
}

pub fn evaluate_moves<B>(board: &B, player: Color, depth: u32) -> SmVec<(Move, i32)>
where
	B: Board,
{
	let mut mv_value = board
		.all_moves(player)
		.iter()
		.copied()
		.map(|mv| (mv, board.with_move(mv)))
		.filter(|(_, board)| !board.is_check(player)) // TODO: remove, or assert not check
		.map(|(mv, board)| (mv, -negamax(&board, player.opposite(), &material, depth)))
		.collect::<SmVec<_>>();

	mv_value.sort_by_key(|(_, v)| -v);
	mv_value
}
