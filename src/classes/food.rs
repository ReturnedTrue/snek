use ggez::{Context, GameResult};
use ggez::graphics::{self, Rect, Mesh, DrawMode, DrawParam, Color};

use super::game::{GamePos, GameObject};

pub struct Food {
	pub position: GamePos,
	pub width: f32,
}

impl Food {
	pub fn new(x: f32, y: f32, width: f32) -> Food {
		return Food {
			position: GamePos(x, y),
			width: width,
		}
	}
}

impl GameObject for Food {
	fn render(&self, ctx: &mut Context) -> GameResult<()> {
		let rect = Rect::new(self.position.0 * self.width, self.position.1 * self.width, self.width, self.width);
		let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::GREEN)?;

		graphics::draw(ctx, &mesh, DrawParam::default())?;

		return Ok(());
	}

	fn would_collide(&self, x: f32, y: f32) -> bool {
		return self.position.0 == x && self.position.1 == y;	
	}
}