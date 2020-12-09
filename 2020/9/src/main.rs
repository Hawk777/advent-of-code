use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

fn find_unequal_pair_adding_to(sum: u64, dataset: &[u64]) -> Option<(u64, u64)> {
	let mut seen = HashSet::<u64>::new();
	for v in dataset {
		if let Some(other) = sum.checked_sub(*v) {
			if *v != other {
				if seen.contains(&other) {
					return Some((*v, other));
				}
			}
		}
		seen.insert(*v);
	}
	None
}

fn find_first_invalid(numbers: &[u64]) -> u64 {
	for i in 25..numbers.len() {
		if let None = find_unequal_pair_adding_to(numbers[i], &numbers[i - 25..i]) {
			return numbers[i];
		}
	}
	panic!("no number found");
}

fn find_range_adding_to(sum: u64, dataset: &[u64]) -> Range<usize> {
	let mut candidate = 0..0;
	let mut candidate_sum = 0;
	loop {
		if candidate_sum == sum && ((candidate.end - candidate.start) >= 2) {
			break candidate;
		} else if candidate_sum < sum {
			candidate_sum += dataset[candidate.end];
			candidate.end += 1;
		} else {
			candidate_sum -= dataset[candidate.start];
			candidate.start += 1;
		}
	}
}

fn main() {
	let numbers: Vec<u64> = BufReader::new(File::open("input").unwrap())
		.lines()
		.map(|line| line.unwrap().parse().unwrap())
		.collect();
	let first_invalid = find_first_invalid(&numbers);
	println!("{}", first_invalid);
	let summing_range = find_range_adding_to(first_invalid, &numbers);
	let min = numbers[summing_range.clone()].iter().min().unwrap();
	let max = numbers[summing_range.clone()].iter().max().unwrap();
	println!("{} + {} = {}", min, max, min + max);
}
