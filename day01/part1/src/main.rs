use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()>{
    let mut total = 0;
    let mut first:Option<u32> = None;
    let mut last:Option<u32> = None;
    let path = "input.txt";
    let file = match File::open(path) {
        Err(_err) => return Ok(()),
        Ok(file) => file
    };
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = match line {
            Err(_) => return Ok(()),
            Ok(ans) => ans,
        };

        let mut found_first = false;
        let mut curr_num:Option<u32> = None;
        let mut iter = line.chars();
        while let Some(c) = iter.next() {
            if c.is_digit(10) {

                curr_num = c.to_digit(10);
                if !found_first {
                    first = curr_num;
                    found_first = true;
                }
            }
        }

        last = curr_num;
        let last = match last {
            None => return Ok(()),
            Some(num) => num,
        };
        let first = match first {
            None => return Ok(()),
            Some(num) => num,
        };
        total += last + first*10;
    }
    println!("{}", total);
    Ok(())
}
