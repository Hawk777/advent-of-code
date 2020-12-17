use std::collections::HashMap;

struct Game<'seed> {
	last_turn: u32,
	last_spoken: u32,
	prev_spoken_turns: HashMap<u32, u32>,
	seed: &'seed [u32],
}

impl<'seed> Game<'seed> {
	fn new(seed: &'seed [u32]) -> Self {
		Self {
			last_turn: 0,
			last_spoken: 0,
			prev_spoken_turns: HashMap::new(),
			seed,
		}
	}
}

impl<'seed> Iterator for Game<'seed> {
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item> {
		let ret = if let Some((&first, rest)) = self.seed.split_first() {
			self.seed = rest;
			first
		} else {
			if let Some(&prev_spoken_turn) = self.prev_spoken_turns.get(&self.last_spoken) {
				self.last_turn - prev_spoken_turn
			} else {
				0
			}
		};

		if self.last_turn != 0 {
			self.prev_spoken_turns
				.insert(self.last_spoken, self.last_turn);
		}
		self.last_turn += 1;
		self.last_spoken = ret;
		Some(ret)
	}
}

fn main() {
	for position in &[2020, 30000000] {
		let mut game = Game::new(&[9, 19, 1, 6, 0, 5, 4]);
		println!("{}", game.nth(position - 1).unwrap());
	}
}
