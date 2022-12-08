use std::fs;

#[derive(Debug)]
struct File {
    name: String,
    size: usize
}

#[derive(Debug)]
struct Dir {
    name: String,
    subdirs: Vec<Dir>,
    files: Vec<File>
}

impl Dir {
    fn new(lines: &Vec<&str>) -> Self {
        Dir::new_subdir("/", lines, 1).0
    }

    fn new_subdir(name: &str, lines: &Vec<&str>, line_idx: usize) -> (Self, usize) {
        let mut subdirs = Vec::new();
        let mut files = Vec::new();
        let mut used_line_count = 0;

        loop {
            if line_idx + used_line_count >= lines.len() {
                break (Dir {
                    name: String::from(name),
                    subdirs: subdirs,
                    files: files
                }, used_line_count);
            }
            let mut tokens = lines[line_idx + used_line_count].split_whitespace();
            used_line_count += 1;

            // Check if the line starts with '$'
            let first_token = tokens.next().unwrap();
            if first_token == "dir" {
                continue;
            } else if first_token == "$" {
                // Move on to next block
            } else {
                files.push(File {
                    name: String::from(tokens.next().unwrap()),
                    size: first_token.parse::<usize>().unwrap()
                });
                continue;
            }

            match tokens.next().unwrap() {
                "ls" => continue,
                "cd" => match tokens.next().unwrap() {
                    ".." => {
                        return (Dir {
                            name: String::from(name),
                            subdirs: subdirs,
                            files: files
                        }, used_line_count);
                    }
                    dest => {
                        let (d, c) = Dir::new_subdir(dest, lines, line_idx + used_line_count);
                        used_line_count += c;
                        subdirs.push(d);
                        continue;
                    }
                }
                _ => panic!()
            };
        }
    }

    fn find_total_size(&self, limit: u32, acc: &mut u32) -> u32 {
        let mut total_size = 0;
        for subdir in &self.subdirs {
            total_size += subdir.find_total_size(limit, acc)
        }
        for file in &self.files {
            total_size += file.size as u32;
        }
        if total_size <= limit {
            *acc += total_size;
        }
        total_size
    }

    fn find_smallest(&self, lower_limit: u32) -> u32 {
        let mut total_size = 0;
        let mut smallest_subdir = u32::MAX;
        for subdir in &self.subdirs {
            total_size += subdir.find_total_size(0, &mut 0);
            match subdir.find_smallest(lower_limit) {
                size if size < smallest_subdir => smallest_subdir = size,
                _ => continue
            }
        }
        for file in &self.files {
            total_size += file.size as u32;
        }
        if total_size >= lower_limit && total_size < smallest_subdir {
            total_size
        } else {
            smallest_subdir
        }
    }
}

fn main() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let mut input_lines = input_str.lines().collect();

    let fs = Dir::new(&mut input_lines);

    let mut result = 0;
    let total = fs.find_total_size(100000, &mut result);
    println!("Sum of dirs less than 100000: {}", result);

    let limit = 30000000 - (70000000 - total);
    println!("Smallest dir to free: {}", fs.find_smallest(limit));
}
