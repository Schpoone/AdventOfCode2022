use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;

    //part_1(&file);
    part_2(&file);

    Ok(())
}

// For part 1: find item in common between 2 compartments
fn part_1(file: &File) {
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            let length = line.chars().count();
            assert!(length % 2 == 0);
            let mut partitions = Vec::new();
            let part1 = &line[..length/2];
            partitions.push(part1
                            .chars()
                            .collect());
            let part2 = &line[length/2..];
            partitions.push(part2
                            .chars()
                            .collect());
            let c = find_common_char(&partitions);
            sum += char_to_prio(c);
        }
    }
    println!("Total: {}", sum);
}

// For part 2: find item in common between all 3 elves
fn part_2(file: &File) {
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut partitions = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            partitions.push(line.chars().collect());
            if i % 3 == 2 {
                let c = find_common_char(&partitions);
                sum += char_to_prio(c);
                partitions.clear();
            }
        }
    }
    println!("Total: {}", sum);
}

fn find_common_char(partitions: &Vec<HashSet<char>>) -> char {
    assert!(partitions.len() > 0);
    for c in &partitions[0] {
        if partitions.into_iter().all(|p| p.contains(&c)) {
            return *c;
        }
    }
    panic!();
}

fn char_to_prio(c: char) -> u32 {
    assert!(c.is_ascii_alphabetic());
    if c.is_ascii_lowercase() {
        let c = c as u32;
        c - 96
    } else if c.is_ascii_uppercase() {
        let c = c as u32;
        c - 38
    } else {
        panic!();
    }
}
