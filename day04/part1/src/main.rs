use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let words = line.split_whitespace().filter(|&s| !s.is_empty());
        let mut winners: HashSet<&str> = HashSet::new();
        let mut in_wins = true;
        let mut wins: u32 = 0;
        for (idx, ch) in words.enumerate() {
            if idx <= 1 {
                continue;
            }

            if ch == "|" {
                in_wins = false;
                continue;
            }

            if in_wins {
                winners.insert(ch);
            } else {
                if winners.contains(ch) {
                    wins += 1;
                }
            }
        }
        println!("wins: {}", wins);
        if wins > 0 {
            println!("add: {}", 2_u32.pow(wins - 1));
            total += 2_u32.pow(wins - 1)
        }
    }
    println!("{}", total)
}
