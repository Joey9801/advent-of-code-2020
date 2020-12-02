const INPUT: &str = include_str!("../input.txt");

fn is_valid(line: &str) -> bool {
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

fn main() {
    let valid_count = INPUT
        .lines()
        .filter(|l| is_valid(*l))
        .count();

    println!("There are {} valid passwords", valid_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2_checker() {
        assert!(is_valid("1-3 a: abcde"));
        assert!(!is_valid("1-3 b: cdefg"));
        assert!(is_valid("2-9 c: ccccccccc"));
    }
}