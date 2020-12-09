use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

struct CPUState {
	accumulator: i32,
	pc: usize,
}

impl Default for CPUState {
	fn default() -> Self {
		Self {
			accumulator: 0,
			pc: 0,
		}
	}
}

#[derive(Debug, Eq, PartialEq)]
enum ParseInstructionError {
	ParseIntError(ParseIntError),
	WrongTokenCount,
	BadOpcode(String),
}

impl Display for ParseInstructionError {
	fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
		match self {
			Self::ParseIntError(e) => e.fmt(f),
			Self::WrongTokenCount => {
				write!(f, "wrong token count in instruction (expected two tokens)")
			}
			Self::BadOpcode(opcode) => write!(f, "unrecognized opcode “{}”", opcode),
		}
	}
}

impl Error for ParseInstructionError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::ParseIntError(e) => Some(e),
			Self::WrongTokenCount => None,
			Self::BadOpcode(_) => None,
		}
	}
}

impl From<ParseIntError> for ParseInstructionError {
	fn from(e: ParseIntError) -> Self {
		Self::ParseIntError(e)
	}
}

#[derive(Debug, Eq, PartialEq)]
enum ExecError {
	JumpRange(usize, i32),
}

impl Display for ExecError {
	fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
		match self {
			Self::JumpRange(pc, distance) => write!(
				f,
				"jump distance {} at PC {} yields negative PC",
				distance, pc
			),
		}
	}
}

impl Error for ExecError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::JumpRange(_, _) => None,
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Instruction {
	Acc(i32),
	Jump(i32),
	Nop(i32),
}

impl Instruction {
	fn jump_pc(start: usize, distance: i32) -> Result<usize, ExecError> {
		if distance < 0 {
			start.checked_sub((-distance) as usize).ok_or(ExecError::JumpRange(start, distance))
		} else {
			start.checked_add(distance as usize).ok_or(ExecError::JumpRange(start, distance))
		}
	}

	fn exec(&self, state: &CPUState, program_len: usize) -> Result<CPUState, ExecError> {
		let result = match self {
			Self::Acc(delta) => CPUState {
				accumulator: state.accumulator + delta,
				pc: state.pc + 1,
			},
			Self::Jump(distance) => {
				let new = CPUState {
					accumulator: state.accumulator,
					pc: Self::jump_pc(state.pc, *distance)?,
				};
				if new.pc > program_len {
					Err(ExecError::JumpRange(state.pc, *distance))?;
				}
				new
			}
			Self::Nop(_) => CPUState {
				accumulator: state.accumulator,
				pc: state.pc + 1,
			},
		};
		Ok(result)
	}
}

impl FromStr for Instruction {
	type Err = ParseInstructionError;

	fn from_str(line: &str) -> Result<Self, Self::Err> {
		let mut parts = line.split_whitespace();
		let opcode = parts.next().ok_or(ParseInstructionError::WrongTokenCount)?;
		let operand = parts.next().ok_or(ParseInstructionError::WrongTokenCount)?;
		if let Some(_) = parts.next() {
			Err(ParseInstructionError::WrongTokenCount)?;
		}
		let operand: i32 = operand.parse()?;
		match opcode {
			"acc" => Ok(Self::Acc(operand)),
			"jmp" => Ok(Self::Jump(operand)),
			"nop" => Ok(Self::Nop(operand)),
			_ => Err(ParseInstructionError::BadOpcode(opcode.to_owned())),
		}
	}
}

#[derive(Debug)]
enum ApplicationError {
	IO(std::io::Error),
	ParseInstruction(ParseInstructionError),
	Exec(ExecError),
}

impl Display for ApplicationError {
	fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
		match self {
			Self::IO(e) => e.fmt(f),
			Self::ParseInstruction(e) => e.fmt(f),
			Self::Exec(e) => e.fmt(f),
		}
	}
}

impl Error for ApplicationError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::IO(e) => Some(e),
			Self::ParseInstruction(e) => Some(e),
			Self::Exec(e) => Some(e),
		}
	}
}

impl From<std::io::Error> for ApplicationError {
	fn from(e: std::io::Error) -> Self {
		Self::IO(e)
	}
}

impl From<ParseInstructionError> for ApplicationError {
	fn from(e: ParseInstructionError) -> Self {
		Self::ParseInstruction(e)
	}
}

impl From<ExecError> for ApplicationError {
	fn from(e: ExecError) -> Self {
		Self::Exec(e)
	}
}

enum ExecResult {
	Loop(CPUState),
	Terminate(CPUState),
}

fn exec(program: &Vec<Instruction>) -> Result<ExecResult, ExecError> {
	let mut reached: Vec<bool> = Vec::new();
	reached.resize(program.len(), false);
	let mut state = CPUState::default();
	loop {
		if state.pc == program.len() {
			break Ok(ExecResult::Terminate(state));
		}
		if reached[state.pc] {
			break Ok(ExecResult::Loop(state));
		}
		reached[state.pc] = true;
		state = program[state.pc].exec(&state, program.len())?;
	}
}

fn main() -> Result<(), ApplicationError> {
	let program: Vec<Instruction> = BufReader::new(File::open("input")?)
		.lines()
		.map(|line| line.map_err(ApplicationError::from))
		.map(|line| line.and_then(|contents| contents.parse().map_err(ApplicationError::from)))
		.collect::<Result<Vec<Instruction>, ApplicationError>>()?;

	match exec(&program)? {
		ExecResult::Loop(state) => println!("{}", state.accumulator),
		ExecResult::Terminate(_) => println!("Original program unexpectedly terminated"),
	}

	let mut modified = program.clone();
	for i in 0..modified.len() {
		if let Some(switched) = match program[i] {
			Instruction::Jump(operand) => Some(Instruction::Nop(operand)),
			Instruction::Nop(operand) => Some(Instruction::Jump(operand)),
			Instruction::Acc(_) => None,
		} {
			let original = modified[i];
			modified[i] = switched;
			let result = exec(&modified);
			modified[i] = original;
			match result {
				Ok(ExecResult::Terminate(state)) => {
					println!("{}", state.accumulator);
				},
				_ => (),
			}
		}
	}

	Ok(())
}
