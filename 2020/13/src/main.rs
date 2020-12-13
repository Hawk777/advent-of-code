use ring_algorithm::chinese_remainder_theorem;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn next_departure_time(now: u32, id: u32) -> u32 {
	now + (id - (now % id)) % id
}

fn main() {
	let start_time;
	let bus_ids;
	{
		let mut lines = BufReader::new(File::open("input").unwrap()).lines();
		start_time = lines.next().unwrap().unwrap().parse::<u32>().unwrap();
		bus_ids = lines
			.next()
			.unwrap()
			.unwrap()
			.split(',')
			.map(|i| {
				if i == "x" {
					None
				} else {
					Some(i.parse::<u32>().unwrap())
				}
			})
			.collect::<Vec<Option<u32>>>();
	}
	let next_departing_id = bus_ids
		.iter()
		.filter_map(|i| *i)
		.min_by_key(|id| next_departure_time(start_time, *id))
		.unwrap();
	let next_departing_time = next_departure_time(start_time, next_departing_id);
	println!(
		"ID {} at {} (wait time {}) = {}",
		next_departing_id,
		next_departing_time,
		next_departing_time - start_time,
		(next_departing_time - start_time) * next_departing_id
	);

	let u: Vec<i64> = bus_ids
		.iter()
		.enumerate()
		.filter_map(|(index, id)| -> Option<i64> {
			if id.is_some() {
				Some(-(index as i64))
			} else {
				None
			}
		})
		.collect();
	let m: Vec<i64> = bus_ids
		.iter()
		.filter_map(|id| match id {
			Some(id) => Some((*id).into()),
			None => None,
		})
		.collect();
	println!("{}", chinese_remainder_theorem(&u, &m).unwrap());
}
