use std::{time::Instant, collections::{HashMap, HashSet}};

const INPUT: &str = ".###.###
.#.#...#
..##.#..
..##..##
........
##.#.#.#
..###...
.####...";

fn real_input() -> HashSet<Coord> {
    let mut active = HashSet::new();
    let mut x = 0;
    let mut y = 0;

    for ch in INPUT.chars() {
        match ch {
            '#' => { active.insert(Coord::new_slice(x, y)); },
            '.' => (),
            '\n' => (),
            _ => panic!("Bad char in input: '{}'", ch)
        }
        
        match ch {
            '\n' => {
                x = 0;
                y += 1;
            },
            _ => x += 1,
        }
    }

    active
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord {
    storage: [i32; 4],
}

impl Coord {
    fn new_slice(x: i32, y: i32) -> Self {
        Self {
            storage: [x, y, 0, 0],
        }
    }

    fn neighbours(&self, dimension: u8) -> impl Iterator<Item=Coord> {
        let mut first = self.clone();
        for i in 0..(dimension as usize) {
            first.storage[i] -= 1;
        }

        NeighbourIter {
            base: *self,
            dimension,
            next: Some(first),
        }
    }
}

struct NeighbourIter {
    base: Coord,
    dimension: u8,
    next: Option<Coord>,
}

impl Iterator for NeighbourIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        let to_return = self.next.take();
        if let Some(c) = &to_return {
            let mut next = c.clone();
            next.storage[0] += 1;
            for d in 0..(self.dimension as usize - 1) {
                if next.storage[d] > self.base.storage[d] + 1 {
                    next.storage[d] = self.base.storage[d] - 1;
                    next.storage[d + 1] += 1;
                }
            }
            
            let last_idx = self.dimension as usize - 1;
            if next.storage[last_idx] > self.base.storage[last_idx] + 1 {
                self.next = None;
            } else {
                self.next = Some(next);
            }
        }
        
        // Skip over the base element if it was about to be returned
        if to_return == Some(self.base) {
            self.next()
        } else {
            to_return
        }
    }
}

fn solve(mut active_set: HashSet<Coord>, dimension: u8) -> usize {
    assert!(dimension as usize <= Coord::new_slice(0, 0).storage.len());

    fn step(active_set: &mut HashSet<Coord>, scratch: &mut HashMap<Coord, (bool, u8)>, dimension: u8) {
        // Copy over the contents of the active_set to the scratch map
        scratch.clear();
        for c in active_set.iter() {
            scratch.insert(*c, (true, 0));
        }
        
        // Count up the neighbours for each of the active nodes
        for c in active_set.iter().flat_map(|c| c.neighbours(dimension)) {
            scratch.entry(c)
                .or_insert((false, 0)).1 += 1;
        }
        
        // For each element in the scratch map that meets the rule, mark it as active in the main set
        active_set.clear();
        for (coord, (prev_state, active_neighbours)) in scratch.iter() {
            match (prev_state, active_neighbours) {
                (true, 2) | (true, 3) | (false, 3) => { active_set.insert(*coord); },
                _ => (),
            }
        }
    }
    
    let mut scratch = HashMap::with_capacity(active_set.capacity());
    for _ in 0..6 {
        step(&mut active_set, &mut scratch, dimension);
    }
    
    active_set.len()
}

fn main() {
    let input = real_input();

    let sw = Instant::now();
    let part_1_ans = solve(input.clone(), 3);
    let part_1_time = sw.elapsed();

    let sw = Instant::now();
    let part_2_ans = solve(input.clone(), 4);
    let part_2_time = sw.elapsed();
    
    dbg!(part_1_ans); // 384
    dbg!(part_2_ans); // 2012
    dbg!(part_1_time); // 1.2ms
    dbg!(part_2_time); // 24ms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(solve(real_input(), 3), 384);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve(real_input(), 4), 2012);
    }
}
