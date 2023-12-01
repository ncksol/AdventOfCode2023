use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn string_to_digit_mapping() -> HashMap<&'static str, i32> {
    let mut map = HashMap::new();
    map.insert("zero", 0);
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);
    map
}

fn extract_and_combine_digits(s: &str) -> i32 {
    let digit_map = string_to_digit_mapping();
    let mut first_digit_found = false;
    let mut first_digit = 0;
    let mut last_digit = 0;

    for (i, c) in s.chars().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            if !first_digit_found {
                first_digit = digit as i32;
                first_digit_found = true;
            }
            last_digit = digit as i32;
        } else {
            for (word, &digit) in &digit_map {
                if s[i..].starts_with(word) {
                    if !first_digit_found {
                        first_digit = digit;
                        first_digit_found = true;
                    }
                    last_digit = digit;
                    break;
                }
            }
        }
    }

    if first_digit_found {
        first_digit * 10 + last_digit
    } else {
        0
    }
}

fn main() -> io::Result<()> {
    let path = Path::new("src/input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let sum: i32 = reader
        .lines()
        .filter_map(Result::ok)
        .map(|s| extract_and_combine_digits(&s))
        .sum();

    println!("Total sum: {}", sum);
    Ok(())
}
