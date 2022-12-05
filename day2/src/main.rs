use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut points = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            let (my_hand, opponent) = parse_line_2(&line);
            points += points_per_line(&my_hand, &opponent);
        }
    }
    println!("{}", points);
    Ok(())
}

#[derive(PartialEq)]
enum HandShape {
    Rock,
    Paper,
    Scissors
}

// For part 1 where X,Y,Z are hand shapes
fn parse_line_1(line: &String) -> (HandShape, HandShape) {
    let hands: Vec<&str> = line.split_whitespace().collect();
    let opponent = match hands[0] {
        "A" => HandShape::Rock,
        "B" => HandShape::Paper,
        "C" => HandShape::Scissors,
        _ => panic!()
    };
    let my_hand = match hands[1] {
        "X" => HandShape::Rock,
        "Y" => HandShape::Paper,
        "Z" => HandShape::Scissors,
        _ => panic!()
    };
    (my_hand, opponent)
}

// For part 2 where X,Y,Z are lose/draw/win
fn parse_line_2(line: &String) -> (HandShape, HandShape) {
    let hands: Vec<&str> = line.split_whitespace().collect();
    let opponent = match hands[0] {
        "A" => HandShape::Rock,
        "B" => HandShape::Paper,
        "C" => HandShape::Scissors,
        _ => panic!()
    };
    let my_hand = match hands[1] {
        "X" => match opponent {
            HandShape::Rock => HandShape::Scissors,
            HandShape::Paper => HandShape::Rock,
            HandShape::Scissors => HandShape::Paper
        }
        "Y" => match opponent {
            HandShape::Rock => HandShape::Rock,
            HandShape::Paper => HandShape::Paper,
            HandShape::Scissors => HandShape::Scissors
        }
        "Z" => match opponent {
            HandShape::Rock => HandShape::Paper,
            HandShape::Paper => HandShape::Scissors,
            HandShape::Scissors => HandShape::Rock
        }
        _ => panic!()
    };
    (my_hand, opponent)
}

fn points_per_line(my_hand: &HandShape, opponent: &HandShape) -> u32 {
    let mut points = 0;
    points += match *my_hand {
        HandShape::Rock =>
            match *opponent {
                HandShape::Rock => 4,
                HandShape::Paper => 1,
                HandShape::Scissors => 7
            },
        HandShape::Paper =>
            match *opponent {
                HandShape::Rock => 8,
                HandShape::Paper => 5,
                HandShape::Scissors => 2
            },
        HandShape::Scissors =>
            match *opponent {
                HandShape::Rock => 3,
                HandShape::Paper => 9,
                HandShape::Scissors => 6
            }
    };
    points
}
