use crate::{Coord, board::Board};

pub trait Beast {
	fn new(position: Coord) -> Self;

	fn advance(
		&mut self,
		board: &Board,
		player_position: &Coord,
	) -> Option<Coord>;
}
