use super::internal::*;
use super::mailbox::*;
use super::*;
use std::ops::Sub;

// Compare BitBoard moves to Mailbox moves for a large number of random boards.
#[test]
fn random_all_moves() {
	for mb in &random_boards(1000) {
		let mut bb = Board::new();
		for (pos, sq) in mb.iter() {
			bb.set(pos, sq);
		}

		for player in [White, Black] {
			let have: Set<Move> = bb.collect_moves(player).iter().copied().collect();
			let want: Set<Move> = mb.collect_moves(player).iter().copied().collect();

			if have != want {
				println!("player: {}", player);
				println!("have: {:?}", &have);
				print_ansi(&bb, &have.iter().map(|mv| mv.to).collect());

				println!("want: {:?}", &want);
				print_ansi(&bb, &want.iter().map(|mv| mv.to).collect());

				println!("diff: +{:?}, -{:?}", have.sub(&want), want.sub(&have));
				panic!("test failed");
			}
		}
	}
}

fn random_boards(n: usize) -> Vec<Mailbox> {
	let seed = 12345;
	let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
	let mut boards = Vec::with_capacity(n);

	for _ in 0..n {
		let mut board = Mailbox::starting_position();

		// remove some random pieces (not kings)
		for _ in 0..rng.gen_range(10..100) {
			let r = rng.gen_range(0..8) as u8;
			let c = rng.gen_range(0..8) as u8;
			let pos = pos(r, c);
			if !board.at(pos).is_king() {
				board.set(pos, Square::Empty)
			}
		}

		for mv in 0..rng.gen_range(0..100) {
			let player = if mv % 2 == 0 { Color::White } else { Color::Black };
			let moves = board.collect_moves(player);
			if moves.len() != 0 {
				let mv = moves[rng.gen_range(0..moves.len())];
				if !board.at(mv.to).is_king() {
					board.do_move(mv)
				}
			}
		}

		boards.push(board);
	}

	boards
}
