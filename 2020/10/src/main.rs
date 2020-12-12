use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let mut levels: Vec<u32> = BufReader::new(File::open("input").unwrap())
		.lines()
		.map(|line| line.unwrap().parse::<u32>().unwrap())
		.collect();
	let output_joltage = *levels.iter().max().unwrap() + 3;
	levels.push(output_joltage);
	levels.push(0u32);
	levels.sort_unstable();

	let diff_count = levels
		.iter()
		.zip(levels.iter().skip(1))
		.map(|(first, second)| second - first)
		.fold([0u32; 4], |mut counts, elt| {
			counts[elt as usize] += 1;
			counts
		});
	println!(
		"{} × {} = {}",
		diff_count[1],
		diff_count[3],
		diff_count[1] * diff_count[3]
	);

	let mut reach_counts: Vec<u64> = Vec::new();
	reach_counts.reserve(levels.len());
	reach_counts.push(1);
	for i in 1..levels.len() {
		let ways = (1usize..=3usize)
			.map(|adapter_delta| match i.checked_sub(adapter_delta) {
				None => 0,
				Some(from_adapter) => {
					if levels[i] - levels[from_adapter] <= 3 {
						reach_counts[from_adapter]
					} else {
						0
					}
				}
			})
			.sum();
		reach_counts.push(ways);
	}
	println!("{}", reach_counts[reach_counts.len() - 1]);
}
