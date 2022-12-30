use std::fs;
use std::ops::Add;
use std::error::Error;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: x,
            y: y,
            z: z
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

#[derive(Debug)]
enum IntExt {
    Interior,
    Exterior
}

#[derive(Debug)]
struct Structure {
    x_bounds: (i32, i32),
    y_bounds: (i32, i32),
    z_bounds: (i32, i32),
    blocks: HashSet<Point>,
    block_to_int_ext_map: HashMap<Point, IntExt>
}

impl Structure {
    fn new() -> Self {
        Self {
            x_bounds: (i32::MAX, i32::MIN),
            y_bounds: (i32::MAX, i32::MIN),
            z_bounds: (i32::MAX, i32::MIN),
            blocks: HashSet::new(),
            block_to_int_ext_map: HashMap::new()
        }
    }

    fn is_within_bounds(&self, point: &Point) -> bool {
        if point.x < self.x_bounds.0 {
            return false;
        }
        if point.x > self.x_bounds.1 {
            return false;
        }
        if point.y < self.y_bounds.0 {
            return false;
        }
        if point.y > self.y_bounds.1 {
            return false;
        }
        if point.z < self.z_bounds.0 {
            return false;
        }
        if point.z > self.z_bounds.1 {
            return false;
        }
        true
    }

    fn get_surface_area(&self) -> u32 {
        let mut surface_area = 0;
        for block in self.blocks.iter() {
            let neighbors = vec![
                *block + Point::new(1,0,0),
                *block + Point::new(-1,0,0),
                *block + Point::new(0,1,0),
                *block + Point::new(0,-1,0),
                *block + Point::new(0,0,1),
                *block + Point::new(0,0,-1)
            ];
            for neighbor in neighbors {
                if !self.blocks.contains(&neighbor) {
                    surface_area += 1;
                }
            }
        }
        surface_area
    }

    fn fill_int_ext_map(&mut self, point: &Point, visited: &mut HashSet<Point>) {
        let neighbors = vec![
            *point + Point::new(1,0,0),
            *point + Point::new(-1,0,0),
            *point + Point::new(0,1,0),
            *point + Point::new(0,-1,0),
            *point + Point::new(0,0,1),
            *point + Point::new(0,0,-1)
        ];
        for neighbor in neighbors {
            if visited.contains(&neighbor) {
                continue;
            }
            visited.insert(*point);
            if self.blocks.contains(&neighbor) {
                continue;
            }
            if !self.is_within_bounds(&neighbor) {
                self.block_to_int_ext_map.insert(*point, IntExt::Exterior);
                return;
            }
            if !self.block_to_int_ext_map.contains_key(&neighbor) {
                self.fill_int_ext_map(&neighbor, visited);
            }
            match self.block_to_int_ext_map.get(&neighbor) {
                Some(IntExt::Interior) => {
                    self.block_to_int_ext_map.insert(*point, IntExt::Interior);
                    return;
                },
                Some(IntExt::Exterior) => {
                    self.block_to_int_ext_map.insert(*point, IntExt::Exterior);
                    return;
                },
                _ => panic!()
            }
        }
        self.block_to_int_ext_map.insert(*point, IntExt::Interior);
        return;
    }

    fn get_ext_surface_area(&mut self) -> u32 {
        let mut surface_area = 0;
        let blocks = self.blocks.clone();
        for block in blocks {
            let neighbors = vec![
                block + Point::new(1,0,0),
                block + Point::new(-1,0,0),
                block + Point::new(0,1,0),
                block + Point::new(0,-1,0),
                block + Point::new(0,0,1),
                block + Point::new(0,0,-1)
            ];
            for neighbor in neighbors {
                if self.blocks.contains(&neighbor) {
                    continue;
                }
                if !self.is_within_bounds(&neighbor) {
                    surface_area += 1;
                    continue;
                }
                if !self.block_to_int_ext_map.contains_key(&neighbor) {
                    self.fill_int_ext_map(&neighbor, &mut HashSet::new());
                }
                match self.block_to_int_ext_map.get(&neighbor) {
                    Some(IntExt::Interior) => continue,
                    Some(IntExt::Exterior) => surface_area += 1,
                    _ => panic!()
                }
            }
        }
        surface_area
    }
}

fn main() {
    let mut structure = read_scan("input.txt").unwrap();

    println!("Surface area: {}", structure.get_surface_area());
    println!("Exterior Surface area: {}", structure.get_ext_surface_area());
}

fn read_scan(filename: &str) -> Result<Structure, Box<dyn Error>> {
    let input_str = fs::read_to_string(filename)?;

    let mut structure = Structure::new();
    for line in input_str.lines() {
        let mut components = line.split(',');
        let point = Point {
            x: components.next().unwrap().parse::<i32>().unwrap(),
            y: components.next().unwrap().parse::<i32>().unwrap(),
            z: components.next().unwrap().parse::<i32>().unwrap(),
        };
        if point.x < structure.x_bounds.0 {
            structure.x_bounds = (point.x, structure.x_bounds.1);
        }
        if point.x > structure.x_bounds.1 {
            structure.x_bounds = (structure.x_bounds.0, point.x);
        }
        if point.y < structure.y_bounds.0 {
            structure.y_bounds = (point.y, structure.y_bounds.1);
        }
        if point.y > structure.y_bounds.1 {
            structure.y_bounds = (structure.y_bounds.0, point.y);
        }
        if point.z < structure.z_bounds.0 {
            structure.z_bounds = (point.z, structure.z_bounds.1);
        }
        if point.z > structure.z_bounds.1 {
            structure.z_bounds = (structure.z_bounds.0, point.z);
        }
        structure.blocks.insert(point);
    }
    Ok(structure)
}
