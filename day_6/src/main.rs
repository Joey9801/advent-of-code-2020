use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Debug)]
struct SurveySet {
    /// The count of yes answers for each question
    answers: [u32; 26],
    
    /// The number of participants in the group
    group_size: u32,
}

impl SurveySet {
    fn any_yes_count(&self) -> u32 {
        self.answers
            .iter()
            .map(|a| if *a > 0 { 1 } else { 0 })
            .sum()
    }

    fn all_yes_count(&self) -> u32 {
        self.answers
            .iter()
            .map(|a| if *a == self.group_size { 1 } else { 0 })
            .sum()
    }
}

#[derive(Clone, Copy, Debug)]
enum SurveySetParseError {
    InvalidChar
}

impl FromStr for SurveySet {
    type Err = SurveySetParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = SurveySet {
            answers: [0; 26],
            group_size: 1,
        };

        for c in s.chars() {
            match c {
                c if c.is_ascii_lowercase() => {
                    let question = c as u8 - b'a';
                    set.answers[question as usize] += 1;
                },
                '\n' => set.group_size += 1,
                _ => return Err(SurveySetParseError::InvalidChar)
            }
        }
        
        Ok(set)
    }
}

fn main() {
    let results  = INPUT.split("\n\n")
        .map(SurveySet::from_str)
        .collect::<Result<Vec<_>, _>>()
        .expect("Valid problem input");

    let part_1: u32 = results
        .iter()
        .map(|s| s.any_yes_count())
        .sum();

    let part_2: u32 = results
        .iter()
        .map(|s| s.all_yes_count())
        .sum();
    
    println!("Sum for part 1: {}", part_1);
    println!("Sum for part 2: {}", part_2);
}
