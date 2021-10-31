use super::internal::*;
use std::ops::Sub;
use Color::*;
use Square::*;

#[test]
fn test_iter() {
	let b = BitBoard::from_str(
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		",
	)
	.unwrap();

	let have = BitBoard::iter(b.black()).collect::<Vec<_>>();
	assert_eq!(have, vec![]);

	let b = BitBoard::from_str(
		r"
		. . . . . . . p
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . r . . .
		. . . . . . . .
		. . . . . . . .
		p . . . . . . .
		",
	)
	.unwrap();

	let have = BitBoard::iter(b.black()).collect::<Vec<_>>();
	assert_eq!(have, vec![pos(0, 0), pos(3, 4), pos(7, 7)]);
}

#[test]
fn test_king_position() {
	let b = BitBoard::from_str(
		r"
		. . . . . . . k
		. . . . . . . .
		. . . p . . . .
		. . . . . . . .
		. . . . . R . .
		. . . . . . . .
		. . . . . . . .
		K . . . . . . .
		",
	)
	.unwrap();

	assert_eq!(b.king_position(White), pos(0, 0));
	assert_eq!(b.king_position(Black), pos(7, 7));

	let b = BitBoard::from_str(
		r"
		. . . . . . . p
		. . . . . . . .
		. . . p . . . .
		. . . . . . . .
		. . k . . R . .
		. . . . . . . .
		. . . . . K . .
		n . . . . . . .
		",
	)
	.unwrap();

	assert_eq!(b.king_position(White), pos(1, 5));
	assert_eq!(b.king_position(Black), pos(3, 2));
}

