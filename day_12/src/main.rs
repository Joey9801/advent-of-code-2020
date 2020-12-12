use std::str::FromStr;

use util::{
    geometry::{CardDir, Rotation},
    vec2::Vec2,
};

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    AbsoluteMove { dir: CardDir, value: u32, },
    RelativeMove { value: u32, },
    Turn { rot: Rotation, count: u32, },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InstructionParseError {
    InvalidAction,
    InvalidValue,
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action_char = s
            .chars()
            .next()
            .ok_or(InstructionParseError::InvalidAction)?;

        if !action_char.is_ascii_alphabetic() {
            return Err(InstructionParseError::InvalidAction);
        }

        // All actions have a value of the same type
        // All actions are a single ascii char, so s[1..] is always correct
        let value: u32 = s[1..]
            .parse()
            .map_err(|_| InstructionParseError::InvalidValue)?;

        if action_char == 'L' || action_char == 'R' {
            if value % 90 != 0 {
                return Err(InstructionParseError::InvalidValue);
            }
        }

        Ok(match action_char {
            'N' => Self::AbsoluteMove { value, dir: CardDir::Up },
            'S' => Self::AbsoluteMove { value, dir: CardDir::Down },
            'E' => Self::AbsoluteMove { value, dir: CardDir::Right },
            'W' => Self::AbsoluteMove { value, dir: CardDir::Left },
            'F' => Self::RelativeMove { value },
            'L' => Self::Turn { rot: Rotation::CounterClockwise, count: value / 90 },
            'R' => Self::Turn { rot: Rotation::Clockwise, count: value / 90 },
            _ => return Err(InstructionParseError::InvalidAction),
        })
    }
}

#[derive(Debug)]
struct ShipState1 {
    loc: Vec2,
    dir: CardDir,
}

impl ShipState1 {
    fn apply_instr(&mut self, instr: Instruction) {
        match instr {
            Instruction::AbsoluteMove { dir, value } => self.loc += dir.vec() * value,
            Instruction::RelativeMove { value } => self.loc += self.dir.vec() * value,
            Instruction::Turn { rot, count } => self.dir = self.dir.turn(rot, count as i32),
        }
    }
}

fn part_1(instructions: &[Instruction]) -> u32 {
    let mut ship = ShipState1 {
        loc: (0, 0).into(),
        dir: CardDir::Right,
    };

    for instr in instructions {
        ship.apply_instr(*instr);
    }

    ship.loc.l1_norm() as u32
}

#[derive(Debug)]
struct ShipState2 {
    /// Global location
    loc: Vec2,

    /// Relative to the ships location
    waypoint: Vec2,
}

impl ShipState2 {
    fn apply_instr(&mut self, instr: Instruction) {
        match instr {
            Instruction::AbsoluteMove { dir, value } => self.waypoint += dir.vec() * value,
            Instruction::RelativeMove { value } => self.loc += self.waypoint * value,
            Instruction::Turn { rot, count } => {
                for _ in 0..count {
                    self.waypoint = self.waypoint.rotate(rot)
                }
            }
        }
    }
}

fn part_2(instructions: &[Instruction]) -> u32 {
    let mut ship = ShipState2 {
        loc: (0, 0).into(),
        waypoint: (10, 1).into(),
    };

    for instr in instructions {
        ship.apply_instr(*instr);
    }

    ship.loc.l1_norm() as u32
}

fn main() {
    let instructions = INPUT
        .lines()
        .map(Instruction::from_str)
        .collect::<Result<Vec<_>, _>>()
        .expect("Expected a valid input");

    dbg!(part_1(&instructions));
    dbg!(part_2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INSTRUCTIONS: [Instruction; 5] = [
        Instruction::RelativeMove { value: 10 },
        Instruction::AbsoluteMove { dir: CardDir::Up, value: 3 },
        Instruction::RelativeMove { value: 7 },
        Instruction::Turn { rot: Rotation::Clockwise, count: 1 },
        Instruction::RelativeMove { value: 11 },
    ];

    fn real_instructions() -> Vec<Instruction> {
        INPUT
            .lines()
            .map(Instruction::from_str)
            .collect::<Result<Vec<_>, _>>()
            .expect("Expected a valid input")
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&EXAMPLE_INSTRUCTIONS), 25);
        assert_eq!(part_1(&real_instructions()), 923);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&EXAMPLE_INSTRUCTIONS), 286);
        assert_eq!(part_2(&real_instructions()), 24769);
    }
}