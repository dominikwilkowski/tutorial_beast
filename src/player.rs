use crate::{BOARD_HEIGHT, BOARD_WIDTH, Direction, Tile, board::Board};

#[derive(Debug)]
pub struct Player {
	position: (usize, usize),
}

impl Player {
	pub fn new() -> Self {
		Self { position: (0, 0) }
	}

	pub fn advance(&mut self, board: &mut Board, direction: Direction) {
		board.buffer[self.position.1][self.position.0] = Tile::Empty;

		match direction {
			Direction::Up => {
				if self.position.1 > 0 {
					self.position.1 -= 1
				}
			},
			Direction::Right => {
				if self.position.0 < BOARD_WIDTH - 1 {
					self.position.0 += 1
				}
			},
			Direction::Down => {
				if self.position.1 < BOARD_HEIGHT - 1 {
					self.position.1 += 1
				}
			},
			Direction::Left => {
				if self.position.0 > 0 {
					self.position.0 -= 1
				}
			},
		}

		board.buffer[self.position.1][self.position.0] = Tile::Player;
	}
}
