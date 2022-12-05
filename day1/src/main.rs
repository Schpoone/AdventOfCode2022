use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    const TOP_N: usize = 3;

    let mut sum = 0;
    let mut max: [u32; TOP_N] = [0; TOP_N]; // sorted greatest to least
    for line in reader.lines() {
        if let Ok(line) = line {
            match line.parse::<u32>() {
                Ok(n) => {
                    sum += n;
                }
                Err(_) => {
                    for max_item in max.iter_mut() {
                        if sum > *max_item {
                            let tmp = *max_item;
                            *max_item = sum;
                            sum = tmp;
                        }
                    }
                    sum = 0;
                }
            }
        }
    }
    println!("{}", max.iter().sum::<u32>());
    Ok(())
}
