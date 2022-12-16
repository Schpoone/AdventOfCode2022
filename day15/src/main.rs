use std::fs;
use regex::Regex;
use std::ops::Add;
use std::error::Error;
use std::collections::{HashSet, HashMap};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i128,
    y: i128
}

impl Point {
    fn new(x: i128, y: i128) -> Self {
        Point {
            x: x,
            y: y
        }
    }

    fn distance(p1: Self, p2: Self) -> i128 {
        (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

struct SparseScan {
    sensor2beacon: HashMap<Point, Point>,
    beacons: HashSet<Point>,
    x_min: i128,
    x_max: i128,
}

impl SparseScan {
    fn new(x_min: i128, x_max: i128) -> Self {
        Self {
            sensor2beacon: HashMap::new(),
            beacons: HashSet::new(),
            x_min: x_min,
            x_max: x_max,
        }
    }

    fn is_covered(&self, p: Point) -> bool {
        for (sensor, beacon) in self.sensor2beacon.iter() {
            let d = Point::distance(*sensor, *beacon);
            if Point::distance(p, *sensor) <= d {
                return true;
            }
        }
        false
    }

    fn get_potential_beacons_in_row(&self, y: i128, min: i128, max: i128, include_beacons: bool) -> Vec<Point> {
        let mut potentials = Vec::new();
        for x in min..=max {
            let p = Point::new(x, y);
            if self.beacons.contains(&p) {
                if include_beacons {
                    potentials.push(p);
                }
                continue;
            }
            if !self.is_covered(p) {
                potentials.push(p);
            }
        }
        potentials
    }

    fn get_covered_in_row(&self, y: i128) -> u128 {
        let beacons = self.get_potential_beacons_in_row(y, self.x_min, self.x_max, true);
        (self.x_max - self.x_min + 1) as u128 - beacons.len() as u128
    }

    fn find_beacon(&self, p_min: Point, p_max: Point) -> Option<Point> {
        let w = p_max.x - p_min.x;
        let h = p_max.y - p_min.y;

        // Check if search region is contained by a single sensor neighborhood
        let corners = vec![
            p_min,
            p_min + Point::new(w, 0),
            p_min + Point::new(0, h),
            p_max
        ];
        for (sensor, beacon) in self.sensor2beacon.iter() {
            let d = Point::distance(*sensor, *beacon);
            if corners.iter().all(|&c| Point::distance(c, *sensor) <= d) {
                return None;
            }
        }

        // If the search region is small enough, check individual tiles
        if Point::distance(p_min, p_max) <= 1 {
            for i in p_min.x..=p_max.x {
                for j in p_min.y..=p_max.y {
                    let p = Point::new(i, j);
                    if !self.is_covered(Point::new(i, j)) {
                        return Some(p);
                    }
                }
            }
            return None;
        }

        // Break up the search region into 4 smaller regions
        let quads = vec![
            (p_min, p_min + Point::new(w/2, h/2)), // top left
            (p_min + Point::new(w/2 + 1, 0), p_min + Point::new(w, h/2)), // top right
            (p_min + Point::new(0, h/2 + 1), p_min + Point::new(w/2, h)), // bot left
            (p_min + Point::new(w/2 + 1, h/2 + 1), p_max) // bot right
        ];
        for quad in quads {
            match self.find_beacon(quad.0, quad.1) {
                Some(p) => return Some(p),
                None => continue
            }
        }
        None
    }
}

fn main() {
    //let scan = read_scan("test.txt").unwrap();
    let scan = read_scan("input.txt").unwrap();

    //println!("Covered positions at y={}: {}", 10, scan.get_covered_in_row(10));
    println!("Covered positions at y={}: {}", 2000000, scan.get_covered_in_row(2000000));

    //let beacon = scan.find_beacon(Point::new(0, 0), Point::new(20, 20)).unwrap();
    let beacon = scan.find_beacon(Point::new(0, 0), Point::new(4000000, 4000000)).unwrap();
    println!("Missing beacon at {:?}", beacon);
    println!("Tuning frequency: {}", beacon.x*4000000 + beacon.y);
}

fn read_scan(filename: &str) -> Result<SparseScan, Box<dyn Error>> {
    let input_str = fs::read_to_string(filename)?;

    let (x_min, x_max) = find_bounds(input_str.as_str());

    let mut scan = SparseScan::new(x_min, x_max);
    for line in input_str.lines() {
        let mut coords = parse_coords(line).into_iter();
        let sensor = coords.next().unwrap();
        let beacon = coords.next().unwrap();
        scan.sensor2beacon.insert(sensor, beacon);
        scan.beacons.insert(beacon);
    }

    Ok(scan)
}

fn find_bounds(input_str: &str) -> (i128, i128) {
    let (mut x_min, mut x_max) = (i128::MAX, i128::MIN);

    for line in input_str.lines() {
        let mut coords = parse_coords(line).into_iter();
        let sensor = coords.next().unwrap();
        let beacon = coords.next().unwrap();
        let d = Point::distance(sensor, beacon);
        if sensor.x - d < x_min {
            x_min = sensor.x - d;
        }
        if sensor.x + d > x_max {
            x_max = sensor.x + d;
        }
    }

    (x_min, x_max)
}

fn parse_coords(input_str: &str) -> Vec<Point> {
    let re = Regex::new(r"x=(-*\d+), y=(-*\d+)").unwrap();
    re.captures_iter(input_str)
        .map(|cap| Point::new(cap[1].parse::<i128>().unwrap(), cap[2].parse::<i128>().unwrap()))
        .collect::<Vec<Point>>()
}
