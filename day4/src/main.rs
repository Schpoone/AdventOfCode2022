use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            let extremes: Vec<u32> = line
                .split(|c| c == '-' || c == ',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
            assert!(extremes.len() == 4);
            if overlaps(&extremes) {
                    count += 1;
            }
        }
    }
    println!("{}", count);
    Ok(())
}

// Part 1: how many pairs have one fully containing the other
fn fully_contains(x: &Vec<u32>) -> bool {
    x[0] >= x[2] && x[1] <= x[3] || x[0] <= x[2] && x[1] >= x[3]
}

// Part 2: how many pairs have overlapping ranges
fn overlaps(x: &Vec<u32>) -> bool {
    (x[2]..=x[3]).contains(&x[0]) || (x[2]..=x[3]).contains(&x[1])
        || (x[0]..=x[1]).contains(&x[2]) || (x[0]..=x[1]).contains(&x[3])
}
