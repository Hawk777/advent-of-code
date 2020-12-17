use puzzle_solver::Puzzle;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct Field {
	name: String,
	ranges: Vec<std::ops::RangeInclusive<u32>>,
}

impl FromStr for Field {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let colon = s.find(':').unwrap();
		let (name, ranges) = s.split_at(colon);
		let name = name.trim();
		let ranges = ranges.trim_start_matches(':');
		let ranges = ranges.trim();
		let ranges = ranges.split(" or ");
		let ranges = ranges
			.map(|i| {
				let hyphen = i.find('-').unwrap();
				let (left, right) = i.split_at(hyphen);
				let right = right.trim_start_matches('-');
				let left = left.parse::<u32>().unwrap();
				let right = right.parse::<u32>().unwrap();
				left..=right
			})
			.collect::<Vec<_>>();
		Ok(Field {
			name: name.to_owned(),
			ranges,
		})
	}
}

fn ticket_invalid_values_sum(ticket: &[u32], fields: &[Field]) -> u32 {
	ticket
		.iter()
		.filter(|field_value| {
			!fields
				.iter()
				.any(|field| field.ranges.iter().any(|range| range.contains(field_value)))
		})
		.sum()
}

fn ticket_has_invalid_value(ticket: &[u32], fields: &[Field]) -> bool {
	ticket.iter().any(|field_value| {
		!fields
			.iter()
			.any(|field| field.ranges.iter().any(|range| range.contains(field_value)))
	})
}

fn main() {
	let (fields, mine, nearby) = {
		let mut fields: Vec<Field> = Vec::new();
		let mut mine: Vec<u32> = Vec::new();
		let mut nearby: Vec<Vec<u32>> = Vec::new();
		let mut reading_mine = false;
		let mut reading_nearby = false;
		for line in BufReader::new(File::open("input").unwrap()).lines() {
			let line = line.unwrap();
			if line.is_empty() {
			} else if line == "your ticket:" {
				reading_mine = true;
				reading_nearby = false;
			} else if line == "nearby tickets:" {
				reading_mine = false;
				reading_nearby = true;
			} else if reading_mine {
				mine = line.split(',').map(|i| i.parse::<u32>().unwrap()).collect();
			} else if reading_nearby {
				nearby.push(line.split(',').map(|i| i.parse::<u32>().unwrap()).collect());
			} else {
				fields.push(Field::from_str(&line).unwrap());
			}
		}
		(fields, mine, nearby)
	};

	let error_rate: u32 = nearby
		.iter()
		.map(|ticket| ticket_invalid_values_sum(ticket, &fields))
		.sum();
	println!("{}", error_rate);

	let mut valid_tickets: Vec<&Vec<u32>> = nearby
		.iter()
		.filter(|ticket| !ticket_has_invalid_value(ticket, &fields))
		.collect();
	valid_tickets.push(&mine);
	let mut puzzle = Puzzle::new();
	// The position of each var in the vars vec is the position of the field in the tickets. The
	// value of each var is the position of the field in the “fields” vec.
	let vars = puzzle.new_vars_with_candidates_1d(
		fields.len(),
		&(0i32..(fields.len() as i32)).collect::<Vec<_>>(),
	);
	puzzle.all_different(&vars);
	for ticket in &valid_tickets {
		for (ticket_field_num, ticket_field_value) in ticket.iter().enumerate() {
			for (candidate_num, candidate_field) in fields.iter().enumerate() {
				if !candidate_field
					.ranges
					.iter()
					.any(|range| range.contains(ticket_field_value))
				{
					puzzle.remove_candidates(vars[ticket_field_num], &[candidate_num as i32]);
				}
			}
		}
	}
	let solution = puzzle.solve_unique().unwrap();
	println!(
		"{}",
		(0..fields.len())
			.into_iter()
			.filter(|&i| fields[solution[vars[i]] as usize]
				.name
				.starts_with("departure"))
			.fold(1u64, |acc, i| acc * (mine[i] as u64))
	);
}
