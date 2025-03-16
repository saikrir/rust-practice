use std::{str::FromStr, string::ParseError};

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

impl FromStr for Isbn {
    type Err = String; // TODO: replace with appropriate type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits: Vec<u8> = Vec::with_capacity(13);

        for (_i, c) in s.char_indices() {
            match c {
                '-' => continue,
                '0'..='9' => digits.push(c.to_digit(10).unwrap() as u8),
                _ => return Err("Bad char".to_string()),
            }
        }

        let last_digit = digits[12];

        if last_digit != calculate_check_digit(&digits) {
            return Err("Invalid val".to_string());
        }

        Ok(Self {
            raw: s.to_string(),
            digits,
        })
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    let mut sum = 0;
    for (i, d) in digits.iter().enumerate() {
        let weight = if i % 2 == 0 { 1 } else { 3 };
        sum += weight * (*d);
    }

    let rem = sum % 10;

    (10 - rem) % 10
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("Rust in Action's ISBN-13 ({})is valid!", rust_in_action);
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}
