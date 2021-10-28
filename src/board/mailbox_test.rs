use super::internal::*;
use Color::*;
use Square::*;


#[test]
fn test_check() {
	let b1 = Mailbox::from_str(
		r"
		. . . k . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. r . . . . . .
		. . r . . . . K
		",
	)
	.unwrap();

	assert_eq!(b1.is_check(White), true);
	assert_eq!(b1.is_check(Black), false);
	assert_eq!(is_mate(&b1, White), true);
	assert_eq!(is_mate(&b1, Black), false);
}

#[test]
fn king_moves() {
	check_all_moves(
		BKing,
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . k
		",
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . x x
		. . . . . . x k
		",
	);
	check_all_moves(
		WKing,
		r"
		K . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		",
		r"
		K x . . . . . .
		x x . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		",
	);
	check_all_moves(
		WKing,
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . K . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		",
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. x x x . . . .
		. x K x . . . .
		. x x x . . . .
		. . . . . . . .
		. . . . . . . .
		",
	);
}

#[test]
fn knight_moves() {
	/*
	check_all_moves(
		WKnight,
		r"
		. . p . . . . .
		. . . . . . . .
		. N . . . . . .
		. . . p . . . .
		. . . . . . . .
		. . . . . N . .
		. . . . . . . .
		. . . . P . . .
		",
		r"
		x . x . . . . .
		. . . x . . . .
		. N . . . . . .
		. . . x x . x .
		x . x x . . . x
		. . . . . N . .
		. . . x . . . x
		. . . . P . x .
		",
	);
	*/
}

#[test]
fn queen_moves() {
	check_all_moves(
		WQueen,
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . p .
		. . . . . . . .
		. P . . Q . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		",
		r"
		x . . . x . . .
		. x . . x . . .
		. . x . x . x .
		. . . x x x . .
		. P x x Q x x x
		. . . x x x . .
		. . x . x . x .
		. x . . x . . x
		",
	);
}

#[test]
fn rook_moves() {
	check_all_moves(
		BRook,
		r"
		. . . . . . . r
		. . . . . . . .
		. . Q . . . . .
		. . . . . . . Q
		. . . . . . . .
		. . r . . . . .
		. . . . . . . .
		. . . . . . . .
		",
		r"
		x x x x x x x r
		. . . . . . . x
		. . x . . . . x
		. . x . . . . x
		. . x . . . . .
		x x r x x x x x
		. . x . . . . .
		. . x . . . . .
		",
	);
	check_all_moves(
		WRook,
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		R . . . . . . .
		",
		r"
		x . . . . . . .
		x . . . . . . .
		x . . . . . . .
		x . . . . . . .
		x . . . . . . .
		x . . . . . . .
		x . . . . . . .
		R x x x x x x x
		",
	);
}

#[test]
fn bpawn_moves() {
	check_all_moves(
		BPawn,
		r"
		. . . . . . . .
		p . p . . . . .
		. . . . p . . .
		Q Q p Q . . . .
		. . . . . p . .
		. . . . . p . .
		. . . . Q . . p
		p . . . . . p p
		",
		r"
		. . . . . . . .
		p . p . . . . .
		x . x . p . . .
		Q Q p x x . . .
		. . x . . p . .
		. . . . . p . .
		. . . . x x . p
		p . . . . . p p
		",
	);
}

#[test]
fn wpawn_moves() {
	check_moves(
		pos(1, 2),
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . r . . . .
		. . P . . . . .
		. . . . . . . .
		",
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . x . . . . .
		. . x x . . . .
		. . P . . . . .
		. . . . . . . .
		",
	);

	check_all_moves(
		WPawn,
		r"
		. . . . . . . P
		. . . . . . . .
		. . . R . . . .
		. . . P r . . .
		r . . . . P . .
		P . r . . . . .
		. . P . . . . P
		. . . . . . . .
		",
		r"
		. . . . . . . P
		. . . . . . . .
		. . . R . . . .
		. . . P x x . .
		r . . . . P . x
		P . r . . . . x
		. . P . . . . P
		. . . . . . . .
		",
	);
}

#[test]
fn set() {
	let mut b = Mailbox::new();
	b[pos(1, 2)] = WPawn;

	assert_eq!(
		b.to_string(),
		r"
8 . . . . . . . .
7 . . . . . . . .
6 . . . . . . . .
5 . . . . . . . .
4 . . . . . . . .
3 . . . . . . . .
2 . . P . . . . .
1 . . . . . . . .
  a b c d e f g h"
	);
}

#[test]
fn from_str() {
	let b = Mailbox::from_str(
		r"
rnbqkbnr
pppppppp
........
........
........
........
PPPPPPPP
RNBQKBNR
",
	)
	.unwrap();

	assert_eq!(b[pos(0, 7)], WRook);
	assert_eq!(b[pos(1, 0)], WPawn);
	assert_eq!(b[pos(7, 3)], BQueen);
}

#[test]
fn from_str_err() {
	let b = Mailbox::from_str(
		r"
rnbqkbnr
pppppppp
........
........
........
PPPPPPPP
RNBQKBNR
",
	);
	assert!(b.is_err())
}

// check the moves for all pieces of given type,
// by comparing to a stringified board where destinations are marked with `x`.
fn check_all_moves(piece: Square, board: &str, want: &str) {
	let board: Mailbox = board.parse().unwrap();
	let mut have = Set::default();
	for (pos, p) in board.iter() {
		if p == piece {
			have.extend(board.dests_for(pos))
		}
	}
	let who = piece.to_string();
	check_moves_(&who, &board, have, want)
}

// check the moves for piece at `pos`,
// by comparing to a stringified board where destinations are marked with `x`.
fn check_moves(pos: Pos, board: &str, want: &str) {
	let board: Mailbox = board.parse().unwrap();
	let have: Set<Pos> = board.dests_for(pos).iter().copied().collect();
	let who = format!("{} @ {}", board[pos], pos);
	check_moves_(&who, &board, have, want)
}

fn check_moves_(who: &str, board: &Mailbox, have: Set<Pos>, want: &str) {
	for pos in &have {
		assert!(pos.is_valid())
	}
	let want = parse_positions(want);

	if have != want {
		println!(
			"moves for {}\ngot: {:?}\n{}\nwant:{:?}\n{}",
			who,
			&have,
			mark_moves(&board, &have),
			&want,
			mark_moves(&board, &want)
		);
		panic!("test failed")
	}
}

// render `board` as text, but mark destinations as `x`.
fn mark_moves(board: &Mailbox, dests: &Set<Pos>) -> String {
	let marks = board
		.iter()
		.map(|(pos, piece)| if dests.contains(&pos) { 'x' } else { piece.to_char() });
	format_board(marks)
}
