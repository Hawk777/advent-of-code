use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Content {
	name: String,
	quantity: usize,
}

impl Content {
	fn new(input: &[&str]) -> Content {
		assert!(input.len() > 1);
		let quantity = input[0].parse::<usize>().unwrap();
		Content {
			name: input[1..].join(" "),
			quantity,
		}
	}
}

fn parse_line(input: &str) -> (String, Vec<Content>) {
	let separators: &[char] = &[' ', ',', '.'];
	let words: Vec<&str> = input
		.split(separators)
		.filter(|elt| !elt.is_empty())
		.collect();
	let (name, contents) = {
		let mut groups = words.split(|&elt| elt == "contain");
		let first = groups.next().unwrap();
		let second = groups.next().unwrap();
		(first, second)
	};
	assert_eq!(name.last(), Some(&"bags"));
	let name = &name[..name.len() - 1];
	let contents: Vec<Content> = if contents == ["no", "other", "bags"] {
		vec![]
	} else {
		contents
			.split(|&elt| elt == "bag" || elt == "bags")
			.filter(|elt| !elt.is_empty())
			.map(Content::new)
			.collect()
	};
	(name.join(" "), contents)
}

struct IContent {
	id: usize,
	quantity: usize,
}

fn make_contains(
	raw: &HashMap<String, Vec<Content>>,
	names: &HashMap<String, usize>,
) -> Vec<Vec<IContent>> {
	let mut ret: Vec<Vec<IContent>> = Vec::new();
	ret.resize_with(raw.len(), Default::default);
	for i in raw {
		let row = &mut ret[*names.get(i.0).unwrap()];
		row.reserve(i.1.len());
		for j in i.1 {
			row.push(IContent {
				id: *names.get(&j.name).unwrap(),
				quantity: j.quantity,
			});
		}
	}
	ret
}

fn invert(contains: &Vec<Vec<IContent>>) -> Vec<Vec<usize>> {
	let mut ret: Vec<Vec<usize>> = Vec::new();
	ret.resize_with(contains.len(), Default::default);
	for (container, contents) in contains.iter().enumerate() {
		for content in contents {
			ret[content.id].push(container);
		}
	}
	ret
}

fn search<'a, Iter: Iterator<Item = &'a usize>, Reach: Fn(usize) -> Iter>(
	start: usize,
	reach: Reach,
) -> HashSet<usize> {
	let mut ret: HashSet<usize> = HashSet::new();
	let mut queue: VecDeque<usize> = VecDeque::new();
	ret.insert(start);
	queue.push_back(start);
	while let Some(elt) = queue.pop_front() {
		for neighbour in reach(elt) {
			if ret.insert(*neighbour) {
				queue.push_back(*neighbour);
			}
		}
	}
	ret
}

fn count_contents(start: usize, contains: &Vec<Vec<IContent>>) -> usize {
	let mut ret = 0usize;
	let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
	queue.push_back((start, 1));
	while let Some((id, scale)) = queue.pop_front() {
		for content in &contains[id] {
			let content_quantity = content.quantity * scale;
			ret += content_quantity;
			queue.push_back((content.id, content_quantity));
		}
	}
	ret
}

fn main() {
	let raw: HashMap<String, Vec<Content>> = BufReader::new(File::open("input").unwrap())
		.lines()
		.map(|elt| parse_line(&elt.unwrap()))
		.collect();
	let names: HashMap<String, usize> = raw
		.iter()
		.enumerate()
		.map(|(pos, (name, _))| (name.clone(), pos))
		.collect();
	let contains = make_contains(&raw, &names);
	let contained_in = invert(&contains);
	drop(raw);
	println!(
		"{}",
		search(*names.get("shiny gold").unwrap(), |elt| contained_in
			.get(elt)
			.unwrap()
			.iter())
		.len() - 1 /* do not count the bag on its own */
	);
	println!(
		"{}",
		count_contents(*names.get("shiny gold").unwrap(), &contains)
	);
}
