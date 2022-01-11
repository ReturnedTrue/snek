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
	const UPDATES_PER_SECOND: u32 = 5;

	let width = COLUMNS * ITEM_WIDTH;
	let height = ROWS * ITEM_WIDTH;

	let window_mode = WindowMode::default()
		.dimensions(width, height)
		.resizable(false);

	let color_mode = dark_light::detect();
	let icon_path = String::from("/images/icon_") + match color_mode {
		dark_light::Mode::Dark => "light.png",
		dark_light::Mode::Light => "dark.png"
	};

	let window_setup = WindowSetup::default()
		.title("Snek")
		.icon(&icon_path);

	let package_name = env!("CARGO_PKG_NAME");
	let (mut ctx, event_loop) = ContextBuilder::new(package_name, "ReturnedTrue")
		.window_mode(window_mode)
		.window_setup(window_setup)
		.build()?;

	let mut game = Game::new(&mut ctx, (COLUMNS, ROWS), ITEM_WIDTH, UPDATES_PER_SECOND);
	game.add_new_food();

    event::run(ctx, event_loop, game);
}

