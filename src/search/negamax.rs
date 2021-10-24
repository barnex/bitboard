use super::internal::*;

const INF: i32 = 1_000_000; // effectively infinite value

pub fn negamax(board: &Mailbox, depth: u32, c: Color, mv: Move) -> i32 {

	//if board[mv.to].mask(KIND_MASK) == KING{
	//	return - INF  * board[mv.to].color().map_or(0, Color::sign)
	//}


	let board = board.with_move(mv);
	if depth == 0{
		return material_value_white(&board);
	}

	let mut value = INF;

	for mv in board.all_moves(c.opposite().mask()){
		value = i32::min(value, -negamax(&board, depth-1, c.opposite(), mv))
	}

	value
}

/*
func (e *worf) negamax(n *Node, depth int, c Color, m Move) int {

	if dst := n.board.At(m.DstI()); dst == WK || dst == BK {
		return inf(-c * dst.Color())
	}

	if depth == 0 {
		return int(c) * Heuristic3(n, m)
	}

	value := inf(1)

	n2 := n.WithMove(m)
	n = nil
	for _, m := range AllMoves(&n2.board, -c) {
		v := e.negamax(n2, depth-1, -c, m) * -1
		value = min(value, v)
	}
	return value
}

*/
