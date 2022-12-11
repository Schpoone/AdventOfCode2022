use std::fs;
use std::cmp;
use std::error::Error;
use std::ops::{Add, Sub, Neg, AddAssign};
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Vector {
    x: i32,
    y: i32
}

impl Vector {
    fn new(x: i32, y: i32) -> Self {
        Vector {
            x: x,
            y: y
        }
    }

    fn magnitude(&self) -> i32 {
        cmp::max(self.x.abs(), self.y.abs())
    }

    fn normalized(&self) -> Self {
        Self {
            x: match self.x {
                n if n < -1 => -1,
                n if n > 1 => 1,
                n @ _ => n
            },
            y: match self.y {
                n if n < -1 => -1,
                n if n > 1 => 1,
                n @ _ => n
            }
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y
        }

    }
}

struct Simulation {
    knots: Vec<Vector>,
    tail_visited: HashSet<Vector>
}

impl Simulation {
    fn new(n: u32) -> Self {
        let mut knots = Vec::new();
        for _ in 0..n {
            knots.push(Vector::new(0, 0))
        }
        Simulation {
            knots: knots,
            tail_visited: HashSet::new()
        }
    }

    fn simulate(&mut self, step: Vector) {
        // First, move head
        self.knots[0] += step;

        // Calculate where knots need to move
        for i in 1..self.knots.len() {
            let diff = self.knots[i-1] - self.knots[i];
            if diff.magnitude() > 1 {
                self.knots[i] += diff.normalized();
            }
        }

        // Add new position to tail_visited
        self.tail_visited.insert(self.knots[self.knots.len()-1]);
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let input_str = fs::read_to_string("input.txt")?;
    let steps = get_step_list(input_str.as_str());
    let mut sim = Simulation::new(10);

    for step in steps {
        sim.simulate(step);
    }

    println!("Visited: {}", sim.tail_visited.len());

    Ok(())
}

fn get_step_list(input: &str) -> Vec<Vector> {
    let mut steps = Vec::new();
    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        let direction = tokens.next().unwrap();
        let count = tokens.next().unwrap();
        for _ in 0..count.parse::<u32>().unwrap() {
            steps.push(match direction {
                "L" => Vector::new(-1, 0),
                "R" => Vector::new(1, 0),
                "U" => Vector::new(0, 1),
                "D" => Vector::new(0, -1),
                _ => panic!()
            });
        }
    }
    steps
}
