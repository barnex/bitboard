use super::internal::*;

pub fn material(board: &BitBoard, player: Color) -> i32 {
	Square::ALL_PIECES //
		.into_iter()
		.map(|p| p.value() * (board.piece_count(p) as i32))
		.sum::<i32>()
		* player.sign()
}

pub fn material_and<F>(f: F) -> impl Fn(&BitBoard, Color) -> i32
where
	F: Fn(&BitBoard, Color) -> i32 + 'static,
{
	move |board, player| 1_000_000 * material(board, player) + f(board, player)
}

pub fn king_distance(board: &BitBoard, player: Color) -> i32 {
	let king = board.king_position(player.opposite());
	-(BitBoard::iter(board.all_pieces(player)) //
		.map(|pos| pos.l1_distance_to(king))
		.sum::<u8>() as i32)
}
