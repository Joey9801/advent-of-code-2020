use std::time::Instant;

use util::math::modinverse;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Debug)]
enum Bus {
    OutOfService,
    InService(u64)
}

#[derive(Clone, Debug)]
struct Input {
    arrival_time: u64,
    busses: Vec<Bus>,
}

fn real_input() -> Input {
    let mut lines = INPUT.lines();
    let arrival_time = lines
        .next()
        .expect("Expected a valid input")
        .parse()
        .expect("Expected a valid input");

    let busses = lines
        .next()
        .expect("Expected a valid input")
        .split(',')
        .map(|s| match s {
            "x" => Bus::OutOfService,
            other => Bus::InService(other.parse().expect("Expected a valid input"))
        }).collect();

    Input {
        arrival_time,
        busses,
    }
}


fn part_1(input: &Input) -> u64 {
    let min_bus = input.busses
        .iter()
        .filter_map(|b| match b {
            Bus::OutOfService => None,
            Bus::InService(loop_duration) => Some(loop_duration)
        })
        .min_by_key(|&d| d - (input.arrival_time % d))
        .expect("Expected a solution to part 1");
        
    min_bus * (min_bus - (input.arrival_time % min_bus))
}

fn part_2(input: &Input) -> u128 {
    // If bus i is in service with a 'bus_time' of ni, it adds the following constraint to the
    // solution, x:
    //     x (mod ni) = -i (mod ni)
    //
    // prod = product of all ni
    // ai = -i (mod ni)
    // yi = prod / ni
    // yi * zi = 1 (mod ni)
    // x = sum(a * y * z) (mod prod)
    //
    // Satisfies constraint i:
    //    for j != i, yj (mod ni) = 0
    //    => sum(a * y * z) (mod ni) = ai * yi * zi (mod ni)
    //                                  = ai (mod ni)

    let prod: u128 = input.busses.iter().filter_map(|&bus| match bus {
        Bus::OutOfService => None,
        Bus::InService(time) => Some(time as u128),
    }).product();
    
    let sum: u128 = input.busses
        .iter()
        .enumerate()
        .filter_map(|(id, &bus)| match bus {
            Bus::OutOfService => None,
            Bus::InService(bus_time) => {
                let a = (-(id as i64)).rem_euclid(bus_time as i64) as u128;
                let n = bus_time as u128;
                Some((a, n))
            }
        })
        .map(|(a, n)| {
            let y = prod / n;
            let z = modinverse(y as i128, n as i128).unwrap() as u128;
            a * y * z
        }).sum();
    
    sum  % prod
}

fn main() {
    let input = real_input();

    let sw = Instant::now();
    let part_1_ans = part_1(&input);
    let part_1_time = sw.elapsed();
    
    let sw = Instant::now();
    let part_2_ans = part_2(&input);
    let part_2_time = sw.elapsed();

    dbg!(part_1_ans);
    dbg!(part_2_ans);
    dbg!(part_1_time);
    dbg!(part_2_time);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Input {
        Input {
            arrival_time: 939,
            busses: vec![
                Bus::InService(7),
                Bus::InService(13),
                Bus::OutOfService,
                Bus::OutOfService,
                Bus::InService(59),
                Bus::OutOfService,
                Bus::InService(31),
                Bus::InService(19),
            ]
        }
    }
    
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&example_input()), 295);
        assert_eq!(part_1(&real_input()), 5257);
    }

    
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&example_input()), 1068781);
        assert_eq!(part_2(&real_input()), 538703333547789);
    }
}
