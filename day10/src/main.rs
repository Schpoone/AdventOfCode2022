use std::fs;
use std::error::Error;

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop
}

fn main() {
    let instructions = parse_instructions("input.txt").unwrap();
    let signal_strengths = calc_signal_strengths(&instructions);

    let mut sum = 0;
    for idx in vec![19, 59, 99, 139, 179, 219] {
        println!("{}", signal_strengths[idx]);
        sum += signal_strengths[idx];
    }

    println!("Sum: {}", sum);

    display_render(&instructions);
}

fn parse_instructions(filename: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    let input_str = fs::read_to_string(filename)?;
    let mut instructions = Vec::new();
    for line in input_str.lines() {
        let mut tokens = line.split_whitespace();
        instructions.push(match tokens.next() {
            Some("addx") => Instruction::Addx(
                tokens.next().unwrap().parse::<i32>().unwrap()),
            Some("noop") => Instruction::Noop,
            Some(_) | None => panic!()
        });
    }
    Ok(instructions)
}

fn calc_signal_strengths(instructions: &Vec<Instruction>) -> Vec<i32> {
    let mut register = 1;
    let mut register_values = Vec::new();
    for instruction in instructions {
        match instruction {
            Instruction::Addx(n) => {
                register_values.push(register);
                register_values.push(register);
                register += n;
            },
            Instruction::Noop => {
                register_values.push(register);
            }
        }
    }
    let signal_strengths = register_values.iter()
        .enumerate()
        .map(|(idx, n)| (idx as i32 + 1) * n)
        .collect();
    signal_strengths
}

fn display_render(instructions: &Vec<Instruction>) {
    let mut position = 1;
    let mut cycle_number = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Addx(n) => {
                display_pixel(cycle_number, position);
                cycle_number  = (cycle_number + 1) % 40;
                display_pixel(cycle_number, position);
                cycle_number  = (cycle_number + 1) % 40;
                position += n;
            },
            Instruction::Noop => {
                display_pixel(cycle_number, position);
                cycle_number  = (cycle_number + 1) % 40;
            }
        }
    }
}

fn display_pixel(cycle_number: i32, position: i32) {
    if cycle_number >= position - 1 && cycle_number <= position + 1 {
        print!("#");
    } else {
        print!(".");
    }
    if cycle_number == 39 {
        println!();
    }
}
