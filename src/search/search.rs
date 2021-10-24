use super::internal::*;

pub fn search(board: &Mailbox, player: Color, depth: u32) -> Move {
	evaluate_moves(board, player, depth).get(0).expect("at least one possible move").0
}

pub fn evaluate_moves(board: &Mailbox, player: Color, depth: u32) -> SmVec<(Move, i32)> {
	let mut mv_value = board
		.all_moves(player)
		.iter()
		.map(|&mv| (mv, player.sign() * negamax(board, depth, player, mv)))
		.collect::<SmVec<_>>();

	mv_value.sort_by_key(|(_, v)| -*v);
	mv_value
}
