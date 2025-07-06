use std::{
	io::{Read, stdin},
	sync::mpsc,
	thread,
	time::{Duration, Instant},
};

use crate::{
	BOARD_HEIGHT, BOARD_WIDTH, Coord, Direction, TILE_SIZE, Tile,
	beasts::{Beast, CommonBeast},
	board::Board,
	level::Level,
	player::{AdvanceEffect, Player},
};

#[derive(Debug)]
pub struct Game {
	board: Board,
	player: Player,
	level: Level,
	beasts: Vec<CommonBeast>,
	input_receiver: mpsc::Receiver<u8>,
}

impl Game {
	pub fn new() -> Self {
		let (board, beasts) = Board::new(&Level::One);
		let (input_sender, input_receiver) = mpsc::channel::<u8>();
		let stdin = stdin();
		thread::spawn(move || {
			let mut lock = stdin.lock();
			let mut buffer = [0_u8; 1];
			while lock.read_exact(&mut buffer).is_ok() {
				if input_sender.send(buffer[0]).is_err() {
					break;
				}
			}
		});

		Self {
			board,
			player: Player::new(),
			level: Level::One,
			beasts,
			input_receiver,
		}
	}

	pub fn play(&mut self) {
		let mut last_tick = Instant::now();
		println!("{}", self.render(false));

		'game_loop: loop {
			if let Ok(byte) = self.input_receiver.try_recv() {
				let advance_effect = match byte as char {
					'w' => self.player.advance(&self.board, &Direction::Up),
					'd' => self.player.advance(&self.board, &Direction::Right),
					's' => self.player.advance(&self.board, &Direction::Down),
					'a' => self.player.advance(&self.board, &Direction::Left),
					'q' => {
						println!("Good bye");
						return;
					},
					_ => AdvanceEffect::Stay,
				};

				match advance_effect {
					AdvanceEffect::Stay => {},
					AdvanceEffect::MoveIntoTile(player_position) => {
						if self.board[&player_position] == Tile::CommonBeast {
							self.player.lives -= 1;
							if self.player.lives > 0 {
								let new_position = self.player.respawn(&self.board);
								self.board[&self.player.position] = Tile::Empty;
								self.player.position = new_position;
								self.board[&self.player.position] = Tile::Player;
							} else {
								self.board[&self.player.position] = Tile::Empty;
							}
						} else {
							self.board[&self.player.position] = Tile::Empty;
							self.player.position = player_position;
							self.board[&self.player.position] = Tile::Player;
						}
					},
					AdvanceEffect::MoveAndPushBlock {
						player_to,
						block_to,
					} => {
						self.board[&self.player.position] = Tile::Empty;
						self.player.position = player_to;
						self.board[&self.player.position] = Tile::Player;
						self.board[&block_to] = Tile::Block;
					},
					AdvanceEffect::SquishBeast {
						player_to,
						beast_at,
					} => {
						self.board[&self.player.position] = Tile::Empty;
						self.player.position = player_to;
						self.board[&self.player.position] = Tile::Player;
						self.beasts.retain_mut(|beast| beast.position != beast_at);
						self.board[&beast_at] = Tile::Block;
					},
				}

				println!("{}", self.render(true));
			}

			if last_tick.elapsed() > Duration::from_millis(1000) {
				last_tick = Instant::now();
				for beast in self.beasts.iter_mut() {
					if let Some(new_position) =
						beast.advance(&self.board, &self.player.position)
					{
						match self.board[&new_position] {
							Tile::Empty => {
								self.board[&beast.position] = Tile::Empty;
								beast.position = new_position;
								self.board[&new_position] = Tile::CommonBeast;
							},
							Tile::Player => {
								self.board[&beast.position] = Tile::Empty;
								beast.position = new_position;
								self.board[&new_position] = Tile::CommonBeast;
								self.player.lives -= 1;

								if self.player.lives > 0 {
									let new_position = self.player.respawn(&self.board);
									self.player.position = new_position;
									self.board[&self.player.position] = Tile::Player;
								}
							},
							_ => {},
						}
					}
				}
				println!("{}", self.render(true));
			}

			if self.player.lives == 0 {
				println!("Game Over");
				break 'game_loop;
			}

			if self.beasts.is_empty() {
				if let Some(level) = self.level.next() {
					let (board, beasts) = Board::new(&level);
					self.board = board;
					self.beasts = beasts;
					self.level = level;
					self.player.position = Coord { column: 0, row: 0 };
				} else {
					println!("You won");
					break 'game_loop;
				}
			}
		}
	}

	fn render(&self, reset: bool) -> String {
		const BORDER_SIZE: usize = 1;
		const FOOTER_SIZE: usize = 1;
		const FOOTER_LENGTH: usize = 11;

		let mut board = if reset {
			format!(
				"\x1B[{}F",
				BORDER_SIZE + BOARD_HEIGHT + BORDER_SIZE + FOOTER_SIZE
			)
		} else {
			String::new()
		};

		board.push_str(&format!(
			"{board}\n{footer:>width$}{level}  Lives: {lives}",
			board = self.board.render(),
			footer = "Level: ",
			level = self.level,
			lives = self.player.lives,
			width =
				BORDER_SIZE + BOARD_WIDTH * TILE_SIZE + BORDER_SIZE - FOOTER_LENGTH,
		));

		board
	}
}
