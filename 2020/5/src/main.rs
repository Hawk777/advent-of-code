use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pass {
	row: u32,
	col: u32,
}

impl Pass {
	fn new(s: &str) -> Pass {
		assert_eq!(s.len(), 10);
		Pass {
			row: u32::from_str_radix(&s[..7].replace('F', "0").replace('B', "1"), 2).unwrap(),
			col: u32::from_str_radix(&s[7..].replace('L', "0").replace('R', "1"), 2).unwrap(),
		}
	}

	fn from_id(id: u32) -> Pass {
		Pass {
			row: id / 8,
			col: id % 8,
		}
	}

	fn id(&self) -> u32 {
		self.row * 8 + self.col
	}
}

fn main() {
	let passes: HashSet<Pass> = BufReader::new(File::open("input").unwrap()).lines().map(|line| Pass::new(&line.unwrap())).collect();
	println!("{}", passes.iter().map(Pass::id).max().unwrap());
	for candidate in 1..1024 {
		if !passes.contains(&Pass::from_id(candidate)) && passes.contains(&Pass::from_id(candidate - 1)) && passes.contains(&Pass::from_id(candidate + 1)) {
			println!("{}", candidate);
		}
	}
}
