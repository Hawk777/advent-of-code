use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn load() -> Vec<HashMap<String, String>> {
	let mut ret = vec![];
	let mut cur = HashMap::<String, String>::new();
	for line in BufReader::new(File::open("input").unwrap()).lines() {
		let line = line.unwrap();
		let line = line.trim();
		if line.is_empty() {
			if !cur.is_empty() {
				ret.push(cur);
				cur = HashMap::<String, String>::new();
			}
		} else {
			for datum in line.split_whitespace() {
				let colon = datum.find(':').unwrap();
				let key = &datum[..colon];
				let value = &datum[colon + 1..];
				cur.insert(key.to_owned(), value.to_owned());
			}
		}
	}
	if !cur.is_empty() {
		ret.push(cur);
	}
	ret
}

fn valid_year(value: &str, min: u32, max: u32) -> bool {
	if value.len() == 4{
		if let Ok(n) = value.parse::<u32>() {
			return (min..=max).contains(&n)
		}
	}
	false
}

fn valid_byr(value: &str) -> bool {
	valid_year(value, 1920, 2002)
}

fn valid_iyr(value: &str) -> bool {
	valid_year(value, 2010, 2020)
}

fn valid_eyr(value: &str) -> bool {
	valid_year(value, 2020, 2030)
}

fn valid_hgt_impl(value: &str, min: u32, max: u32) -> bool {
	let value = &value[..value.len() - 2];
	if let Ok(n) = value.parse::<u32>() {
		(min..=max).contains(&n)
	} else {
		false
	}
}

fn valid_hgt(value: &str) -> bool {
	if value.ends_with("cm") {
		valid_hgt_impl(value, 150, 193)
	} else if value.ends_with("in") {
		valid_hgt_impl(value, 59, 76)
	} else {
		false
	}
}

fn valid_hcl(value: &str) -> bool {
	value.len() == 7 && value.starts_with('#') && value[1..].chars().all(|ch| "0123456789abcdef".contains(ch))
}

fn valid_ecl(value: &str) -> bool {
	["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value)
}

fn valid_pid(value: &str) -> bool {
	value.len() == 9 && value.chars().all(|ch| "0123456789".contains(ch))
}

fn main() {
	let passports: Vec<HashMap<String, String>> = load();
	let required_fields: [(&str, fn(&str)->bool); 7] = [("byr", valid_byr), ("iyr", valid_iyr), ("eyr", valid_eyr), ("hgt", valid_hgt), ("hcl", valid_hcl), ("ecl", valid_ecl), ("pid", valid_pid)];
	println!("{}", passports.iter().filter(|p| required_fields.iter().all(|(f, _)| p.contains_key::<str>(f))).count());
	println!("{}", passports.iter().filter(|p| required_fields.iter().all(|(f, validate)| {
		if let Some(value) = p.get::<str>(f) {
			validate(value)
		} else {
			false
		}
	})).count());
}
