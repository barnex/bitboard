use super::internal::*;

pub fn material_value(board: &BitBoard, player: Color) -> i32 {
	Square::ALL_PIECES //
		.into_iter()
		.map(|p| p.value() * (board.piece_count(p) as i32))
		.sum::<i32>()
		* player.sign()
}

pub fn basic_value(b: &BitBoard, player: Color) -> i32 {
	1000 * material_value(b, player) + strategic_value(b, player)
}

fn strategic_value(b: &BitBoard, player: Color) -> i32 {
	(forward_value(b, player) as i32) - (forward_value(b, player.opposite()) as i32)
}

fn forward_value(b: &BitBoard, player: Color) -> u32 {
	//let reach = b.reach(player);
	//let other_reach = b.reach(player.opposite());

	//let own = b.all_pieces(player);
	//let own_king = b.bits(player.king());
	//let other = b.all_pieces(player.opposite());
	//let mobility = reach & b.empty();

	//let protect = reach & (own & !own_king);
	//let threaten = reach & other;
	//let threaten_unprotected = reach & (other & !other_reach);

	//let mobility = mobility.count_ones();
	//let protect = protect.count_ones();
	//let threaten = threaten.count_ones();
	//let threaten_unprotected = threaten_unprotected.count_ones();

	//let check = if b.is_check(player.opposite()) { 900 } else { 0 };

	//mobility + 3 * protect + 2 * threaten + 10 * threaten_unprotected + check

	let mobility = b.reach(player) & !b.reach(player.opposite());
	mobility.count_ones()
}
