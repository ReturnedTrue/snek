use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{self, Color, DrawParam, Font, Text, TextFragment};
use ggez::input::keyboard::KeyMods;
use ggez::timer;
use ggez::{Context, GameResult};

use rand::{self, Rng};

use super::food::Food;
use super::r#yield::Yield;
use super::snake::{Direction, MoveSnakeResult, Snake};
use crate::services::highscore::{HighscoreService};

pub trait GameObject {
	fn render(&self, _ctx: &mut Context) -> GameResult<()> {
		Ok(())
	}
	fn would_collide(&self, _x: f32, _y: f32) -> bool {
		false
	}
}

#[derive(PartialEq)]
pub enum GameState {
	Playing,
	ToBegin,
	GameOver,
}

#[derive(Clone)]
pub struct GamePos(pub f32, pub f32);

pub struct Game {
	snake: Snake,
	food: Vec<Food>,
	score: u32,

	state: GameState,
	game_over_yield: Yield,

	font: Font,
	cached_window_size: GamePos,

	highscorer: HighscoreService,

	columns: f32,
	rows: f32,
	item_width: f32,
	updates_per_second: u32,
}

impl Game {
	pub fn new(
		ctx: &mut Context,
		col_and_row: (f32, f32),
		item_width: f32,
		updates_per_second: u32,
	) -> Game {
		let inner_size = graphics::window(ctx).inner_size();

		return Game {
			snake: Snake::new(
				GamePos(col_and_row.0 / 2f32, col_and_row.1 / 2f32),
				item_width,
			),

			food: Vec::new(),
			score: 0,

			state: GameState::ToBegin,
			game_over_yield: Yield::new(3f32),

			font: Font::new(ctx, "/fonts/Android101.ttf").expect("Failed to load font"),
			cached_window_size: GamePos(inner_size.width as f32, inner_size.height as f32),

			highscorer: HighscoreService::start(),

			columns: col_and_row.0,
			rows: col_and_row.1,
			item_width: item_width,
			updates_per_second: updates_per_second,
		};
	}

	fn get_text<T>(&self, string: T) -> Text
	where T: Into<TextFragment> 
	{
		return Text::new(TextFragment::from((string, self.font, 16f32)));
	}

	fn reset_game(&mut self) {
		self.state = GameState::GameOver;

		self.snake = Snake::new(
			GamePos(self.columns / 2f32, self.rows / 2f32),
			self.item_width,
		);

		self.food.clear();
		self.add_new_food();
	}

	fn look_for_food_eaten(&mut self) {
		for food_index in 0..self.food.len() {
			let food = &self.food[food_index];

			if (self.snake.would_collide(food.position.0, food.position.1)) {
				self.snake.just_ate = true;

				self.food.remove(food_index);
				self.add_new_food();

				self.score += 1;

				break;
			}
		}
	}

	pub fn add_new_food(&mut self) {
		let mut generator = rand::thread_rng();

		loop {
			let new_x = generator.gen_range(0f32, self.columns);
			let new_y = generator.gen_range(0f32, self.rows);

			let collides_with_food = self
				.food
				.iter()
				.any(|food| food.would_collide(new_x, new_y));

			if (!collides_with_food && !self.snake.would_collide(new_x, new_y)) {
				let new_food = Food::new(new_x.floor(), new_y.floor(), self.item_width);
				self.food.push(new_food);

				break;
			}
		}
	}
}

impl EventHandler for Game {
	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		while (timer::check_update_time(ctx, self.updates_per_second)) {
	
			if (self.state == GameState::ToBegin) {
				return Ok(());
			} else if (self.state == GameState::GameOver) {
				if (self.game_over_yield.completed_yield(ctx)) {
					self.state = GameState::ToBegin;

					self.highscorer.register_score(self.score).unwrap();
					self.score = 0;
				}

				return Ok(());				
			}

			let move_result = self.snake.move_snake((self.columns, self.rows));

			match move_result {
				MoveSnakeResult::GameOver => self.reset_game(),
				MoveSnakeResult::Moved => self.look_for_food_eaten(),
			}			
		}

		return Ok(());
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx, Color::WHITE);

		match self.state {
			GameState::ToBegin => {
				let highscore = self.highscorer.get_highscore().unwrap();
				let text = self.get_text(format!("Press space to begin!\nHighscore: {}", highscore));
				let param = DrawParam::default().dest([0f32, 0f32]).color(Color::BLACK);

				graphics::draw(ctx, &text, param)?;
				graphics::present(ctx)?;

				return Ok(());
			}

			GameState::GameOver => {
				let string = format!(
					"Game Over!\nYour score was: {}{}", 
					self.score,
					if (self.highscorer.is_new_highscore(self.score)) {" (new highscore)"} else {""}
				);

				let text = self.get_text(string);
				let param = DrawParam::default().dest([0f32, 0f32]).color(Color::BLACK);

				graphics::draw(ctx, &text, param)?;
				graphics::present(ctx)?;

				return Ok(());
			},

			GameState::Playing => {
				let text = self.get_text(format!("Score: {}", self.score));
				let text_dimensions = text.dimensions(ctx);

				let param = DrawParam::default()
					.dest([0f32, self.cached_window_size.1 - text_dimensions.h])
					.color(Color::BLACK);

				graphics::draw(ctx, &text, param)?;
			}
		}

		self.snake.render(ctx)?;

		for food in self.food.iter() {
			food.render(ctx)?;
		}

		graphics::present(ctx)?;

		return Ok(());
	}

	fn key_down_event(
		&mut self,
		_ctx: &mut Context,
		keycode: KeyCode,
		_keymods: KeyMods,
		_repeat: bool,
	) {
		if (self.state == GameState::ToBegin) {
			if (keycode == KeyCode::Space) {
				self.state = GameState::Playing;
			}

			return;
		}

		let new_direction = match keycode {
			KeyCode::A | KeyCode::Left => Some(Direction::Left),
			KeyCode::D | KeyCode::Right => Some(Direction::Right),
			KeyCode::W | KeyCode::Up => Some(Direction::Up),
			KeyCode::S | KeyCode::Down => Some(Direction::Down),
			_ => None,
		};

		if (new_direction.is_some()) {
			let unwrapped_direction = new_direction.unwrap();
			let direction_denied = match self.snake.direction {
				Direction::Right => Direction::Left,
				Direction::Left => Direction::Right,
				Direction::Up => Direction::Down,
				Direction::Down => Direction::Up,
			};

			if (self.snake.parts.len() < 2 || unwrapped_direction != direction_denied) {
				self.snake.direction = unwrapped_direction;
			}
		}
	}
}
