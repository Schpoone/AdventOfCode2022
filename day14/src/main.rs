use std::fs;
use std::fmt;
use std::error::Error;
use std::cmp::{min, max};

#[derive(Debug)]
struct Scan {
    grid: Vec<Vec<Tile>>,
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize
}

impl Scan {
    fn new(x_min: usize, x_max: usize, y_min: usize, y_max: usize) -> Self {
        let width = x_max - x_min + 1;
        let height = y_max - y_min + 1;
        let mut grid = vec![vec![Tile::Air; width]; height];
        grid[0 - y_min][500 - x_min] = Tile::Start;
        grid.push(vec![Tile::Air; width]);
        grid.push(vec![Tile::Rock; width]);
        Self {
            grid: grid,
            x_min: x_min,
            x_max: x_max,
            y_min: y_min,
            y_max: y_max + 2
        }
    }

    fn get(&mut self, x: usize, y: usize) -> Option<Tile> {
        if y < self.y_min || y > self.y_max {
            return None;
        }
        if x < self.x_min {
            for row in self.grid.iter_mut() {
                row.insert(0, Tile::Air);
            }
            let height = self.grid.len() - 1;
            self.grid[height][0] = Tile::Rock;
            self.x_min -= 1;
        }
        if x > self.x_max {
            for row in self.grid.iter_mut() {
                row.push(Tile::Air);
            }
            let height = self.grid.len() - 1;
            self.grid[height].pop();
            self.grid[height].push(Tile::Rock);
            self.x_max += 1;
        }
        Some(self.grid[y - self.y_min][x - self.x_min])
    }

    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        self.grid[y - self.y_min][x - self.x_min] = tile;
    }

    fn insert_sand(&mut self) -> Option<Tile> {
        let mut pos = (500, 0);
        loop {
            let (x, y) = pos;
            if self.get(x, y + 1)? == Tile::Air {
                pos = (x, y + 1);
                continue;
            }
            if self.get(x - 1, y + 1)? == Tile::Air {
                pos = (x - 1, y + 1);
                continue;
            }
            if self.get(x + 1, y + 1)? == Tile::Air {
                pos = (x + 1, y + 1);
                continue;
            }
            self.set(pos.0, pos.1, Tile::Sand);
            if pos == (500, 0) {
                return None
            }
            return Some(Tile::Sand);
        }
    }
}

impl fmt::Display for Scan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output_str = String::new();
        for row in self.grid.iter() {
            for tile in row.iter() {
                output_str.push(match tile {
                    Tile::Air => '.',
                    Tile::Rock => '#',
                    Tile::Sand => 'o',
                    Tile::Start => '+'
                });
            }
            output_str.push('\n');
        }
        write!(f, "{}", output_str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Air,
    Rock,
    Sand,
    Start
}

fn main() {
    let mut scan = read_scan("input.txt").unwrap();
    println!("{}", scan);

    let mut num_sand = 0;
    loop {
        let result = scan.insert_sand();
        match result {
            Some(_) => num_sand += 1,
            None => break
        }
        print!("{esc}[2J{esc}[1;1H{}", scan, esc = 27 as char);
    }

    println!("Amount of sand: {}", num_sand + 1);
}

fn read_scan(filename: &str) -> Result<Scan, Box<dyn Error>> {
    let input_str = fs::read_to_string(filename)?;

    let (x_min, x_max, _, y_max) = find_bounds(input_str.as_str());
    assert!(x_min <= 500);
    assert!(x_max >= 500);

    let mut scan = Scan::new(x_min, x_max, 0, y_max);
    for line in input_str.lines() {
        let mut prev_coordinate = line.split(" -> ").nth(0).unwrap();
        for coordinate in line.split(" -> ") {
            let (x_prev, y_prev) = parse_coord(prev_coordinate);
            let (x, y) = parse_coord(coordinate);
            if x == x_prev {
                for y2 in min(y, y_prev)..=max(y, y_prev) {
                    scan.set(x, y2, Tile::Rock);
                }
            } else if y == y_prev {
                for x2 in min(x, x_prev)..=max(x, x_prev) {
                    scan.set(x2, y, Tile::Rock);
                }
            } else {
                panic!();
            }
            prev_coordinate = coordinate;
        }
    }

    Ok(scan)
}

fn find_bounds(input_str: &str) -> (usize, usize, usize, usize) {
    let (mut x_min, mut x_max, mut y_min, mut y_max) = (usize::MAX, 0, usize::MAX, 0);

    for line in input_str.lines() {
        for coordinate in line.split(" -> ") {
            let (x, y) = parse_coord(coordinate);
            if x < x_min {
                x_min = x;
            }
            if x > x_max {
                x_max = x
            }
            if y < y_min {
                y_min = y;
            }
            if y > y_max {
                y_max = y;
            }
        }
    }

    (x_min - 2, x_max + 2, y_min, y_max)
}

fn parse_coord(input_str: &str) -> (usize, usize) {
    let mut components = input_str.split(',');
    let x = components.next().unwrap().parse::<usize>().unwrap();
    let y = components.next().unwrap().parse::<usize>().unwrap();
    (x,y)
}
