use std::io::{self, prelude::Write};
use std::fs;

const HIGHSCORE_FILE: &'static str = "highscore.txt";

pub struct HighscoreService {
	cached_highscore: Option<u32>,
}

impl HighscoreService {
	pub fn start() -> HighscoreService {
		return HighscoreService {
			cached_highscore: None,
		}
	}

	pub fn is_new_highscore(&mut self, score: u32) -> bool {
		let current_high_score = self.cached_highscore.unwrap_or(
			self.get_highscore().unwrap()
		);
		
		return current_high_score < score;
	}

	pub fn get_highscore(&mut self) -> Result<u32, &str> {
		if (self.cached_highscore.is_some()) {
			return Ok(self.cached_highscore.unwrap());
		}

		let file_result = fs::read_to_string(HIGHSCORE_FILE);

		match file_result {
			Ok(file_contents) => {
				return file_contents.parse::<u32>().map_err(|_err| "Parsing failed");
			}

			Err(err) => {
				if (err.kind() == io::ErrorKind::NotFound) {
					self.cached_highscore = Some(0);
					return Ok(0);
				}

				return Err("IO Error");
			}
		};
	}

	pub fn register_score(&mut self, score: u32) -> Result<(), &str> {
		if (!self.is_new_highscore(score)) {
			return Ok(());
		}

		let open_result = fs::OpenOptions::new()
			.write(true)
			.create(true)
			.open(HIGHSCORE_FILE);

		let mut file = match open_result {
			Ok(file) => file,
			Err(_err) => return Err("IO Error")
		};

		let score_string = score.to_string();
		let write_result = file.write(score_string.as_bytes());

		if (write_result.is_err()) {
			return Err("IO Error");
		}

		self.cached_highscore = Some(score);
		return Ok(());
	}
} 
