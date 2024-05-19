use std::fs;
use std::fmt;

#[derive(Debug)]
enum Jet {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
enum Shape {
    Minus,
    Plus,
    L,
    Pipe,
    Square,
}

// Coordinates:
// x: left-most unit is @ 0, right-most unit is @ width-1
// y: 0 is the bottom layer above the floor, increasing upwards
#[derive(Debug)]
struct Rock {
    pos: (u32, u32),
    shape: Shape
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Air,
    Rock,
    FallingRock,
}

#[derive(Debug)]
struct Chamber {
    jets: Vec<Jet>,
    width: usize,
    grid: Vec<Vec<Tile>>,
}

impl Chamber {
    fn new(filename: &str) -> Self {
        let jet_string = fs::read_to_string(filename).unwrap();
        let jets = jet_string
            .chars()
            .map_while(|c| match c {
                '<' => Some(Jet::Left),
                '>' => Some(Jet::Right),
                _ => None,
            })
            .collect();
        Chamber {
            jets: jets,
            width: 7,
            grid: Vec::new(),
        }
    }

    // Rock is created with left edge two units away from the left wall
    // and bottom edge three units above the highest rock
    // Rock position is anchored in bottom left corner
    fn create_rock(&self, shape: Shape) -> Rock {
        Rock {
            pos: (2, self.height() + 3),
            shape: shape
        }
    }

    fn get_rock_tiles(&self, rock: &Rock) {
        // idk write an iterator adapter
    }

    fn get_mut_rock_tiles(&mut self, rock: &Rock) {
    }

    fn insert_rock_into_grid(&mut self, rock: &Rock, tile: Tile) {
        while (self.grid.len() as u32) < self.height() + 8 {
            self.grid.push(vec![Tile::Air; self.width]);
        }
        let x = rock.pos.0 as usize;
        let y = rock.pos.1 as usize;
        match rock.shape {
            Shape::Minus => {
                self.grid[y][x] = tile;
                self.grid[y][x + 1] = tile;
                self.grid[y][x + 2] = tile;
                self.grid[y][x + 3] = tile;
            },
            _ => panic!("other shapes not implemented")
        }
    }

    // Update the rock position to be the next position
    // (after being pushed by the jet and falling)
    // If it is stopped for either move, return false
    // Otherwise, return true
    fn move_rock(&mut self, rock: &mut Rock, jet_idx: usize) -> bool {
        // Insert air into cur rock position
        // move rock with jet
        // check if rock is blocked (maybe write an iterator for rock shapes)
        // if so, move the rock back, insert rock, and return false
        // else, insert falling rock and continue
        // (print grid)
        // insert air into cur rock position
        // move rock down
        // check if rock is blocked
        // if so, move the rock back, insert rock,  and return false
        // else, insert falling rock and return true
    }

    // Height of 0 is the floor
    // 1 means 1 layer of rocks above the floor,
    // etc...
    fn height(&self) -> u32 {
        let mut height = self.grid.len();
        for row in self.grid.iter().rev() {
            if row.contains(&Tile::Rock) {
                break;
            }
            height -= 1;
        }
        height as u32
    }
}

impl fmt::Display for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output_str = String::new();
        for row in self.grid.iter().rev() {
            output_str.push('|');
            for tile in row.iter() {
                output_str.push(match tile {
                    Tile::Air => '.',
                    Tile::Rock => '#',
                    Tile::FallingRock => '@'
                });
            }
            output_str.push('|');
            output_str.push('\n');
        }
        output_str.push('+');
        for _ in 0..self.width {
            output_str.push('-');
        }
        output_str.push('+');
        output_str.push('\n');
        write!(f, "{}", output_str)
    }
}

fn main() {
    let mut chamber = Chamber::new("test.txt");

    let shapes = vec![
        Shape::Minus,
        Shape::Plus,
        Shape::L,
        Shape::Pipe,
        Shape::Square
    ];

    let mut jet_counter = 0;
    for rock_num in 0..2022 {
        let mut rock = chamber.create_rock(shapes[rock_num]);
        println!("{}", chamber);
        //print!("{esc}[2J{esc}[1;1H{}", chamber, esc = 27 as char);
        while chamber.move_rock(&mut rock, jet_counter) {
            println!("{}", chamber);
            //print!("{esc}[2J{esc}[1;1H{}", chamber, esc = 27 as char);
        }
    }

    println!("Height: {}", chamber.height());
}
