use std::fs::File;
use std::io::{self, read_to_string, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let s = read_to_string(reader)?;
    println!("Answer: {}", start_marker_index(&s, 14).unwrap());

    Ok(())
}

fn start_marker_index(s: &str, size: usize) -> Option<usize> {
    let mut window: HashMap<char, usize> = HashMap::new();
    // Populate window with first few characters
    for c in s.chars().take(size-1) {
        match window.get(&c) {
            Some(n) => window.insert(c, n+1),
            None => window.insert(c, 1)
        };
    }

    // Check for repeats with sliding window
    let mut right_chars = s.chars().enumerate().skip(size-1);
    let mut left_chars = s.chars();
    while window.len() < size {
        //insert into window using right_chars
        let c = right_chars.next();
        if let Some((i, c)) = c {
            match window.get(&c) {
                Some(n) => window.insert(c, n+1),
                None => window.insert(c, 1)
            };
            if window.len() >= size {
                return Some(i+1);
            }
        } else {
            break;
        }

        //pop from window using left_chars
        let c = left_chars.next().unwrap();
        let repeats = window.get(&c).unwrap();
        if *repeats == 1usize {
            window.remove(&c);
        } else {
            window.insert(c, repeats - 1);
        }

    }
    return None;
}
