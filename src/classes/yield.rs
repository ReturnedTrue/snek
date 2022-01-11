use ggez::Context;
use ggez::timer;

pub struct Yield {
	time_to_yield: f32,
	elapsed_time: Option<f32>,
}

impl Yield {
	pub fn new(time_to_yield: f32) -> Yield {
		return Yield {
			time_to_yield: time_to_yield,
			elapsed_time: None,
		}
	}

	pub fn completed_yield(&mut self, ctx: &mut Context) -> bool {
		let delta_time = timer::delta(ctx).as_secs_f32() * 50f32;
		let new_value = self.elapsed_time.unwrap_or(0f32) + delta_time;

		if (new_value > self.time_to_yield) {
			self.elapsed_time = None;
			return true;
		} else {
			self.elapsed_time = Some(new_value);
			return false;
		}
	}
}