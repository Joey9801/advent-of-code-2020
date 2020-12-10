use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn real_input() -> Vec<i64> {
    const MAX_INPUT_SIZE: usize = 100;
    let mut sorted_staging = [None; MAX_INPUT_SIZE * 3];
    let mut count = 0;
    for x in INPUT.lines().map(str::parse::<i64>) {
        let value = x.expect("Expected a valid input");
        sorted_staging[value as usize] = Some(value);
        count += 1;
    }
    
    assert!(count <= MAX_INPUT_SIZE);
    let mut input = Vec::with_capacity(count + 2);
    input.push(0);
    for x in sorted_staging.iter().filter_map(|x| *x) {
        input.push(x);
    }
    input.push(input.last().unwrap() + 3);
    
    input
}

fn part_1(input: &[i64]) -> i64 {
    let mut counts = [0i64; 4];
    input.windows(2)
        .map(|w| w[1] - w[0])
        .for_each(|x| counts[x as usize] += 1 );
    
    assert!(counts[0] == 0);

    counts[1] * counts[3]
}

fn part_2(input: &[i64]) -> i64 {
    const MAX_GAP: usize = 3;

    // For each element, for each of the precending elements that are within
    // MAX_GAP of this element, sum the number of ways that that preceding
    // element can be reached from the first element.
    // Since there are no repeats in the input, only the last MAX_GAP elements
    // can possibly be close enough => store the number of ways that element i
    // can be reached from the first element in counts[i % MAX_GAP].
    let mut counts = [0; MAX_GAP];
    
    // Initial condition - There is one way to get to first element from the
    // first element.
    counts[0] = 1;

    for i in 1..input.len() {
        let min_j = if i < MAX_GAP { 0 } else { i - MAX_GAP };
        counts[i % MAX_GAP] = (min_j..i)
            .filter(|&j| input[i] - input[j] <= 3)
            .map(|j| counts[j % MAX_GAP])
            .sum();
    }

    counts[(input.len() - 1) % MAX_GAP]
}

fn main() {
    let sw = Instant::now();
    let input = real_input();
    let load_input_time = sw.elapsed();
    
    let sw = Instant::now();
    let part_1_ans = part_1(&input);
    let part_1_time = sw.elapsed();

    let sw = Instant::now();
    let part_2_ans = part_2(&input);
    let part_2_time = sw.elapsed();
    
    dbg!(part_1_ans);
    dbg!(part_2_ans);
    dbg!(load_input_time);
    dbg!(part_1_time);
    dbg!(part_2_time);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Vec<i64> {
        let mut input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        input.push(0);
        input.sort();
        input.push(input.last().unwrap() + 3);
        input
    }

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