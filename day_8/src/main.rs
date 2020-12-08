use std::{iter, num::ParseIntError, str::FromStr};
use util::math::add;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum InstructionParseError {
    MissingOp,
    MissingArgument,
    UnrecognizedOp(String),
    BadNumber,
}

impl From<ParseIntError> for InstructionParseError {
    fn from(_: ParseIntError) -> Self {
        Self::BadNumber
    }
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();

        let op_str = tokens.next().ok_or(InstructionParseError::MissingOp)?;

        let arg_str = tokens
            .next()
            .ok_or(InstructionParseError::MissingArgument)?;
        let arg_str = arg_str.strip_prefix('+').unwrap_or(arg_str);

        Ok(match op_str {
            "jmp" => Instruction::Jmp(arg_str.parse()?),
            "acc" => Instruction::Acc(arg_str.parse()?),
            "nop" => Instruction::Nop(arg_str.parse()?),
            other => Err(InstructionParseError::UnrecognizedOp(other.to_string()))?,
        })
    }
}

struct Vm {
    instructions: Vec<Instruction>,
    visited_instructions: Vec<bool>,
    program_counter: usize,
    accumulator: i32,
}

impl Vm {
    fn new(instructions: Vec<Instruction>) -> Self {
        let visited_instructions = iter::repeat(false).take(instructions.len()).collect();

        Self {
            instructions,
            visited_instructions,
            program_counter: 0,
            accumulator: 0,
        }
    }

    fn reset(&mut self) {
        self.program_counter = 0;
        self.accumulator = 0;
        for x in self.visited_instructions.iter_mut() {
            *x = false;
        }
    }

    fn step(&mut self) {
        match self.instructions.get(self.program_counter as usize) {
            Some(instr) => {
                self.visited_instructions[self.program_counter] = true;
                match instr {
                    Instruction::Jmp(x) => self.program_counter = add(self.program_counter, *x),
                    Instruction::Acc(x) => self.accumulator += x,
                    Instruction::Nop(_) => (),
                }

                match instr {
                    Instruction::Jmp(_) => (),
                    _ => self.program_counter += 1,
                }
            }
            None => (),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VmTestResult {
    InfiniteLoop(i32),
    RegularTermination(i32),
    IrregularTermination,
}

fn test_vm(vm: &mut Vm) -> VmTestResult {
    vm.reset();
    while let Some(false) = vm.visited_instructions.get(vm.program_counter) {
        vm.step();
    }

    match vm.program_counter {
        x if x < vm.instructions.len() => VmTestResult::InfiniteLoop(vm.accumulator),
        x if x == vm.instructions.len() => VmTestResult::RegularTermination(vm.accumulator),
        x if x > vm.instructions.len() => VmTestResult::IrregularTermination,

        // Why does rustc demand this?
        _ => unreachable!(),
    }
}

fn fix_corruption(vm: &mut Vm) -> Option<i32> {
    fn swap_instr(i: &mut Instruction) {
        match *i {
            Instruction::Acc(_) => (),
            Instruction::Jmp(x) => *i = Instruction::Nop(x),
            Instruction::Nop(x) => *i = Instruction::Jmp(x),
        }
    }

    for trial in 0..vm.instructions.len() {
        swap_instr(&mut vm.instructions[trial]);

        match test_vm(vm) {
            VmTestResult::RegularTermination(x) => return Some(x),
            _ => (),
        }

        // This wasn't the one - swap it back
        swap_instr(&mut vm.instructions[trial]);
    }

    None
}

fn main() {
    let instructions = INPUT
        .lines()
        .map(Instruction::from_str)
        .collect::<Result<Vec<_>, _>>()
        .expect("Expected a valid input");

    let mut vm = Vm::new(instructions);
    dbg!(test_vm(&mut vm));

    dbg!(fix_corruption(&mut vm));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_parse() {
        assert_eq!(Ok(Instruction::Acc(10)), "acc +10".parse());
        assert_eq!(Ok(Instruction::Acc(-10)), "acc -10".parse());
        assert_eq!(Ok(Instruction::Jmp(10)), "jmp +10".parse());
        assert_eq!(Ok(Instruction::Jmp(-10)), "jmp -10".parse());
        assert_eq!(Ok(Instruction::Nop(10)), "nop +10".parse());
        assert_eq!(Ok(Instruction::Nop(-10)), "nop -10".parse());

        assert_eq!(
            Err(InstructionParseError::MissingOp),
            Instruction::from_str("")
        );
        assert_eq!(
            Err(InstructionParseError::MissingArgument),
            Instruction::from_str("acc")
        );
        assert_eq!(
            Err(InstructionParseError::UnrecognizedOp("foo".to_string())),
            Instruction::from_str("foo +10")
        );
        assert_eq!(
            Err(InstructionParseError::BadNumber),
            Instruction::from_str("acc asdf")
        );
    }

    fn example_vm() -> Vm {
        const SRC: &str = "nop +0;acc +1;jmp +4;acc +3;jmp -3;acc -99;acc +1;jmp -4;acc +6";
        let instructions = SRC
            .split(';')
            .map(Instruction::from_str)
            .collect::<Result<Vec<_>, _>>()
            .expect("Expected a valid input");

        Vm::new(instructions)
    }

    fn real_vm() -> Vm {
        let instructions = INPUT
            .lines()
            .map(Instruction::from_str)
            .collect::<Result<Vec<_>, _>>()
            .expect("Expected a valid input");

        Vm::new(instructions)
    }

    #[test]
    fn test_part_1() {
        assert_eq!(test_vm(&mut example_vm()), VmTestResult::InfiniteLoop(5));
        assert_eq!(test_vm(&mut real_vm()), VmTestResult::InfiniteLoop(1528));
    }

    #[test]
    fn test_part_2() {
        let mut vm = example_vm();
        assert_eq!(fix_corruption(&mut vm), Some(8));
        assert_eq!(test_vm(&mut vm), VmTestResult::RegularTermination(8));

        let mut vm = real_vm();
        assert_eq!(fix_corruption(&mut vm), Some(640));
        assert_eq!(test_vm(&mut vm), VmTestResult::RegularTermination(640));
    }
}
