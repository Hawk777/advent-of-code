use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{BitAnd, BitOr};

fn main() {
	let groups: Vec<Vec<u32>> = BufReader::new(File::open("input").unwrap())
		.lines()
		.map(|line| {
			line.unwrap()
				.chars()
				.map(|ch| 1u32 << ((ch as u32) - ('a' as u32)))
				.fold(0u32, u32::bitor)
		})
		.collect::<Vec<u32>>()
		.split(|&elt| elt == 0u32)
		.map(<[u32]>::to_vec)
		.collect();
	println!(
		"{}",
		groups
			.iter()
			.map(|group| group.iter().fold(0u32, u32::bitor).count_ones())
			.sum::<u32>()
	);
	println!(
		"{}",
		groups
			.iter()
			.map(|group| group.iter().fold(0xFFFFFFFFu32, u32::bitand).count_ones())
			.sum::<u32>()
	);
}
