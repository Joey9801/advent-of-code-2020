use std::{collections::HashMap, time::Instant, num::ParseIntError, str::FromStr};

const INPUT: &str = include_str!("../input.txt");

struct FloatingAddrIter {
    floating_bits: [u8; 36],
    floating_bit_count: u8,
    counter: u64,
    addr: usize,
}

impl Iterator for FloatingAddrIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter >= (1 << self.floating_bit_count) {
            None
        } else {
            // Scatter the bits of self.counter into the floating bits of the address
            for i in 0..self.floating_bit_count {
                let index = self.floating_bits[i as usize];
                if (self.counter >> i) & 1 == 0 {
                    self.addr &= !(1 << index);
                } else {
                    self.addr |= 1 << index;
                }
            }
            self.counter += 1;
            Some(self.addr)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MaskBit {
    Zero,
    One,
    X,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Mask {
    bits: [MaskBit; 36],
}

impl Mask {
    /// Apply this mask to an integer as described in part 1
    fn apply_1(&self, mut num: u64) -> u64 {
        self.bits.iter().enumerate()
            .for_each(|(id, bit)| {
                match bit {
                    MaskBit::Zero => num &= !(1 << id),
                    MaskBit::One => num |= 1 << id,
                    MaskBit::X => (),
                }
            });

        num
    }
    
    /// Apply this mask to an address as described in part 2, returning an iterator over all
    /// concrete addresses
    fn apply_2(&self, num: usize) -> impl Iterator<Item=usize> {
        let mut floating_bits = [0u8; 36];
        let mut floating_bit_count = 0u8;
        let mut addr = 0;

        self.bits.iter()
            .enumerate()
            .for_each(|(i, b)| match *b {
                MaskBit::Zero => addr |= num & (1 << i),
                MaskBit::One => addr |= 1 << i,
                MaskBit::X => {
                    floating_bits[floating_bit_count as usize] = i as u8;
                    floating_bit_count += 1;
                }
            });

        FloatingAddrIter {
            floating_bits,
            floating_bit_count,
            addr,
            counter: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MaskParseError {
    BadChar,
    BadLen,
}

impl FromStr for Mask {
    type Err = MaskParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(MaskParseError::BadChar);
        }
        if s.len() != 36 {
            return Err(MaskParseError::BadLen);
        }
        
        // Start off with an all ones and mask, and an all zeros or mask
        let mut bits = [MaskBit::X; 36];
        for (id, c) in s.chars().rev().enumerate() {
            match c {
                'X' => (),
                '1' => bits[id] = MaskBit::One,
                '0' => bits[id] = MaskBit::Zero,
                _ => return Err(MaskParseError::BadChar),
            }
        }

        Ok(Self { bits })
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ProgLine {
    SetMask(Mask),
    SetMem { index: usize, value: u64 }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ProgLineParseError {
    BadFormat,
    BadMask(MaskParseError)
}

impl From<MaskParseError> for ProgLineParseError {
    fn from(e: MaskParseError) -> Self {
        Self::BadMask(e)
    }
}

impl From<ParseIntError> for ProgLineParseError {
    fn from(_e: ParseIntError) -> Self {
        Self::BadFormat
    }
}

impl FromStr for ProgLine {
    type Err = ProgLineParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(mask_str) = s.strip_prefix("mask = ") {
            return Ok(Self::SetMask(mask_str.parse()?))
        }
        
        if !s.starts_with("mem[") {
            return Err(ProgLineParseError::BadFormat);
        }
        
        let index_end = s.find(']')
            .ok_or(ProgLineParseError::BadFormat)?;

        let value_start = s.find('=')
            .ok_or(ProgLineParseError::BadFormat)?;
        let value_start = value_start + 2;

        let index: usize = s["mem[".len()..index_end].parse()?;
        let value: u64 = s[value_start..].parse()?;
        
        Ok(Self::SetMem { index, value })
    }
}

fn part_1(prog: &[ProgLine]) -> u64 {
    let mut mem = HashMap::<usize, u64>::new();
    let mut mask = Mask {
        bits: [MaskBit::X; 36]
    };
    
    for line in prog {
        match line {
            ProgLine::SetMask(m) => mask = *m,
            ProgLine::SetMem { index, value } => {
                mem.insert(*index, mask.apply_1(*value));
            },
        }
    }
    
    mem.values().sum()
}

fn part_2(prog: &[ProgLine]) -> u64 {
    let mut mem = HashMap::<usize, u64>::new();
    let mut mask = Mask {
        bits: [MaskBit::Zero; 36]
    };
    
    for line in prog {
        match line {
            ProgLine::SetMask(m) => mask = *m,
            ProgLine::SetMem { index, value } => {
                for addr in mask.apply_2(*index) {
                    mem.insert(addr, *value);
                }
            },
        }
    }
    
    dbg!(mem.len());
    mem.values().sum()
}

fn main() {
    let prog = INPUT.lines()
        .map(ProgLine::from_str)
        .collect::<Result<Vec<_>, _>>()
        .expect("Expected a valid input");

    let sw = Instant::now();
    let part_1_ans = part_1(&prog);
    let part_1_time = sw.elapsed();

    let sw = Instant::now();

    let part_2_ans = part_2(&prog);
    let part_2_time = sw.elapsed();

    dbg!(part_1_ans);
    dbg!(part_2_ans);
    dbg!(part_1_time); // 200µs in release mode
    dbg!(part_2_time); // 6ms in release mode
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    const EXAMPLE_2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
    
    fn parse(s: &str) -> Vec<ProgLine> {
        s.lines()
            .map(ProgLine::from_str)
            .collect::<Result<Vec<_>, _>>()
            .expect("Expected valid input")
    }

    #[test]
    fn test_floating_iter() {
        let addr = 0b1001000_usize;
        let mut mask = Mask {
            bits: [MaskBit::Zero; 36]
        };
        
        assert_eq!(vec![0b1001000], mask.apply_2(addr).collect::<Vec<usize>>());
        
        mask.bits[0] = MaskBit::X;
        assert_eq!(vec![0b1001000, 0b1001001], mask.apply_2(addr).collect::<Vec<usize>>());

        mask.bits[2] = MaskBit::X;
        assert_eq!(
            vec![0b1001000, 0b1001001, 0b1001100, 0b1001101], 
            mask.apply_2(addr).collect::<Vec<usize>>()
        );
    }
    
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse(EXAMPLE_1)), 165);
        assert_eq!(part_1(&parse(INPUT)), 4886706177792);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse(EXAMPLE_2)), 208);
        assert_eq!(part_2(&parse(INPUT)), 3348493585827);
    }
}