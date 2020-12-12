use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
	Floor,
	Empty,
	Occupied,
}

impl From<char> for Cell {
	fn from(c: char) -> Self {
		match c {
			'.' => Self::Floor,
			'L' => Self::Empty,
			'#' => Self::Occupied,
			_ => panic!("invalid map char “{}”", c),
		}
	}
}

trait Rule {
	fn occupied_around(&self, field: &Field, base_row: usize, base_col: usize) -> usize;
	fn neighbour_limit() -> usize;

	fn apply(&self, field: &Field, row: usize, col: usize) -> Cell {
		match field.0[row][col] {
			Cell::Floor => Cell::Floor,
			Cell::Empty => {
				if self.occupied_around(field, row, col) == 0 {
					Cell::Occupied
				} else {
					Cell::Empty
				}
			}
			Cell::Occupied => {
				if self.occupied_around(field, row, col) >= Self::neighbour_limit() {
					Cell::Empty
				} else {
					Cell::Occupied
				}
			}
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Field(Vec<Vec<Cell>>);

impl Field {
	fn checked_offset(base: usize, delta: isize) -> Option<usize> {
		if delta < 0 {
			base.checked_sub(-delta as usize)
		} else {
			base.checked_add(delta as usize)
		}
	}

	fn get_offset(&self, base_row: usize, base_col: usize, dr: isize, dc: isize) -> Option<Cell> {
		let target_row = Self::checked_offset(base_row, dr);
		let target_col = Self::checked_offset(base_col, dc);
		if let Some(target_row) = target_row {
			if let Some(target_col) = target_col {
				if let Some(row) = self.0.get(target_row) {
					if let Some(cell) = row.get(target_col) {
						return Some(*cell);
					}
				}
			}
		}
		None
	}

	fn step<R: Rule>(&self, rule: &R) -> Self {
		Self(
			(0..self.0.len())
				.map(|row_num| {
					(0..self.0[row_num].len())
						.map(|col_num| rule.apply(&self, row_num, col_num))
						.collect()
				})
				.collect(),
		)
	}

	fn run_to_stable<R: Rule>(&self, rule: &R) -> Self {
		let mut cur = self.clone();
		loop {
			let next = cur.step(rule);
			if next == cur {
				break next;
			} else {
				cur = next;
			}
		}
	}
}

struct Part1Rule;

impl Rule for Part1Rule {
	fn occupied_around(&self, field: &Field, base_row: usize, base_col: usize) -> usize {
		(-1isize..=1isize)
			.map(|dr| {
				(-1isize..=1isize).map(move |dc| {
					if dr != 0 || dc != 0 {
						field.get_offset(base_row, base_col, dr, dc) == Some(Cell::Occupied)
					} else {
						false
					}
				})
			})
			.flatten()
			.filter(|i| *i)
			.count()
	}

	fn neighbour_limit() -> usize {
		4usize
	}
}

struct Part2Rule {
	sees: Vec<Vec<Vec<(usize, usize)>>>,
}

impl Part2Rule {
	fn new(field: &Field) -> Self {
		Self {
			sees: (0..field.0.len())
				.map(|row_num| {
					(0..field.0[row_num].len())
						.map(|col_num| Self::calc_sees(field, row_num, col_num))
						.collect()
				})
				.collect(),
		}
	}

	fn calc_sees(field: &Field, row: usize, col: usize) -> Vec<(usize, usize)> {
		let mut ret: Vec<(usize, usize)> = Vec::new();
		for dr in -1isize..=1isize {
			for dc in -1isize..=1isize {
				if dr != 0 || dc != 0 {
					let mut scaled_dr = dr;
					let mut scaled_dc = dc;
					loop {
						match field.get_offset(row, col, scaled_dr, scaled_dc) {
							None => break,
							Some(Cell::Floor) => {
								scaled_dr += dr;
								scaled_dc += dc;
							}
							Some(_) => {
								ret.push((
									Field::checked_offset(row, scaled_dr).unwrap(),
									Field::checked_offset(col, scaled_dc).unwrap(),
								));
								break;
							}
						}
					}
				}
			}
		}
		ret
	}
}

impl Rule for Part2Rule {
	fn occupied_around(&self, field: &Field, base_row: usize, base_col: usize) -> usize {
		self.sees[base_row][base_col]
			.iter()
			.filter(|(seen_row, seen_col)| field.0[*seen_row][*seen_col] == Cell::Occupied)
			.count()
	}

	fn neighbour_limit() -> usize {
		5usize
	}
}

fn main() {
	let initial_field = Field(
		BufReader::new(File::open("input").unwrap())
			.lines()
			.map(|line| line.unwrap().chars().map(Cell::from).collect())
			.collect(),
	);

	println!(
		"{}",
		initial_field
			.run_to_stable(&Part1Rule)
			.0
			.iter()
			.map(|row| row.iter().filter(|cell| **cell == Cell::Occupied).count())
			.sum::<usize>()
	);

	println!(
		"{}",
		initial_field
			.run_to_stable(&Part2Rule::new(&initial_field))
			.0
			.iter()
			.map(|row| row.iter().filter(|cell| **cell == Cell::Occupied).count())
			.sum::<usize>()
	);
}
