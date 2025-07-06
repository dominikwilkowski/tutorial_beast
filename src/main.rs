use std::io::{Read, stdin};

mod board;
mod player;
mod raw_mode;

use crate::{board::Board, player::Player, raw_mode::RawMode};

pub const BOARD_WIDTH: usize = 39;
pub const BOARD_HEIGHT: usize = 20;
pub const TILE_SIZE: usize = 2;

pub const ANSI_YELLOW: &str = "\x1B[33m";
pub const ANSI_GREEN: &str = "\x1B[32m";
pub const ANSI_CYAN: &str = "\x1B[36m";
pub const ANSI_RESET: &str = "\x1B[39m";

#[derive(Copy, Clone, Debug)]
pub enum Tile {
	Empty,       // There will be empty spaces on our board "  "
	Player,      // We will need the player "◀▶"
	Block,       // Some tiles will be blocks "░░"
	StaticBlock, // Others will be blocks that can't be moved "▓▓"
}

pub enum Direction {
	Up,
	Right,
	Down,
	Left,
}

#[derive(Debug)]
struct Game {
	board: Board,
	player: Player,
}

impl Game {
	fn new() -> Self {
		Self {
			board: Board::new(),
			player: Player::new(),
		}
	}

	fn play(&mut self) {
		let stdin = stdin();
		let mut lock = stdin.lock();
		let mut buffer = [0_u8; 1];
		println!("{}", self.board.render());

		while lock.read_exact(&mut buffer).is_ok() {
			match buffer[0] as char {
				'w' => {
					self.player.advance(&mut self.board, Direction::Up);
				},
				'd' => {
					self.player.advance(&mut self.board, Direction::Right);
				},
				's' => {
					self.player.advance(&mut self.board, Direction::Down);
				},
				'a' => {
					self.player.advance(&mut self.board, Direction::Left);
				},
				'q' => {
					println!("Good bye");
					break;
				},
				_ => {},
			}

			println!("\x1B[{}F{}", BOARD_HEIGHT + 1 + 1, self.board.render());
		}
	}
}

fn main() {
	let _raw_mode = RawMode::enter();

	let mut game = Game::new();
	game.play();
}
