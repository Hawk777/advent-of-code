use std::fs::File;
use std::io::{BufRead, BufReader};

struct Map {
	data: Vec<Vec<bool>>,
}

impl Map {
	fn new() -> Self {
		Map {
			data: BufReader::new(File::open("input").unwrap())
				.lines()
				.map(|line| {
					line.unwrap()
						.chars()
						.map(|c| match c {
							'#' => true,
							'.' => false,
							_ => panic!("bad char"),
						})
						.collect()
				})
				.collect(),
		}
	}

	fn tree_at(&self, x: i32, y: i32) -> bool {
		0 <= y && (y as usize) < self.data.len() && {
			let row = &self.data[y as usize];
			let xm = x % (row.len() as i32);
			let xm = if xm < 0 { xm + row.len() as i32 } else { xm };
			row[xm as usize]
		}
	}

	fn evaluate_slope(&self, dx: i32, dy: i32) -> usize {
		let mut x = 0i32;
		let mut y = 0i32;
		let mut hits = 0usize;
		while y < self.data.len() as i32 {
			x += dx;
			y += dy;
			if self.tree_at(x, y) {
				hits += 1;
			}
		}
		hits
	}
}

fn main() {
	let map = Map::new();
	println!("{}", map.evaluate_slope(3, 1));
	let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
	println!(
		"{}",
		slopes
			.iter()
			.map(|slope| map.evaluate_slope(slope.0, slope.1))
			.product::<usize>()
	);
}
