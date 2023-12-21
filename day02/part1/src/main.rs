use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let REDS: u32 = 12;
    let GREENS: u32 = 13;
    let BLUES: u32 = 14;
    let mut total: u32 = 0;
    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.split(" ");
        let mut game_num: u32 = 0;
        let mut curr_num: u32 = 0;
        let mut game_good = true;
        for (idx, word) in line.enumerate() {
            // println!("idx: {}, {}", idx, word);
            if idx == 1 {
                game_num = word[..word.len()-1].parse::<u32>().unwrap(); // remove ; then parse u32
            }

            if idx > 1 {
                if let Ok(num) = word.parse::<u32>() {  // if its a number save it, next word is col
                    curr_num = num;
                    // println!("{}", curr_num)
                } else {  // check which colour it is and if bad the game is not added to total
                    if word.starts_with("r") {
                        if curr_num > REDS { game_good = false; break; }
                    }
                    else if word.starts_with("g") {
                        if curr_num > GREENS { game_good = false; break; }
                    }
                    else if word.starts_with("b") {
                        if curr_num > BLUES { game_good = false; break; } 
                    }
                }
            }
        }
            if game_good { total += game_num; }
    }
    println!("{}", total)
}
