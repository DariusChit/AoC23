use std::{collections:: HashSet, io::{BufRead, BufReader}, fs::File, usize};


fn main() {
    let symbols: HashSet<char> = HashSet::from_iter(vec!['!','/', '#', '$', '%', '^', '&', '*', '@', '\\', '-', '_', '=', '+']);
    let file = File::open("input.txt").expect("file does not exist");
    let reader = BufReader::new(file);
    let mut total: u32 = 0;

    let mut lines = reader.lines();

    let mut curr_line = lines.next().unwrap().unwrap();
    let mut prev_line = String::from("");
    let mut next_line = lines.next().unwrap().unwrap();
    
    let mut in_num = false;
    let mut num = 0;
    let mut start_idx = 0;
    let mut end_idx: usize;

    loop {
        if curr_line.eq("") { break; }
        for (idx, c) in curr_line.char_indices() {
            if c.is_numeric() {
                if !in_num {
                    in_num = true;
                    start_idx = if idx == 0 {0} else {idx-1};
                    num = c.to_digit(10).unwrap();
                } else {
                    num = num*10 + c.to_digit(10).unwrap();
                }
            } else {
                if in_num {
                    in_num = false;
                    end_idx = if idx == curr_line.len()-1 {idx} else {idx+1};
                    let top = prev_line.get(start_idx..end_idx).unwrap_or_else(|| "");
                    let mid = curr_line.get(start_idx..end_idx).unwrap_or_else(|| "");
                    let bot = next_line.get(start_idx..end_idx).unwrap_or_else(|| "");
                    let surround = top.to_owned() + mid + bot;
                    if surround.chars().any(|ch| symbols.contains(&ch)) { total += num; println!("n: {}, t: {}", num, total) }
                    num = 0;
                }
            }
        }
        prev_line = curr_line;
        curr_line = next_line;
        next_line = match lines.next() {
            Some(Ok(l)) => l,
            _ => String::from(""),
        };
    }
    println!("{}", total)
}
