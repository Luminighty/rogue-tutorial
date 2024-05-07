#[derive(Default)]
pub struct GameLog {
	pub entries: Vec<String>
}

impl GameLog {

	pub fn new() -> Self {
		Self {
			entries: vec!["Hello to DF".to_string()]
		}
	}

	pub fn log(&mut self, message: String) {
		self.entries.push(message);
	}
}