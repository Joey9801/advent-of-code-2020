use std::time::Instant;

// O(|numbers| * window_size ^ 2) solution
fn part_1(numbers: &[i64], window: usize) -> Option<i64> {
    
    // Does the given slice contain a pair of different numbers equal to the target
    fn contains_pair_sum(slice: &[i64], target: i64) -> bool {
        for x in 0..(slice.len() - 1) {
            if slice[x] * 2 == target {
                continue;
            }
            if slice[x..].contains(&(target - slice[x])) {
                return true;
            }
        }
        false
    }

    numbers.windows(window)
        .zip(numbers.iter().skip(window))
        .filter(|(window, target)| !contains_pair_sum(window, **target))
        .map(|(_, x)| *x)
        .next()
}

fn part_2(numbers: &[i64], target: i64) -> Option<i64> {
    assert!(numbers.len() >= 1);
    assert!(numbers[0] < target);

    let mut sum = numbers[0];
    let mut start = 0;
    let mut end = 0;
    loop {
        match sum.cmp(&target) {
            std::cmp::Ordering::Less => {
                end += 1;
                if end >= numbers.len() {
                    return None;
                }
                sum += numbers[end];
            }
            std::cmp::Ordering::Equal => {
                if (end - start + 1) >= 2 {
                    break;
                } else {
                    sum -= numbers[start];
                    start += 1;
                }
            }
            std::cmp::Ordering::Greater => {
                sum -= numbers[start];
                start += 1;
            }
        }
    }
    
    debug_assert!(end - start >= 1);
    debug_assert!(numbers[start..=end].iter().sum::<i64>() == target);
    
    let min = *numbers[start..=end].iter().min().unwrap();
    let max = *numbers[start..=end].iter().max().unwrap();
    Some(min + max)
}

fn main() {
    let sw = Instant::now();
    let input = include_str!("../input.txt")
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<i64>, _>>()
        .expect("Expected valid input");
    let parse_input_time = sw.elapsed();

    let sw = Instant::now();
    let part_1 = part_1(&input, 25).unwrap();
    let part_1_time = sw.elapsed();
    let sw = Instant::now();
    let part_2 = part_2(&input, part_1).unwrap();
    let part_2_time = sw.elapsed();

    let sw = Instant::now();
    println!("part_1 = {}", part_1);
    println!("part_2 = {}", part_2);
    let print_answers_time = sw.elapsed();

    dbg!(parse_input_time);
    dbg!(part_1_time);
    dbg!(part_2_time);
    dbg!(print_answers_time);
}


#[cfg(test)]
mod tests {
    use super::*;
    
    fn example_input() -> &'static [i64] {
        &[
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
        ]
    }
    
    fn real_input() -> Vec<i64> {
        include_str!("../input.txt")
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<i64>, _>>()
            .expect("A valid input")
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(example_input(), 5), Some(127));
        assert_eq!(part_1(&real_input(), 25), Some(756008079));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(example_input(), 127), Some(62));
        assert_eq!(part_2(&real_input(), 756008079), Some(93727241));
    }
}