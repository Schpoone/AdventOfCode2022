use std::fs;
use std::error::Error;

#[derive(Debug, Clone)]
struct Monkey {
    idx: usize,
    num_items_inspected: u128,
    items: Vec<u128>,
    operation: String,
    test: u128,
    next: (usize, usize),
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            idx: 0,
            num_items_inspected: 0,
            items: Vec::new(),
            operation: String::new(),
            test: 0, 
            next: (0, 0),
        }
    }

    fn inspect_items(&mut self, modulus: u128) {
        for item in self.items.iter_mut() {
            // Increment items_inspected
            self.num_items_inspected += 1;

            // Apply operation
            let mut tokens = self.operation.split_whitespace().skip(3);
            match tokens.next() {
                Some("+") => {
                    *item += match tokens.next() {
                        Some("old") => *item,
                        Some(n) => n.parse::<u128>().unwrap(),
                        _ => panic!()
                    };
                },
                Some("*") => {
                    *item *= match tokens.next() {
                        Some("old") => *item,
                        Some(n) => n.parse::<u128>().unwrap(),
                        _ => panic!()
                    };
                },
                _ => panic!()
            }

            // Keep worry levels manageable
            *item %= modulus;
            //*item /= 3;
        }
    }

    fn throw_to_monkeys(&self, new_monkeys: &mut Vec<Monkey>) {
        for item in self.items.iter() {
            if *item % self.test == 0 {
                new_monkeys[self.next.0].items.push(*item);
            } else {
                new_monkeys[self.next.1].items.push(*item);
            }
        }
        new_monkeys[self.idx].items.clear();
    }
}

fn main() {
    let mut monkeys = parse_input("input.txt").unwrap();
    let modulus: u128 = monkeys.iter().map(|m| m.test).product();

    for round in 0..10000 {
        for idx in 0..monkeys.len() {
            // Inspect all the items of the current monkey
            let monkey = &mut monkeys[idx];
            monkey.inspect_items(modulus);

            // Prepare a second buffer to record changes in item ownership
            let mut new_monkeys = monkeys.clone();
            let monkey = &monkeys[idx];
            monkey.throw_to_monkeys(&mut new_monkeys);

            // Replace the old buffer
            monkeys = new_monkeys;
        }
    }

    for monkey in monkeys.iter() {
        println!("Monkey {} inspected items {:?}", monkey.idx, monkey.num_items_inspected);
        println!("Monkey {}: {:?}", monkey.idx, monkey.items);
    }

    let mut monkey_activity: Vec<u128> = monkeys.iter()
        .map(|m| m.num_items_inspected)
        .collect();
    monkey_activity.sort();
    let monkey_business: u128 = monkey_activity.iter().rev().take(2).product();
    println!("Level of monkey business: {}", monkey_business);
}

fn parse_input(filename: &str) -> Result<Vec<Monkey>, Box<dyn Error>> {
    let input_str = fs::read_to_string(filename)?;

    let mut monkeys = Vec::new();
    let mut curr = Monkey::new();
    for line in input_str.lines() {
        let mut tokens = line.split_whitespace();
        match tokens.next() {
            Some("Monkey") => {
                curr.idx = tokens.next().unwrap()
                    .chars().take_while(|c| c.is_ascii_digit()).collect::<String>()
                    .parse::<usize>().unwrap();
            },
            Some("Starting") => {
                loop {
                    match tokens.next() {
                        Some(token) => {
                            let token = token.chars()
                                .take_while(|c| c.is_ascii_digit())
                                .collect::<String>();
                            if let Ok(num) = token.parse::<u128>() {
                                curr.items.push(num);
                            }
                        },
                        None => break
                    }
                }
            },
            Some("Operation:") => {
                curr.operation = tokens.collect::<Vec<&str>>().join(" ");
            },
            Some("Test:") => {
                curr.test = tokens.last().unwrap().parse::<u128>().unwrap();
            },
            Some("If") => {
                match tokens.next() {
                    Some("true:") => {
                        curr.next = (tokens.last().unwrap().parse::<usize>().unwrap(),
                                     curr.next.1)
                    },
                    Some("false:") => {
                        curr.next = (curr.next.0,
                                     tokens.last().unwrap().parse::<usize>().unwrap())
                    },
                    _ => panic!(),
                }
            },
            Some(_) => panic!(),
            None => {
                monkeys.push(curr);
                curr = Monkey::new();
            },
        }
    }
    monkeys.push(curr);
    Ok(monkeys)
}
