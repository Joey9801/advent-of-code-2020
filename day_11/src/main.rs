use std::{str::FromStr, iter};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl Cell {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Cell::Floor),
            'L' => Some(Cell::EmptySeat),
            '#' => Some(Cell::OccupiedSeat),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Map {
    data: Vec<Cell>,
    width: usize,
    height: usize
}

#[derive(Clone, Copy, Debug)]
enum MapParseError {
    InvalidChar,
    NotRectanglular,
}

impl FromStr for Map {
    type Err = MapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.chars()
            .position(|c| c == '\n')
            .unwrap_or(s.chars().count());
        
        let data = s.chars()
            .filter_map(|c| match c {
                '\n' => None,
                c => Some(Cell::from_char(c))
            })
            .collect::<Option<Vec<_>>>()
            .ok_or(MapParseError::InvalidChar)?;
            
        if data.len() % width != 0 {
            Err(MapParseError::NotRectanglular)
        } else {
            let height = data.len() / width;
            Ok(Self {
                data,
                width,
                height,
            })
        }
    }
}

impl Map {
    fn get(&self, x: isize, y: isize) -> Option<Cell> {
        if x < 0 || y < 0 {
            None
        } else {
            let x = x as usize;
            let y = y as usize;

            if x < self.width && y < self.height {
                Some(self.data[y * self.width + x])
            } else {
                None
            }
        }
    }
    
    // Perform one step according to the rules in part 1, storing the result in the given buffer.
    fn step_1(&self, result: &mut Map) {
        assert_eq!(result.width, self.width);
        assert_eq!(result.height, self.height);

        result.data.clear();

        for y in 0..(self.height as isize) {
            for x in 0..(self.width as isize) {
                let adjacent_states = [
                    self.get(x - 1, y - 1),
                    self.get(x + 1, y - 1),
                    self.get(x,     y - 1),
                    self.get(x - 1, y + 1),
                    self.get(x + 1, y + 1),
                    self.get(x,     y + 1),
                    self.get(x - 1, y),
                    self.get(x + 1, y),
                ];
                
                let adjacent_occupied_count = adjacent_states
                    .iter()
                    .filter_map(|&c| c)
                    .filter(|&c| c == Cell::OccupiedSeat)
                    .count();

                let current_state = self.get(x, y).unwrap();
                
                let new_state = if current_state == Cell::EmptySeat && adjacent_occupied_count == 0 {
                    Cell::OccupiedSeat
                } else if current_state == Cell::OccupiedSeat && adjacent_occupied_count >= 4 {
                    Cell::EmptySeat
                } else {
                    current_state
                };
                
                result.data.push(new_state);
            }
        }
    }

    fn first_seat_in_dir(&self, x: usize, y: usize, step_x: isize, step_y: isize) -> Option<Cell> {
        if step_x == 0 && step_y == 0 {
            None
        } else {
            iter::successors(Some((x as isize, y as isize)), |(x, y)| Some((x + step_x, y + step_y)))
                .skip(1) // Skip the element at exactly (x, y)
                .map(|(x, y)| self.get(x, y))
                .filter(|c| *c != Some(Cell::Floor))
                .next().unwrap() // the initial iter::successors is infinite, so .next() will always return Some(Option<Cell>)
        }
    }

    // This method could run faster the seats to check were precomputed - the positions of the
    // seats never change, just just their occupancy, so iterating along each every line of sight
    // for every seat at every step is wasteful!
    // Precomputing the seats to check may allow the two step methods to be combined.
    fn step_2(&self, result: &mut Map) {
        assert_eq!(result.width, self.width);
        assert_eq!(result.height, self.height);

        result.data.clear();

        for y in 0..self.height {
            for x in 0..self.width {
                let seen_states = [
                    self.first_seat_in_dir(x, y, -1, -1),
                    self.first_seat_in_dir(x, y,  1, -1),
                    self.first_seat_in_dir(x, y,  0, -1),
                    self.first_seat_in_dir(x, y, -1, 1),
                    self.first_seat_in_dir(x, y,  1, 1),
                    self.first_seat_in_dir(x, y,  0, 1),
                    self.first_seat_in_dir(x, y, -1, 0),
                    self.first_seat_in_dir(x, y,  1, 0),
                ];
                
                let seen_occupied_count = seen_states
                    .iter()
                    .filter_map(|&c| c)
                    .filter(|&c| c == Cell::OccupiedSeat)
                    .count();

                let current_state = self.get(x as isize, y as isize).unwrap();
                
                let new_state = if current_state == Cell::EmptySeat && seen_occupied_count == 0 {
                    Cell::OccupiedSeat
                } else if current_state == Cell::OccupiedSeat && seen_occupied_count >= 5 {
                    Cell::EmptySeat
                } else {
                    current_state
                };
                result.data.push(new_state);
            }
        }
    }
}

fn settle(map: Map, step_fn: impl Fn(&Map, &mut Map)) -> usize {
    let mut tick = map.clone();
    let mut tock = map;
    let mut is_tick = true;
    
    // Take an intial step - tick and tock always start off the same.
    tock.step_1(&mut tick);

    while tick != tock {
        if is_tick {
            step_fn(&tick, &mut tock);
        } else {
            step_fn(&tock, &mut tick);
        }
        is_tick = !is_tick;
    }

    // Doesn't matter which one we pick, they're both the same now.
    tick.data.iter()
        .filter(|&&c| c == Cell::OccupiedSeat)
        .count()
}

fn part_1(map: Map) -> usize {
    settle(map, Map::step_1)
}

fn part_2(map: Map) -> usize {
    settle(map, Map::step_2)
}

fn real_input() -> Map {
    include_str!("../input.txt")
        .parse()
        .expect("Expected a valid input")
}

fn main() {
    let map = real_input();
    dbg!(part_1(map.clone()));
    dbg!(part_2(map));
}


#[cfg(test)]
mod tests {
    use super::*;
    
    fn example_input() -> Map {
        "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL".parse().expect("Expected a valid exapmle input")
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(example_input()), 37);
        assert_eq!(part_1(real_input()), 2418);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(example_input()), 26);
        assert_eq!(part_2(real_input()), 2144);
    }
}