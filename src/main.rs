#![allow(unused_parens)]
#![windows_subsystem = "windows"]

mod classes;
mod services;

use ggez::{ContextBuilder, GameResult};
use ggez::event;

use ggez::conf::{WindowMode, WindowSetup};

use classes::game::Game;

fn main() -> GameResult<()> {
	const COLUMNS: f32 = 20f32;
	const ROWS: f32 = 20f32;
	const ITEM_WIDTH: f32 = 20f32;
	const UPDATES_PER_SECOND: f32 = 5f32;

	let width = COLUMNS * ITEM_WIDTH;
	let height = ROWS * ITEM_WIDTH;

	let window_mode = WindowMode::default()
		.dimensions(width, height)
		.resizable(false);

	let window_setup = WindowSetup::default().title("Snek");

	let (mut ctx, event_loop) = ContextBuilder::new(env!("CARGO_PKG_NAME"), "ReturnedTrue")
		.window_mode(window_mode)
		.window_setup(window_setup)
		.build()?;

	let mut game = Game::new(&mut ctx, (COLUMNS, ROWS), ITEM_WIDTH, UPDATES_PER_SECOND);
	game.add_new_food();

    event::run(ctx, event_loop, game);
}

