use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn find_numbers(text: &str) -> Option<(u32, u32)>{
    let pattern = r"\d|(?:one|two|three|four|five|six|seven|eight|nine)";
    let re = Regex::new(pattern).expect("Failed to compile regex");
    let pattern_backward = r"\d|(?:eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)";
    let re_backward = Regex::new(pattern_backward).expect("Failed to compile backward regex");

    let start_word: &str = re
        .find(text)
        .map(|m| m.as_str())
        .unwrap();

    let binding = reverse_string(text);
    let text = binding.as_str();

    let end_word: &str = re_backward
        .find(text)
        .map(|m| m.as_str())
        .unwrap();

    let start_num = {
        if let Ok(num) = start_word.parse::<u32>() {
            Some(num)   
        } else {
            match start_word.to_lowercase().as_str() {
                "one" => Some(1),
                "two" => Some(2),
                "three" => Some(3),
                "four" => Some(4),
                "five" => Some(5),
                "six" => Some(6),
                "seven" => Some(7),
                "eight" => Some(8),
                "nine" => Some(9),
                _ => None,
            }
        }
    };

    let end_num = {
        if let Ok(num) = end_word.parse::<u32>() {
            Some(num)   
        } else {
            match end_word.to_lowercase().as_str() {
                "eno" => Some(1),
                "owt" => Some(2),
                "eerht" => Some(3),
                "ruof" => Some(4),
                "evif" => Some(5),
                "xis" => Some(6),
                "neves" => Some(7),
                "thgie" => Some(8),
                "enin" => Some(9),
                _ => None,
            }
        }
    };

    if start_num != None {
    Some((start_num.unwrap(), end_num.unwrap()))
    } else {
        None
    }
}


fn main() {
    let mut total = 0;
    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();

        let (first, last) = find_numbers(line.as_str()).unwrap();
        total += first*10 + last;
        // println!("line: {}, first: {}, last: {}, sum: {}, total: {}", line, first, last, last + first*10, total)
    }
    println!("{}", total)
}
