use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut total: u32 = 0;
    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.split(" ");
        let mut curr_num: u32 = 0;
        let (mut max_r, mut max_g, mut max_b) = (0, 0, 0);
        for (idx, word) in line.enumerate() {
            if idx > 1 {
                if let Ok(num) = word.parse::<u32>() {  // if its a number save it, next word is col
                    curr_num = num;
                } else {  // check which colour it is and if bad the game is not added to total
                    if word.starts_with("r") {
                        if curr_num > max_r {
                            max_r = curr_num;
                        }
                    }
                    else if word.starts_with("g") {
                        if curr_num > max_g {
                            max_g = curr_num;
                        }
                    }
                    else if word.starts_with("b") {
                        if curr_num > max_b {
                            max_b = curr_num;
                        }
                    }
                }
            }
        }
        total += max_g*max_r*max_b;
        println!("{}, {}, {}", max_r, max_g, max_b)
    }
    println!("{}", total)
}
