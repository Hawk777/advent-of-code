use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Mask {
	and_v1: u64,
	or: u64,
	float_v2: u64,
}

impl Mask {
	fn apply_to_v1(&self, value: u64) -> u64 {
		(value | self.or) & self.and_v1
	}
}

impl Default for Mask {
	fn default() -> Self {
		Self {
			and_v1: 0xFFFFFFFFFu64,
			or: 0u64,
			float_v2: 0u64,
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Op {
	SetMask(Mask),
	WriteMem { address: u64, value: u64 },
}

impl FromStr for Op {
	type Err = std::num::ParseIntError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if let Some(s) = s.strip_prefix("mask = ") {
			let mut and_v1 = 0u64;
			let mut or = 0u64;
			let mut float_v2 = 0u64;
			for c in s.chars() {
				and_v1 <<= 1;
				or <<= 1;
				float_v2 <<= 1;
				match c {
					'0' => {
						and_v1 |= 0;
						or |= 0;
						float_v2 |= 0;
					}
					'1' => {
						and_v1 |= 1;
						or |= 1;
						float_v2 |= 0;
					}
					'X' => {
						and_v1 |= 1;
						or |= 0;
						float_v2 |= 1;
					}
					_ => panic!("Mask character “{}” is neither 0, 1, nor X", c),
				}
			}
			Ok(Op::SetMask(Mask {
				and_v1,
				or,
				float_v2,
			}))
		} else if let Some(s) = s.strip_prefix("mem[") {
			let cb_pos = s.find(']').expect("Missing “]” in memory assignment");
			let address: u64 = s[..cb_pos].parse()?;
			if let Some(s) = s[cb_pos..].strip_prefix("] = ") {
				let value: u64 = s.parse()?;
				Ok(Op::WriteMem { address, value })
			} else {
				panic!("Memory assignment does not have “] = ” in it");
			}
		} else {
			panic!("Op “{}” starts with neither “mask = ” nor “mem[”", s);
		}
	}
}

fn spread_bits(mut value: u64, mut positions: u64) -> u64 {
	let mut ret = 0u64;
	while value != 0u64 {
		let next_pos = positions.trailing_zeros();
		if (value & 1) != 0 {
			ret |= 1u64 << next_pos;
		}
		positions &= !(1u64 << next_pos);
		value >>= 1;
	}
	ret
}

fn main() {
	let insns: Vec<_> = BufReader::new(File::open("input").unwrap())
		.lines()
		.map(|i| Op::from_str(&i.unwrap()).unwrap())
		.collect();

	{
		let mut mem: HashMap<u64, u64> = HashMap::new();
		let mut mask = Mask::default();
		for i in &insns {
			match i {
				Op::SetMask(m) => mask = *m,
				Op::WriteMem { address, value } => {
					mem.insert(*address, mask.apply_to_v1(*value));
				}
			}
		}
		println!("{}", mem.values().sum::<u64>());
	}

	{
		let mut mem: HashMap<u64, u64> = HashMap::new();
		let mut mask = Mask::default();
		for i in &insns {
			match i {
				Op::SetMask(m) => mask = *m,
				Op::WriteMem { mut address, value } => {
					address |= mask.or;
					address &= !mask.float_v2;
					for i in 0u64..(1u64 << mask.float_v2.count_ones()) {
						mem.insert(address | spread_bits(i, mask.float_v2), *value);
					}
				}
			}
		}
		println!("{}", mem.values().sum::<u64>());
	}
}
