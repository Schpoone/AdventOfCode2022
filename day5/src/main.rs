use std::fs::File;
use std::io::{self, read_to_string, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let text = read_to_string(reader)?;

    let mut parts = text.split("\n\n");

    // Read initial configuration
    let header = parts.nth(0).unwrap();
    let mut config = parse_start_config(header);

    // Parse all the instructions
    for line in parts.nth(0).unwrap().lines() {
        parse_instruction_2(line, &mut config);
    }

    // Print output
    for stack in config {
        print!("{}", stack.last().unwrap());
    }
    println!();
    Ok(())
}

fn parse_start_config(input: &str) -> Vec<Vec<char>> {
    let mut config: Vec<Vec<char>> = Vec::new();
    let mut input_lines = input.lines().rev();

    // Consume line with stack number and initialize config
    for _ in input_lines.nth(0).unwrap().split_whitespace() {
        config.push(Vec::new());
    }

    // Fill config in stack order
    for line in input_lines {
        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_alphabetic() {
                config[i/4].push(c);
            }
        }
    }
    config
}

// Part 1: crates move 1 at a time
fn parse_instruction_1(instruction: &str, config: &mut Vec<Vec<char>>) {
    let parts: Vec<usize> = instruction
        .split_whitespace()
        .filter(|s| s.parse::<usize>().is_ok())
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let num = parts[0];
    let start = parts[1]-1;
    let dest = parts[2]-1;

    for _ in 0..num {
        let item = config[start].pop().unwrap();
        config[dest].push(item);
    }
}

// Part 2: crates move all at once
fn parse_instruction_2(instruction: &str, config: &mut Vec<Vec<char>>) {
    let parts: Vec<usize> = instruction
        .split_whitespace()
        .filter(|s| s.parse::<usize>().is_ok())
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let num = parts[0];
    let start = parts[1]-1;
    let dest = parts[2]-1;
    let idx = config[start].len()-num;

    let mut items: Vec<char> = config[start].drain(idx..).collect();
    config[dest].append(&mut items);
}