#[test]
fn test_check() {
	let b1 = BitBoard::from_str(
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
fn reach() {
	test_bits(
		BitBoard::w_reach,
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . p . . . . .
		. . . p . . . .
		Q . . p . . . K
		",
		r"
		x . . . . . . .
		x . . . . . . .
		x . . . . . . .
		x . . . . . . .
		x . . . . . . .
		x . x . . . . .
		x x . p . . x x
		Q x x x . . x x
		",
	);
	test_bits(
		BitBoard::w_reach,
		r"
		. . . . . . . .
		. . . . . P . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . p
		p . . P . . . .
		P . P . P . . P
		. . . . . . . .
		",
		r"
		. . . . x x x .
		. . . . . P . .
		. . . . . . . .
		. . . . . . . .
		. . x x x . . p
		p x x x x x x x
		P . P . P . . P
		. . . . . . . .
		",
	);
}

#[test]
fn all_moves() {
	test_moves(
		White,
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. K . . . . . .
		",
		&["Kb1a1", "Kb1c1", "Kb1a2", "Kb1b2", "Kb1c2"],
	);
	test_moves(
		Black,
		r"
		. . . . . . . n
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . n
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		",
		&["nh4g2", "nh4f3", "nh4f5", "nh4g6", "nh8g6", "nh8f7"],
	);
	test_moves(
		White,
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. p . . . . . .
		. . . . . . . .
		. R p . . . . .
		. . . . . . . .
		",
		&["Rb2b3", "Rb2b1", "Rb2c2", "Rb2a2", "Rb2b4"],
	);
	test_moves(
		Black,
		r"
		b . . . . . . .
		. P . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		",
		&["ba8b7"],
	);
	test_moves(
		White,
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . Q . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		",
		&[
			"Qd4c3", "Qd4c5", "Qd4e4", "Qd4d2", "Qd4d6", "Qd4d8", "Qd4a1", "Qd4a7", "Qd4b2", "Qd4b4", "Qd4b6", "Qd4g1", "Qd4g7", "Qd4h4", "Qd4c4",
			"Qd4h8", "Qd4e3", "Qd4d1", "Qd4d3", "Qd4f2", "Qd4a4", "Qd4f4", "Qd4d5", "Qd4e5", "Qd4f6", "Qd4d7", "Qd4g4",
		],
	);
}

#[test]
fn king_moves() {
	test_bits(
		BitBoard::b_king_moves,
		r"
		k . . . . . . .
		. p . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		",
		r"
		k x . . . . . .
		x j . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		",
	);
	test_bits(
		BitBoard::b_king_moves,
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . P . . . . .
		. . p k . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		",
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . x x x . . .
		. . p k x . . .
		. . x x x . . .
		. . . . . . . .
		. . . . . . . .
		",
	);
	test_bits(
		BitBoard::w_king_moves,
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . p
		. . . . . . . K
		. . . . . . . P
		. . . . . . . .
		",
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . x x
		. . . . . . x K
		. . . . . . x P
		. . . . . . . .
		",
	);
}

#[test]
fn knight_moves() {
	test_bits(
		BitBoard::w_knight_moves,
		r"
		N . . . . . . .
		. . . . . . . .
		. . . . . p . .
		. . . . . . P .
		. . . . N . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		",
		r"
		N . . . . . . .
		. . x . . . . .
		. x . x . x . .
		. . x . . . P .
		. . . . N . . .
		. . x . . . x .
		. . . x . x . .
		. . . . . . . .
		",
	);
	test_bits(
		BitBoard::b_knight_moves,
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. n . . . . . .
		. . . p . . . .
		P . . . . . . .
		",
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		x . x . . . . .
		. . . x . . . .
		. n . . . . . .
		. . . . . . . .
		x . x . . . . .
		",
	);
}

#[test]
fn bisshop_moves() {
	test_bits(
		BitBoard::b_bisshop_moves,
		r"
		. . . . . . . .
		. b . . . . p .
		. . . . . . . b
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . P .
		. . . . . . . .
		",
		r"
		x . x . . . . .
		. b . . . . p .
		x . x . . . . b
		. . . x . . x .
		. . . . x x . .
		. . . . x x . .
		. . . x . . x .
		. . x . . . . .
		",
	);
}

#[test]
fn rook_moves() {
	test_bits(
		BitBoard::b_rook_moves,
		r"
		. . . . . . . .
		. p . . r . . P
		. . . . . . . .
		. . . . . . . .
		. P . . R . . .
		. . . . . . . .
		. . . . P . . .
		. . . . . . . .
		",
		r"
		. . . . x . . .
		. p x x r x x x
		. . . . x . . .
		. . . . x . . .
		. P . . x . . .
		. . . . . . . .
		. . . . P . . .
		. . . . . . . .
		",
	);
	test_bits(
		BitBoard::w_rook_moves,
		r"
		. . . . . . . .
		. p . . r . . P
		. . . . . . . .
		. . . . . . . .
		. P . . R . . .
		. . . . . . . .
		. . . . P . . .
		. . . . . . . .
		",
		r"
		. . . . . . . .
		. p . . x . . P
		. . . . x . . .
		. . . . x . . .
		. P x x R x x x
		. . . . x . . .
		. . . . P . . .
		. . . . . . . .
		",
	);
}

#[test]
fn rook_reach() {
	test_bits(
		|b| b.rook_reach(b.bits(BRook)),
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		r . . . . . . .
		",
		r"
		x . . . . . . .
		x . . . . . . .
		x . . . . . . .
		x . . . . . . .
		x . . . . . . .
		x . . . . . . .
		x . . . . . . .
		r x x x x x x x
		",
	);
	test_bits(
		|b| b.rook_reach(b.bits(WRook)),
		r"
		. . . . . . R .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . p . R . p .
		p . . . . . . .
		. . . . p . . .
		R p . P . P R .
		",
		r"
		x x x x x x R x
		. . . . x . x .
		. . . . x . x .
		. . . . x . x .
		. . x x R x x .
		x . . . x . x .
		x . . . x . x .
		R x . . . x R x
		",
	);
}

#[test]
fn slide_w() {
	test_bits(
		|b| b.slide(b.bits(WRook), sh_w),
		r"
		. . . . . . . R
		. . . . . . . .
		R . . . . . . .
		. . . . . . . .
		. . p . R . p .
		. . . . . . . .
		. . . . . . . .
		R R . P . . R .
		",
		r"
		x x x x x x x R
		. . . . . . . .
		R . . . . . . .
		. . . . . . . .
		. . x x R . . .
		. . . . . . . .
		. . . . . . . .
		x R . x x x R .
		",
	);
}

#[test]
fn slide_e() {
	test_bits(
		|b| b.slide(b.bits(WRook), sh_e),
		r"
		. . . . . . . R
		. . . . . . . .
		R . . . . . . .
		. . . . . . . .
		. . . . R . p .
		. . . . . . . .
		. . . . . . . .
		R . . P . . R .
		",
		r"
		. . . . . . . R
		. . . . . . . .
		R x x x x x x x
		. . . . . . . .
		. . . . R x x .
		. . . . . . . .
		. . . . . . . .
		R x x x . . R x
		",
	);
}

#[test]
fn all_b_pawn_moves() {
	test_moves(
		Black,
		r"
		. . . . . . . .
		. . . p . . . .
		. . . . R . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . p
		. . . . . . R R
		. . . . . . . .
		",
		&["pd7d6", "pd7d5", "pd7e6", "ph3g2"],
	);
}

#[test]
fn b_pawn_move() {
	test_bits(
		BitBoard::b_pawn_move,
		r"
		. . . . . . . .
		. . . p . . . .
		p . . . . . . .
		. R . . . . p .
		. . p . . R . R
		. R . R . . . .
		. . . . . . . .
		. . . . . p . .
		",
		r"
		. . . . . . . .
		. . . p . . . .
		p . . x . . . .
		x x . x . . p .
		. . p . . x x x
		. x x x . . . .
		. . . . . . . .
		. . . . . p . .
		",
	);
}

#[test]
fn b_pawn_capture() {
	test_bits(
		BitBoard::b_pawn_capture,
		r"
		. . . . . . . .
		. . . p . . . .
		p . . . . . . .
		. R . . . . p .
		. . p . . R . R
		. R . R . . . .
		. . . . . . . .
		. . . . . p . .
		",
		r"
		. . . . . . . .
		. . . p . . . .
		p . . . . . . .
		. x . . . . p .
		. . p . . x . x
		. x . x . . . .
		. . . . . . . .
		. . . . . p . .
		",
	);
}

#[test]
fn b_pawn_capture_we() {
	test_bits(
		BitBoard::b_pawn_capture_sw,
		r"
		. . . . . . . .
		. . . p . . . .
		p . . . . . . .
		. R . . . . p .
		. . p . . R . R
		. R . R . . . .
		. . . . . . . .
		. . . . . p . .
		",
		r"
		. . . . . . . .
		. . . p . . . .
		p . . . . . . .
		. R . . . . p .
		. . p . . x . R
		. x . R . . . .
		. . . . . . . .
		. . . . . p . .
		",
	);
}

#[test]
fn b_pawn_capture_se() {
	test_bits(
		BitBoard::b_pawn_capture_se,
		r"
		. . . . . . . .
		. . . p . . . .
		p . . . . . . .
		. R . . . . p .
		. . p . . R . R
		. R . R . . . .
		. . . . . . . .
		. . . . . p . .
		",
		r"
		. . . . . . . .
		. . . p . . . .
		p . . . . . . .
		. x . . . . p .
		. . p . . R . x
		. R . x . . . .
		. . . . . . . .
		. . . . . p . .
		",
	);
}

#[test]
fn b_pawn_push() {
	test_bits(
		BitBoard::b_pawn_push,
		r"
		. . . . . . . .
		p . p . . . p p
		r . . . . . R .
		. . . . . . . .
		. . p p p . . .
		. . . R . . . .
		. . . . . . . .
		p . . . . . . p
		",
		r"
		. . . . . . . .
		p . p . . . p p
		r . x . . . R x
		. . x . . . . x
		. . p p p . . .
		. . x R x . . .
		. . . . . . . .
		p . . . . . . p
		",
	);
}

#[test]
fn w_pawn_move() {
	test_bits(
		BitBoard::w_pawn_move,
		r"
		P . . . . . . P
		. . . . . . . .
		. . . . . . . .
		. r . . r . . .
		. . P P . . . .
		r P . . r P r .
		P . . . . P . P
		. . . . . . . .
		",
		r"
		P . . . . . . P
		. . . . . . . .
		. . . . . . . .
		. x x x x . . .
		. x P P . x . x
		r P . . x P x x
		P . . . . P . P
		. . . . . . . .
		",
	);
}

#[test]
fn w_pawn_capture() {
	test_bits(
		BitBoard::w_pawn_capture,
		r"
		P . . . . . . P
		. . . . . . . .
		. . . . . . . .
		. r . . r . . .
		. . P P . . . .
		r . . . r . r .
		P . . . . P . .
		. . . . . . . P
		",
		r"
		P . . . . . . P
		. . . . . . . .
		. . . . . . . .
		. x . . x . . .
		. . P P . . . .
		r . . . x . x .
		P . . . . P . .
		. . . . . . . P
		",
	);
}

#[test]
fn w_pawn_capture_nw() {
	test_bits(
		BitBoard::w_pawn_capture_nw,
		r"
		P . . . . . . P
		. . . . . . . .
		. . . . . . . .
		. r . . r . . .
		. . P P . . . .
		r . . . r . r .
		P . . . . P . .
		. . . . . . . P
		",
		r"
		P . . . . . . P
		. . . . . . . .
		. . . . . . . .
		. x . . r . . .
		. . P P . . . .
		r . . . x . . .
		P . . . . P . .
		. . . . . . . P
		",
	);
}

#[test]
fn w_pawn_capture_ne() {
	test_bits(
		BitBoard::w_pawn_capture_ne,
		r"
		P . . . . . . P
		. . . . . . . .
		. . . . . . . .
		. . . . r . . .
		. . P P . . . .
		r . . . r . r .
		P . . . . P . .
		. . . . . . . P
		",
		r"
		P . . . . . . P
		. . . . . . . .
		. . . . . . . .
		. . . . x . . .
		. . P P . . . .
		r . . . r . x .
		P . . . . P . .
		. . . . . . . P
		",
	);
}

#[test]
fn w_pawn_push() {
	test_bits(
		BitBoard::w_pawn_push,
		r"
		. . . . . . P .
		. . . . P . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . r . .
		. R . P . . . .
		P P . . . P . P
		. . . . . . . .
		",
		r"
		. . . . x . P .
		. . . . P . . .
		. . . . . . . .
		. . . . . . . .
		x . . x . R . x
		x R . P . x . x
		P P . . . P . P
		. . . . . . . .
		",
	);
}

#[test]
fn white_black() {
	test_bits(
		BitBoard::white,
		r"
		r n b k q . . .
		p . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		P . . . . . . .
		R N B K Q . . .
		",
		r"
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		x . . . . . . .
		x x x x x . . .
		",
	);
	test_bits(
		BitBoard::black,
		r"
		r n b k q . . .
		p . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		P . . . . . . .
		R N B K Q . . .
		",
		r"
		x x x x x . . .
		x . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		",
	);
}

fn test_moves(player: Color, board: &str, want: &[&str]) {
	let board = BitBoard::from_str(board).unwrap();
	let have = board.collect_moves(player).iter().copied().collect::<Set<_>>();
	let want = want.iter().map(|s| Move::from_str(s).expect("move: syntax error")).collect::<Set<_>>();
	if have != want {
		println!("have: {:?}", &have);
		println!("want: {:?}", &want);
		println!("diff: +{:?}, -{:?}", have.sub(&want), want.sub(&have));
		let have = have.iter().map(|mv| mv.to).collect();
		let want = want.iter().map(|mv| mv.to).collect();
		println!("have:");
		print_ansi(&board, &have);
		println!("want:");
		print_ansi(&board, &want);
		panic!("test failed")
	}
}

fn test_bits<F: Fn(&BitBoard) -> u64>(f: F, board: &str, want: &str) {
	let board = BitBoard::from_str(board).unwrap();
	let have = as_set(f(&board));
	let want = parse_positions(want);
	if have != want {
		println!("have:");
		print_ansi(&board, &have);
		println!("want:");
		print_ansi(&board, &want);
		println!("diff: +{:?}, -{:?}", have.sub(&want), want.sub(&have));
		assert_eq!(have, want);
	}
}

fn as_set(bits: u64) -> Set<Pos> {
	let mut set = Set::default();
	for r in 0..8 {
		for c in 0..8 {
			let pos = pos(r, c);
			if bit_at(bits, pos) {
				set.insert(pos);
			}
		}
	}
	set
}
