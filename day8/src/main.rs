use std::fs;
use std::error::Error;

struct Grid {
    data: Vec<Vec<u32>>,
    visible: Vec<Vec<bool>>,
    scenic: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut data = Vec::new();
        let mut visible = Vec::new();
        let mut scenic = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            let mut visible_row = Vec::new();
            let mut scenic_row = Vec::new();
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap());
                visible_row.push(false);
                scenic_row.push(0);
            }
            data.push(row);
            visible.push(visible_row);
            scenic.push(scenic_row);
        }
        let height = data.len();
        let width = data[0].len();
        Grid {
            data: data,
            visible: visible,
            scenic: scenic,
            height: height,
            width: width
        }
    }

    fn count_sight_lines(&self) -> usize {
        self.height * 2usize + self.width * 2usize
    }

    fn get_sight_line(&self, idx: usize) -> Vec<(usize, usize, u32)> {
        let mut line = Vec::new();
        if idx < self.width {
            // Starting from top left to top right
            for i in 0..self.height {
                line.push((i, idx, self.data[i][idx]));
            }
            return line;
        }
        let idx = idx - self.width;
        if idx < self.height {
            // Starting from top right to bottom right
            for i in 0..self.width {
                let x = self.width - i - 1;
                line.push((idx, x, self.data[idx][x]));
            }
            return line;
        }
        let idx = idx - self.height;
        if idx < self.width {
            // Starting from bottom left to bottom right
            for i in 0..self.height {
                let y = self.height - i - 1;
                line.push((y, idx, self.data[y][idx]));
            }
            return line;
        }
        let idx = idx - self.width;
        if idx < self.height {
            // Starting from top left to bottom left
            for i in 0..self.width {
                line.push((idx, i, self.data[idx][i]));
            }
            return line;
        } else {
            panic!("Invalid idx for height {} and width {}: {}",
                   self.height, self.width, idx);
        }
    }

    fn mark_visibility_for_line(&mut self, idx: usize) {
        let line = self.get_sight_line(idx);
        let mut tallest = None;
        for i in line {
            match tallest {
                None => {
                    tallest = Some(i.2);
                    self.visible[i.0][i.1] = true;
                },
                Some(t) => {
                    if i.2 > t {
                        tallest = Some(i.2);
                        self.visible[i.0][i.1] = true;
                    }
                }
            }
        }
    }

    fn count_visible(&self) -> u32 {
        let mut total = 0;
        for row in &self.visible {
            for cell in row {
                if *cell {
                    total += 1
                }
            }
        }
        total
    }

    fn calc_scenic_score(&self, i: usize, j: usize) -> u32 {
        if i == 0 || j == 0 || i == self.height-1 || j == self.width-1 {
            return 0;
        }
        let cur_height = self.data[i][j];
        let mut score = 1;
        let mut line = self.get_sight_line(j);
        let mut count = 0;
        for cell in line.iter().take(i).rev() {
            count += 1;
            if cell.2 >= cur_height {
                break;
            }
        }
        score *= count;

        count = 0;
        for cell in line.iter().skip(i+1) {
            count += 1;
            if cell.2 >= cur_height {
                break;
            }
        }
        score *= count;

        line = self.get_sight_line(self.width * 2 + self.height + i);
        count = 0;
        for cell in line.iter().take(j).rev() {
            count += 1;
            if cell.2 >= cur_height {
                break;
            }
        }
        score *= count;

        count = 0;
        for cell in line.iter().skip(j+1) {
            count += 1;
            if cell.2 >= cur_height {
                break;
            }
        }
        score *= count;

        score
    }

    fn fill_scenic_score_table(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                self.scenic[i][j] = self.calc_scenic_score(i, j);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_str = fs::read_to_string("input.txt")?;
    let mut grid = Grid::new(input_str.as_str());

    for i in 0..grid.count_sight_lines() {
        grid.mark_visibility_for_line(i);
    }
    println!("Total visible: {}", grid.count_visible());

    grid.fill_scenic_score_table();
    let mut max = 0;
    for i in 0..grid.height {
        for j in 0..grid.width {
            let score = grid.scenic[i][j];
            if score > max {
                max = score;
            }
        }
    }
    println!("Best scenic score: {}", max);

    Ok(())
}
