use std::fs::File;
use std::io::{BufRead, BufReader};

struct Record {
	pos1: usize,
	pos2: usize,
	letter: char,
	password: String,
}

impl Record {
	fn new(line: &str) -> Self {
		let hyphen = line.find('-').unwrap();
		let (pos1, line) = line.split_at(hyphen);
		let pos1 = pos1.parse().unwrap();
		let line = &line[1..];
		let space = line.find(' ').unwrap();
		let (pos2, line) = line.split_at(space);
		let pos2 = pos2.parse().unwrap();
		let line = &line[1..];
		let letter = line.chars().next().unwrap();
		let line = &line[1..];
		assert_eq!(line.chars().next().unwrap(), ':');
		let line = &line[1..];
		assert_eq!(line.chars().next().unwrap(), ' ');
		let line = &line[1..];
		let password = line;
		assert!(pos1 <= pos2);
		Record {
			pos1,
			pos2,
			letter,
			password: password.to_owned(),
		}
	}

	fn valid1(&self) -> bool {
		(self.pos1..=self.pos2)
			.contains(&self.password.chars().filter(|&c| c == self.letter).count())
	}

	fn valid2(&self) -> bool {
		[self.pos1, self.pos2]
			.iter()
			.map(|&pos| {
				pos <= self.password.len()
					&& (*self.password)[pos - 1..].chars().next().unwrap() == self.letter
			})
			.fold(false, |x, y| x ^ y)
	}
}

fn main() {
	for function in &[Record::valid1, Record::valid2] {
		println!(
			"{}",
			BufReader::new(File::open("input").unwrap())
				.lines()
				.map(|line| Record::new(&line.unwrap()))
				.filter(function)
				.count()
		);
	}
}
