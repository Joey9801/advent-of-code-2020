fn input() -> Vec<u64> {
    let input = include_str!("../input.txt");
    input.lines()
        .map(|l| l.trim().parse().expect(&format!("{} wasn't a valid u64", l)))
        .collect::<Vec<u64>>()
}

fn part_1(numbers: &[u64]) -> Option<u64> {
    for i in 0..(numbers.len() - 2) {
        for j in (i+1)..(numbers.len() - 1) {
            let x = numbers[i];
            let y = numbers[j];
            if x + y == 2020 {
                return Some(x * y);
            }
        }
    }
    
    None
}

fn part_2(numbers: &[u64]) -> Option<u64> {
    for i in 0..(numbers.len() - 2) {
        for j in (i+1)..(numbers.len() - 1) {
            for k in (j+1)..numbers.len() {
                let x = numbers[i];
                let y = numbers[j];
                let z = numbers[k];
                if x + y + z == 2020 {
                    return Some(x * y * z);
                }
            }
        }
    }
    
    None
}

fn main() {
    let input = input();
    println!("part 1 => {}", part_1(&input).unwrap());
    println!("part 2 => {}", part_2(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1() {
        let input = input();
        assert_eq!(part_1(&input).expect("Found no solution to part 1"), 877971);
        assert_eq!(part_2(&input).expect("Found no solution to part 2"), 203481432);
    }
}