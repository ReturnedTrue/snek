use ggez::{Context, GameResult};
use ggez::graphics::{self, Rect, Mesh, DrawMode, DrawParam, Color};
use ggez::timer;

use std::collections::vec_deque::VecDeque;

use super::game::{GameObject, GamePos};

#[derive(PartialEq, Clone)]
pub enum Direction {
	Left,
	Right,
	Up,
	Down,
}

pub enum MoveSnakeResult {
	DidNotMove,
	Moved,
	GameOver,
}

pub struct Snake {
	pub parts: VecDeque<GamePos>,	
	pub direction: Direction,
	pub just_ate: bool,

	width: f32,
	update_after_delta: f32,
	accumulated_delta: f32,
}

impl Snake {
	pub fn new(first_part: GamePos, width: f32, updates_per_second: f32) -> Snake {
		return Snake {
			parts: VecDeque::from([first_part]),
			width: width,
			direction: Direction::Right,
			just_ate: false,

			update_after_delta: 1f32 / updates_per_second,
			accumulated_delta: -1f32,
		}
	}

	pub fn move_snake(&mut self, ctx: &mut Context, col_and_row: (f32, f32)) -> MoveSnakeResult {
		if (self.accumulated_delta != -1f32 && self.accumulated_delta < self.update_after_delta) {
			self.accumulated_delta += timer::delta(ctx).as_secs_f32();
			return MoveSnakeResult::DidNotMove;
		} else {
			self.accumulated_delta = 0f32;
		}

		let mut new_front = self.parts
			.get(0)
			.expect("No front part of snake found")
			.clone();

		match self.direction {
			Direction::Left => new_front.0 -= 1.0,
			Direction::Right => new_front.0 += 1.0,
			Direction::Up => new_front.1 -= 1.0,
			Direction::Down => new_front.1 += 1.0,
		}

		if (!self.just_ate) {
			self.parts.pop_back();
		} else {
			// println!("Did just eat");
			self.just_ate = false;
		}
		
		if (new_front.0 < 0f32 || new_front.1 < 0f32) {
			return MoveSnakeResult::GameOver;
		}

		let (high_x, high_y) = (col_and_row.0 - 1f32, col_and_row.1 - 1f32);
		if (new_front.0 > high_x || new_front.1 > high_y) {
			return MoveSnakeResult::GameOver;
		}

		if (self.would_collide(new_front.0, new_front.1)) {
			return MoveSnakeResult::GameOver;
		}

		self.parts.push_front(new_front);

		return MoveSnakeResult::Moved;
	}
}

impl GameObject for Snake {
	fn render(&self, ctx: &mut Context) -> GameResult<()> {
		let meshes: Vec<_> = self.parts
			.iter()
			.map(|part| Rect::new(part.0 * self.width, part.1 * self.width, self.width, self.width))
			.map(|rect| Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::BLUE))
			.collect();
		
		for mesh in meshes {
			graphics::draw(ctx, &mesh?, DrawParam::default())?;
		}

		return Ok(());
	}

	fn would_collide(&self, x: f32, y: f32) -> bool {
		return self.parts.iter().any(|part| part.0 == x && part.1 == y)
	}
}