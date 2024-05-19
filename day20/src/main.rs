use std::fs;
use std::error::Error;

#[derive(Debug)]
struct CyclicList {
    data: Vec<i32>
}

impl CyclicList {
    fn mix(&mut self) {
        let orig_data = self.data.clone();
        for num in orig_data {
            //println!("Moving {}", num);
            //println!("{:?}", self.data);
            let curr_idx = self.data.iter().position(|n| *n == num).unwrap();
            let mut tombstone = 0;
            if num.abs() >= self.data.len() as i32 {
                tombstone = if num > 0 { -1 } else { 1 };
            }
            let new_idx = curr_idx as i32 + self.data.remove(curr_idx) + tombstone;
            self.insert(new_idx, num);
            //println!("{:?}", self.data);
        }
    }

    fn get(&self, idx: i32) -> Option<i32> {
        match self.data.len() {
            0 => None,
            l => {
                let modulus = l as i32;
                let mut idx = idx;
                while idx < 0 {
                    idx += modulus;
                }
                let idx = (idx % modulus) as usize;
                Some(*self.data.get(idx).unwrap())
            }
        }
    }

    fn insert(&mut self, orig_idx: i32, item: i32) {
        let modulus = self.data.len() as i32;
        let mut idx = orig_idx;
        while idx < 0 {
            idx += modulus;
        }
        let idx = (idx % modulus) as usize;
        if idx == 0 {
            self.data.push(item);
        } else {
            self.data.insert(idx, item)
        }
    }
}

fn main() {
    let mut data = parse_input("input.txt").unwrap();
    data.mix();
    let zero_idx = data.data.iter().position(|n| *n == 0).unwrap();
    println!("{}", zero_idx);
    let n1 = data.get(zero_idx as i32 + 1000).unwrap();
    let n2 = data.get(zero_idx as i32 + 2000).unwrap();
    let n3 = data.get(zero_idx as i32 + 3000).unwrap();
    println!("{} {} {}", n1, n2, n3);
    println!("Sum: {}", n1 + n2 + n3);
}

fn parse_input(filename: &str) -> Result<CyclicList, Box<dyn Error>> {
    let input_str = fs::read_to_string(filename)?;

    let mut data = Vec::new();
    for line in input_str.lines() {
        data.push(line.parse::<i32>().unwrap());
    }

    Ok(CyclicList {
        data: data
    })
}
