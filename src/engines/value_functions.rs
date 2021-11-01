use super::internal::*;

pub fn heuristic1(board: &Board, player: Color) -> i32 {
	let attck = board.attack_vectors();

	-1_000_000 * is_check(board, &attck, player)
		+ 1000 * material(board, player)
		+ 3 * protection(board, &attck, player)
		+ 2 * threat(board, &attck, player)
		+ 1 * mobility(&attck, player)
}

pub fn heuristic2(board: &Board, player: Color) -> i32 {
	let attck = board.attack_vectors();

	-1_000_000 * is_check(board, &attck, player) + 1000 * material(board, player)
}

pub fn material(board: &Board, player: Color) -> i32 {
	Square::ALL_PIECES //
		.into_iter()
		.map(|p| p.value() * (board.piece_count(p) as i32))
		.sum::<i32>()
		* player.sign()
}

fn is_check(board: &Board, attck: &AttacVector, player: Color) -> i32 {
	((board.bits(player.king()) & attck.all[player.opposite().index()]) == 0) as i32
}

pub fn protection(board: &Board, attck: &AttacVector, player: Color) -> i32 {
	protection1(board, attck, player) as i32 - protection1(board, attck, player.opposite()) as i32
}

pub fn protection1(board: &Board, attck: &AttacVector, player: Color) -> u32 {
	let expendable = [
		[WPawn, WRook, WKnight, WBisshop], //
		[BPawn, BRook, BKnight, BBisshop],
	][player.index()];

	let expenable_pos = expendable //
		.iter()
		.map(|&piece| board.bits(piece))
		.fold(0, |a, b| a | b);

	let protected_pos = expenable_pos & attck.all[player.index()];

	protected_pos.count_ones()
}

pub fn threat(board: &Board, attck: &AttacVector, player: Color) -> i32 {
	threat1(board, attck, player) as i32 - threat1(board, attck, player.opposite()) as i32
}

pub fn threat1(board: &Board, attck: &AttacVector, player: Color) -> u32 {
	(board.all_pieces(player.opposite()) & attck.all[player.index()]).count_ones()
}

pub fn mobility(attck: &AttacVector, player: Color) -> i32 {
	Square::ALL_PIECES //
		.into_iter()
		.map(|piece| (attck.bitfields[piece.index()].count_ones() as i32) * piece.sign())
		.sum::<i32>()
		* player.sign()
}

//pub fn material_and<F>(f: F) -> impl Fn(&Board, Color) -> i32
//where
//	F: Fn(&Board, Color) -> i32 + 'static,
//{
//	move |board, player| 1_000_000 * material(board, player) + f(board, player)
//}

pub fn king_distance(board: &Board, player: Color) -> i32 {
	let king = board.king_position(player.opposite());
	-(Board::iter(board.all_pieces(player)) //
		.map(|pos| pos.l1_distance_to(king))
		.sum::<u8>() as i32)
}

/// Always returns value 0.
/// Useful for testing only.
pub fn zero(_board: &Board, _player: Color) -> i32 {
	0
}
