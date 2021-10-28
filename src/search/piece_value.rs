use super::internal::*;

pub fn material(b: &impl Board, player: Color) -> i32 {
	player.sign() * b.material_value()
}

