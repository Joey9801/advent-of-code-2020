const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PassportParseError {
    MissingField,
    UnknownField,
    ParseIntError,
}

impl From<std::num::ParseIntError> for PassportParseError {
    fn from(_: std::num::ParseIntError) -> Self {
        Self::ParseIntError
    }
}

struct PassportBuilder<'a> {
    birth_year: Option<usize>,
    issue_year: Option<usize>,
    expiration_year: Option<usize>,
    height: Option<&'a str>,
    hair_color: Option<&'a str>,
    eye_color: Option<&'a str>,
    passport_id: Option<&'a str>,
    country_id: Option<&'a str>
}

impl<'a> PassportBuilder<'a> {
    fn new() -> Self {
        Self {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }
    
    fn build(self) -> Result<Passport<'a>, PassportParseError> {

        if let PassportBuilder {
            birth_year: Some(birth_year),
            issue_year: Some(issue_year),
            expiration_year: Some(expiration_year),
            height: Some(height),
            hair_color: Some(hair_color),
            eye_color: Some(eye_color),
            passport_id: Some(passport_id),
            country_id
        } = self {
            Ok(Passport {
                birth_year,
                issue_year,
                expiration_year,
                height,
                hair_color,
                eye_color,
                passport_id,
                country_id,
            })
        } else {
            Err(PassportParseError::MissingField)
        }

    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Passport<'a> {
    birth_year: usize,
    issue_year: usize,
    expiration_year: usize,
    height: &'a str,
    hair_color: &'a str,
    eye_color: &'a str,
    passport_id: &'a str,
    country_id: Option<&'a str>
}

impl<'a> Passport<'a> {
    /// Parses a single passport string, returning None if invalid
    pub fn parse(s: &'a str) -> Result<Self, PassportParseError> {
        assert!(s.is_ascii());

        let mut builder = PassportBuilder::new();
        for part in s.split_whitespace() {
            assert!(part.as_bytes()[3] == b':');
            let key = &part[0..3];
            let value = &part[4..];
            
            match key {
                "byr" => builder.birth_year = Some(value.parse()?),
                "iyr" => builder.issue_year = Some(value.parse()?),
                "eyr" => builder.expiration_year = Some(value.parse()?),
                "hgt" => builder.height = Some(value),
                "hcl" => builder.hair_color = Some(value),
                "ecl" => builder.eye_color = Some(value),
                "pid" => builder.passport_id = Some(value),
                "cid" => builder.country_id = Some(value),
                _ => return Err(PassportParseError::UnknownField),
            }
        }
        
        builder.build()
    }
}

fn main() {
    let valid_passport_count = INPUT.split("\n\n")
        .map(Passport::parse)
        .filter(Result::is_ok)
        .count();
    
    println!("There were {} valid passport definitions", valid_passport_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passport_parse_1() {
        let input = "
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm
        ";
        assert_eq!(Passport::parse(input), Ok(Passport {
            eye_color: "gry",
            passport_id: "860033327",
            expiration_year: 2020,
            hair_color: "#fffffd",
            birth_year: 1937,
            issue_year: 2017,
            country_id: Some("147"),
            height: "183cm",
        }));
    }

    #[test]
    fn test_passport_parse_2() {
        let input = "
            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929
        ";
        assert_eq!(Passport::parse(input), Err(PassportParseError::MissingField));
    }

    #[test]
    fn test_passport_parse_3() {
        let input = "
            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm
        ";
        assert_eq!(Passport::parse(input), Ok(Passport {
            eye_color: "brn",
            passport_id: "760753108",
            expiration_year: 2024,
            hair_color: "#ae17e1",
            birth_year: 1931,
            issue_year: 2013,
            country_id: None,
            height: "179cm",
        }));
    }
}