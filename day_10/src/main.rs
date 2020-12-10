const INPUT: &str = include_str!("../input.txt");

fn real_input() -> Vec<i64> {
    let mut input = INPUT.lines()
        .map(str::parse)
        .collect::<Result<Vec<i64>, _>>()
        .expect("Expected a valid input");

    // Sort the numbers, and include the socket and phone
    input.push(0);
    input.sort();
    input.push(input.last().unwrap() + 3);
        
    input
}

fn example_input() -> Vec<i64> {
    let mut input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    input.push(0);
    input.sort();
    input.push(input.last().unwrap() + 3);
    input
}

fn part_1(input: &[i64]) -> i64 {
    let mut one_count = 0;
    let mut three_count = 0;
    input.iter()
        .zip(input.iter().skip(1))
        .for_each(|(a, b)| {
            match b - a {
                0 => panic!("Two equal numbers!"),
                1 => one_count += 1,
                3 => three_count += 1,
                x if x > 3 => panic!("Gap of more than three"),
                _ => (),
            }
        });
    
    one_count * three_count
}

fn part_2(input: &[i64]) -> i64 {
    const DYN_SIZE: usize = 3;
    // There are no repeats in the inputs, so only ever have to look three forward to see all
    // possible paths
    let mut counts = [0; DYN_SIZE];
    
    // There is one way to get to the end from the end
    counts[(input.len() - 1) % DYN_SIZE] = 1;

    for i in (0..(input.len() - 1)).rev() {
        let value = input[i];
        let max_i = std::cmp::min(i + DYN_SIZE + 1, input.len());
        let mut count = 0i64;
        for j in (i + 1)..max_i {
            if input[j] - value <= 3 {
                count += counts[j % DYN_SIZE];
            }
        }
        counts[i % DYN_SIZE] = count;
    }

    counts[0]
}

fn main() {
    let input = real_input();
    dbg!(part_1(&input));
    dbg!(part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&example_input()), 35);
        assert_eq!(part_1(&real_input()), 1984);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&example_input()), 8);
        assert_eq!(part_2(&real_input()), 3543369523456);
    }
}