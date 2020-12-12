use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
	North,
	East,
	South,
	West,
}

impl Direction {
	fn dx(&self) -> i32 {
		use Direction::*;
		match self {
			North => 0,
			East => 1,
			South => 0,
			West => -1,
		}
	}

	fn dy(&self) -> i32 {
		use Direction::*;
		match self {
			North => 1,
			East => 0,
			South => -1,
			West => 0,
		}
	}

	fn turn_left_one(&self) -> Self {
		use Direction::*;
		match self {
			North => West,
			West => South,
			South => East,
			East => North,
		}
	}

	fn turn_left(&self, count: usize) -> Self {
		let mut ret = *self;
		for _ in 0usize..count {
			ret = ret.turn_left_one();
		}
		ret
	}

	fn turn_right_one(&self) -> Self {
		use Direction::*;
		match self {
			North => East,
			East => South,
			South => West,
			West => North,
		}
	}

	fn turn_right(&self, count: usize) -> Self {
		let mut ret = *self;
		for _ in 0usize..count {
			ret = ret.turn_right_one();
		}
		ret
	}
}

impl Default for Direction {
	fn default() -> Self {
		Self::East
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Part1State {
	x: i32,
	y: i32,
	face: Direction,
}

impl Default for Part1State {
	fn default() -> Self {
		Self {
			x: 0,
			y: 0,
			face: Default::default(),
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Part2State {
	ship: (i32, i32),
	target: (i32, i32),
}

impl Default for Part2State {
	fn default() -> Self {
		Self {
			ship: (0, 0),
			target: (10, 1),
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Instruction {
	Translate(Direction, i32),
	Left(usize),
	Right(usize),
	Forward(i32),
}

impl Instruction {
	fn execute_part1(&self, init: &Part1State) -> Part1State {
		match self {
			Self::Translate(direction, distance) => Part1State {
				x: init.x + direction.dx() * distance,
				y: init.y + direction.dy() * distance,
				face: init.face,
			},
			Self::Left(count) => Part1State {
				x: init.x,
				y: init.y,
				face: init.face.turn_left(*count),
			},
			Self::Right(count) => Part1State {
				x: init.x,
				y: init.y,
				face: init.face.turn_right(*count),
			},
			Self::Forward(distance) => Part1State {
				x: init.x + init.face.dx() * distance,
				y: init.y + init.face.dy() * distance,
				face: init.face,
			},
		}
	}

	fn rotate_point_left(point: (i32, i32), count: usize) -> (i32, i32) {
		Self::rotate_point_right(point, count * 3)
	}

	fn rotate_point_right(mut point: (i32, i32), count: usize) -> (i32, i32) {
		for _ in 0..(count % 4) {
			point = (point.1, -point.0);
		}
		point
	}

	fn execute_part2(&self, init: &Part2State) -> Part2State {
		match self {
			Self::Translate(direction, distance) => Part2State {
				ship: init.ship,
				target: (
					init.target.0 + direction.dx() * distance,
					init.target.1 + direction.dy() * distance,
				),
			},
			Self::Left(count) => Part2State {
				ship: init.ship,
				target: Self::rotate_point_left(init.target, *count),
			},
			Self::Right(count) => Part2State {
				ship: init.ship,
				target: Self::rotate_point_right(init.target, *count),
			},
			Self::Forward(distance) => Part2State {
				ship: (
					init.ship.0 + init.target.0 * distance,
					init.ship.1 + init.target.1 * distance,
				),
				target: init.target,
			},
		}
	}
}

impl FromStr for Instruction {
	type Err = std::num::ParseIntError;

	fn from_str(src: &str) -> Result<Self, Self::Err> {
		match src.chars().next().unwrap() {
			'N' => Ok(Self::Translate(Direction::North, src[1..].parse::<i32>()?)),
			'E' => Ok(Self::Translate(Direction::East, src[1..].parse::<i32>()?)),
			'S' => Ok(Self::Translate(Direction::South, src[1..].parse::<i32>()?)),
			'W' => Ok(Self::Translate(Direction::West, src[1..].parse::<i32>()?)),
			'L' => Ok(Self::Left(src[1..].parse::<usize>()? / 90)),
			'R' => Ok(Self::Right(src[1..].parse::<usize>()? / 90)),
			'F' => Ok(Self::Forward(src[1..].parse::<i32>()?)),
			other => panic!("Bad instruction character “{}”", other),
		}
	}
}

fn main() {
	let instructions: Vec<Instruction> = BufReader::new(File::open("input").unwrap())
		.lines()
		.map(|line| Instruction::from_str(&line.unwrap()).unwrap())
		.collect();

	{
		let mut cur = Part1State::default();
		for instruction in &instructions {
			cur = instruction.execute_part1(&cur);
		}
		println!("|{}| + |{}| = {}", cur.x, cur.y, cur.x.abs() + cur.y.abs());
	}
	{
		let mut cur = Part2State::default();
		for instruction in &instructions {
			cur = instruction.execute_part2(&cur);
		}
		println!(
			"|{}| + |{}| = {}",
			cur.ship.0,
			cur.ship.1,
			cur.ship.0.abs() + cur.ship.1.abs()
		);
	}
}
