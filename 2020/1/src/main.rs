use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_pair_adding_to(sum: u32, dataset: &Vec<u32>) -> Option<(u32, u32)> {
	let mut seen = HashSet::<u32>::new();
	for v in dataset {
		if let Some(other) = sum.checked_sub(*v) {
			if seen.contains(&other) {
				return Some((*v, other));
			}
		}
		seen.insert(*v);
	}
	None
}

fn find_triple_adding_to(sum: u32, dataset: &Vec<u32>) -> Option<(u32, u32, u32)> {
	for x in dataset {
		if let Some((y, z)) = find_pair_adding_to(sum - x, dataset) {
			return Some((*x, y, z));
		}
	}
	None
}

fn main() {
	let values = BufReader::new(File::open("input").unwrap())
		.lines()
		.map(|line| line.unwrap().parse::<u32>().unwrap())
		.collect::<Vec<_>>();

	let part1 = find_pair_adding_to(2020, &values).unwrap();
	println!("{} × {} = {}", part1.0, part1.1, part1.0 * part1.1);

	let part2 = find_triple_adding_to(2020, &values).unwrap();
	println!(
		"{} × {} × {} = {}",
		part2.0,
		part2.1,
		part2.2,
		part2.0 * part2.1 * part2.2
	);
}
