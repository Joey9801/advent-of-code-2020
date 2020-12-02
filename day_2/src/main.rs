const INPUT: &str = include_str!("../input.txt");

fn is_valid_1(line: &str) -> bool {
    debug_assert!(line.is_ascii());
    let dash = line.find('-').unwrap();
    let space = line.find(' ').unwrap();
    let colon = line.find(':').unwrap();
    
    let min: usize = line[0..dash].parse().unwrap();
    let max: usize = line[(dash + 1)..space].parse().unwrap();
    let letter = line[(space+1)..(space+2)].chars().next().unwrap();
    let password = &line[(colon + 2)..];
    
    let count = password.chars()
        .filter(|c| *c == letter)
        .count();

    (min..(max + 1)).contains(&count)
}

fn is_valid_2(line: &str) -> bool {
    debug_assert!(line.is_ascii());
    let dash = line.find('-').unwrap();
    let space = line.find(' ').unwrap();
    let colon = line.find(':').unwrap();
    
    let idx_1: usize = line[0..dash].parse().unwrap();
    let idx_2: usize = line[(dash + 1)..space].parse().unwrap();
    let letter = line[(space+1)..(space+2)].chars().next().unwrap();
    let password = &line[(colon + 2)..];
    
    let get_char = |idx| password.chars().skip(idx - 1).next();
    (get_char(idx_1) == Some(letter)) ^ (get_char(idx_2) == Some(letter))
}

fn main() {
    let valid_count = INPUT
        .lines()
        .filter(|l| is_valid_2(*l))
        .count();

    println!("There are {} valid passwords", valid_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2_is_valid_1() {
        assert!(is_valid_1("1-3 a: abcde"));
        assert!(!is_valid_1("1-3 b: cdefg"));
        assert!(is_valid_1("2-9 c: ccccccccc"));
    }
    
    #[test]
    fn test_day_2_is_valid_2() {
        assert!(is_valid_2("1-3 a: abcde"));
        assert!(!is_valid_2("1-3 b: cdefg"));
        assert!(!is_valid_2("2-9 c: ccccccccc"));
    }
    
    #[test]
    fn test_day_2_part_1() {
        let valid_count = INPUT
            .lines()
            .filter(|l| is_valid_1(*l))
            .count();
        assert_eq!(valid_count, 524);
    }
    
    #[test]
    fn test_day_2_part_2() {
        let valid_count = INPUT
            .lines()
            .filter(|l| is_valid_2(*l))
            .count();
        assert_eq!(valid_count, 485)
    }
}