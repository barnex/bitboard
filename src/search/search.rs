use super::internal::*;

//pub fn search(board: &Mailbox, player: Color, depth: u32) -> Move {
//	evaluate_moves(board, player, depth)
//		.get(0)
//		.expect("at least one possible move")
//		.0
//}

pub fn evaluate_moves<B, F>(board: &B, player: Color, depth: u32, val: &F) -> SmVec<(Move, i32)>
where
	B: Board,
	F: Fn(&B) -> i32,
{
	let mut mv_value = board
		.all_moves(player)
		.iter()
		.filter(|&mv| !board.with_move(*mv).is_check(player))
		.map(|&mv| (mv, player.sign() * negamax(board, depth, player, mv, val)))
		.collect::<SmVec<_>>();

	mv_value.sort_by_key(|(_, v)| -v);
	mv_value
}
