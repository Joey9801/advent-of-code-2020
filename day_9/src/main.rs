// Super dumb N^3 brute force solution - The input is small enough to fit in ~10% of the L1 cache of a single core on my machine
fn part_1(numbers: &[usize], window: usize) -> Option<usize> {
    fn contains_sum(numbers: &[usize], target: usize) -> bool {
        for x in 0..(numbers.len() - 1) {
            for y in (x + 1)..numbers.len() {
                let x= numbers[x];
                let y= numbers[y];
                if x != y && x + y == target {
                    return true;
                }
            }
        }
        false
    }

    numbers.windows(window)
        .zip(numbers.iter().skip(window))
        .filter(|(window, target)| !contains_sum(window, **target))
        .map(|(_, x)| *x)
        .next()
}

fn part_2(numbers: &[usize], target: usize) -> Option<usize> {
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
    
    assert!(end - start >= 1);
    assert!(numbers[start..=end].iter().sum::<usize>() == target);
    
    let min = *numbers[start..=end].iter().min().unwrap();
    let max = *numbers[start..=end].iter().max().unwrap();
    Some(min + max)
}

fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<usize>, _>>()
        .expect("A valid input");

    let part_1 = dbg!(part_1(&input, 25)).unwrap();
    dbg!(part_2(&input, part_1));
}


#[cfg(test)]
mod tests {
    use super::*;
    
    fn example_input() -> &'static [usize] {
        &[
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
        ]
    }
    
    fn real_input() -> Vec<usize> {
        include_str!("../input.txt")
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<usize>, _>>()
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