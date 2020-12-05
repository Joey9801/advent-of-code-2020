use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");


struct Seat {
    row: u8,
    col: u8,
}

impl Seat {
    fn id(&self) -> u16 {
        self.row as u16 * 8 + self.col as u16
    }
}

#[derive(Clone, Copy, Debug)]
enum SeatParseErr {
    NotAscii,
    WrongLength,
    InvalidChar,
}

impl FromStr for Seat {
    type Err = SeatParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_binary_str(s: &[u8], one: u8, zero: u8) -> Result<u8, SeatParseErr> {
            if s.iter().any(|c| *c != one && *c != zero) {
                Err(SeatParseErr::InvalidChar)
            } else {
                Ok(s.iter()
                    .rev()
                    .map(|c| match c {
                        x if *x == one => 1u8,
                        x if *x == zero => 0u8,
                        _ => unreachable!()
                    })
                    .enumerate()
                    .map(|(i, x)| x << i)
                    .sum()
                )
            }
        }

        if !s.is_ascii() {
            Err(SeatParseErr::NotAscii)
        } else if s.len() != 10 {
            Err(SeatParseErr::WrongLength)
        } else {
            let s = s.as_bytes();
            let row = parse_binary_str(&s[..7], b'B', b'F')?;
            let col = parse_binary_str(&s[7..], b'R', b'L')?;
            Ok(Self {
                row,
                col,
            })
        }
    }
}


fn main() {
    let mut all_ids = INPUT.lines()
        .map(Seat::from_str)
        .map(Result::unwrap)
        .map(|s| s.id())
        .collect::<Vec<_>>();
    
    println!("Maximum seat id: {}", all_ids.iter().max().unwrap());
        
    // Find the first pair of adjacent entries that are exactly two apart
    all_ids.sort();
    let adjacent_seats = all_ids.iter()
        .zip(all_ids.iter().skip(1))
        .filter(|(a, b)| (*a + 2) == **b)
        .next()
        .expect("Expected a solution to the puzzle");
    debug_assert!(adjacent_seats.0 + 2 == *adjacent_seats.1);
    println!("Our seat id: {}", adjacent_seats.0 + 1);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_parser() {
        // Examples from the problem statement
        let seat: Seat = "BFFFBBFRRR".parse().unwrap();
        assert_eq!(seat.row, 70);
        assert_eq!(seat.col, 7);

        let seat: Seat = "FFFBBBFRRR".parse().unwrap();
        assert_eq!(seat.row, 14);
        assert_eq!(seat.col, 7);

        let seat: Seat = "BBFFBBFRLL".parse().unwrap();
        assert_eq!(seat.row, 102);
        assert_eq!(seat.col, 4);
    }
}