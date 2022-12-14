use std::fs;
use std::error::Error;
use std::str::Chars;
use std::cmp::{Ordering, min};
use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Int(u32),
    EndBracket,
    Empty
}

impl Packet {
    fn new(char_iter: &mut Peekable<Chars>) -> Self {
        let c = char_iter.next();
        match c {
            Some('[') => {
                let mut list = Vec::new();
                loop {
                    let next_item = Packet::new(char_iter);
                    if next_item == Packet::Empty {
                        continue;
                    }
                    if next_item == Packet::EndBracket {
                        break;
                    }
                    list.push(next_item);
                }
                Packet::List(list)
            },
            Some(']') => Packet::EndBracket,
            Some(',') => Packet::Empty,
            Some(c) => {
                let mut num = String::new();
                num.push(c);
                loop {
                    match char_iter.peek() {
                        Some(',') => {
                            char_iter.next();
                            break;
                        },
                        Some(']') => break,
                        Some(c) => {
                            num.push(*c);
                            char_iter.next();
                        },
                        _ => panic!()
                    }
                }
                Packet::Int(num.parse::<u32>().unwrap())
            },
            None => Packet::Empty
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
            (Packet::Int(a), Packet::List(_)) => {
                let left = Packet::List(vec![Packet::Int(*a)]);
                left.cmp(other)
            },
            (Packet::List(_), Packet::Int(b)) => {
                let right = Packet::List(vec![Packet::Int(*b)]);
                self.cmp(&right)
            },
            (Packet::List(a), Packet::List(b)) => {
                let max_idx = min(a.len(), b.len());
                for idx in 0..max_idx {
                    match a[idx].cmp(&b[idx]) {
                        Ordering::Equal => continue,
                        other => return other
                    }
                }
                a.len().cmp(&b.len())
            }
            _ => panic!()
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let filename = "input.txt";
    let pairs = parse_input(filename).unwrap();
    let mut sum = 0;
    for (idx, pair) in pairs.into_iter().enumerate() {
        if pair.0 < pair.1 {
            sum += idx + 1;
        } else {
        }
    }
    println!("Sum: {}", sum);

    let mut packets = parse_input_2(filename).unwrap();
    packets.sort();
    let idx1 = packets.binary_search(
        &Packet::List(vec!(Packet::List(vec!(Packet::Int(2))))))
        .unwrap();
    let idx2 = packets.binary_search(
        &Packet::List(vec!(Packet::List(vec!(Packet::Int(6))))))
        .unwrap();
    println!("Decoder key: {}", (idx1+1)*(idx2+1));
}

fn parse_input(filename: &str) -> Result<Vec<(Packet, Packet)>, Box<dyn Error>> {
    let input_str = fs::read_to_string(filename)?;

    let mut data = Vec::new();
    let mut pair = (Packet::Empty, Packet::Empty);
    for line in input_str.lines() {
        match line.len() {
            0 => {
                data.push(pair);
                pair = (Packet::Empty, Packet::Empty);
            },
            _ => {
                let packet = Packet::new(&mut line.chars().peekable());
                if pair.0 == Packet::Empty {
                    pair = (packet, Packet::Empty);
                } else if pair.1 == Packet::Empty {
                    pair = (pair.0, packet);
                } else {
                    panic!();
                }
            }
        }
    }
    data.push(pair);

    Ok(data)
}

fn parse_input_2(filename: &str) -> Result<Vec<Packet>, Box<dyn Error>> {
    let input_str = fs::read_to_string(filename)?;

    let mut data = Vec::new();
    for line in input_str.lines() {
        match line.len() {
            0 => {
                continue;
            },
            _ => {
                let packet = Packet::new(&mut line.chars().peekable());
                data.push(packet);
            }
        }
    }

    // Divider packets
    data.push(Packet::List(vec!(Packet::List(vec!(Packet::Int(2))))));
    data.push(Packet::List(vec!(Packet::List(vec!(Packet::Int(6))))));

    Ok(data)
}
