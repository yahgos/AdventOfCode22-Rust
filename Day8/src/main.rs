fn main() {
    let INPUT = "C:/rust/advent_2022/Day8/input/input.txt";
    let input = std::fs::read_to_string(INPUT).unwrap();
    let mut trees = parse_input(input);
    let visible_trees = trees.run_quadcopter();
    println!("Visible trees: {}", visible_trees);
    println!("Highest Scenic Score: {}", trees.highest_scenic_score);
}

#[derive(Debug)]
struct Tree {
    height: u8,
    is_visible: bool,
    scenic_score: usize,
}
#[derive(Debug)]
struct Forest {
    grid: Vec<Vec<Tree>>,
    size: (usize, usize),
    highest_scenic_score: usize,
}

impl Forest {
    fn run_quadcopter(&mut self) -> u32 {
        // let mut rows = self.grid.iter_mut();
        // let mut row = rows.next();
        let max_width = self.grid.len();
        let max_depth = self.grid[0].len();
        let mut count = 0;
        let mut edge_counter = 0;
        let mut current_highest_scenic_score = 0;
        for i in 0..max_width {
            for j in 0..max_depth {
                if i == 0 || j == 0 || i == self.size.0 - 1 || j == self.size.1 - 1 {
                    edge_counter += 1;
                }
                if check_visibility(self, i, j) {
                    self.grid[i][j].is_visible = true;
                    count += 1;
                }
                let current_tree_scenic_score = self.grid[i][j].scenic_score;
                if current_tree_scenic_score > current_highest_scenic_score {
                    current_highest_scenic_score = current_tree_scenic_score;
                }
            }
        }
        self.highest_scenic_score = current_highest_scenic_score;
        count
    }
}

fn check_visibility(forest: &mut Forest, i: usize, j: usize) -> bool {
    if i == 0 || j == 0 || i == forest.size.0 - 1 || j == forest.size.1 - 1 {
        forest.grid[i][j].scenic_score = 0;
        return true;
    }

    let traverse_order: [Direction; 4] = calculate_route(i as f32, j as f32, forest.size);

    let tree_height = &forest.grid[i][j].height.clone();
    let mut is_visible = false;

    'direction: for direction in traverse_order {
        match direction {
            Direction::Up => {
                for y in (0..j).rev() {
                    if &forest.grid[i][y].height >= tree_height {
                        let scenic_score = j - y;
                        forest.grid[i][j].scenic_score *= scenic_score;
                        continue 'direction;
                    }
                }
                forest.grid[i][j].scenic_score *= j;
                is_visible = true;
            }
            Direction::Down => {
                for y in (j + 1)..forest.size.1 {
                    if &forest.grid[i][y].height >= tree_height {
                        let scenic_score = y - j;
                        forest.grid[i][j].scenic_score *= scenic_score;
                        continue 'direction;
                    }
                }
                forest.grid[i][j].scenic_score *= (forest.size.1 - 1 - j);
                is_visible = true;
            }
            Direction::Left => {
                for x in (0..i).rev() {
                    if &forest.grid[x][j].height >= tree_height {
                        forest.grid[i][j].scenic_score *= (i - x);
                        continue 'direction;
                    }
                }
                forest.grid[i][j].scenic_score *= i;
                is_visible = true;
            }
            Direction::Right => {
                for x in (i + 1)..forest.size.0 {
                    if &forest.grid[x][j].height >= tree_height {
                        forest.grid[i][j].scenic_score *= (x - i);
                        continue 'direction;
                    }
                }
                forest.grid[i][j].scenic_score *= (forest.size.0 - 1 - i);
                is_visible = true;
            }
        }
    }
    is_visible
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn calculate_route(i: f32, j: f32, size: (usize, usize)) -> [Direction; 4] {
    let mut route = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    let x_factor = size.0.clone() as f32 / 2.0;
    let y_factor = size.1.clone() as f32 / 2.0;

    if i < x_factor && j < y_factor {
        if i < j {
            route[0] = Direction::Left;
            route[1] = Direction::Up;
            route[2] = Direction::Right;
            route[3] = Direction::Down;
        } else {
            route[0] = Direction::Up;
            route[1] = Direction::Left;
            route[2] = Direction::Down;
            route[3] = Direction::Right;
        }
    } else if i < x_factor && j > y_factor {
        if i < j {
            route[0] = Direction::Left;
            route[1] = Direction::Down;
            route[2] = Direction::Right;
            route[3] = Direction::Up;
        } else {
            route[0] = Direction::Down;
            route[1] = Direction::Left;
            route[2] = Direction::Up;
            route[3] = Direction::Right;
        }
    } else if i > x_factor && j < y_factor {
        if i < j {
            route[0] = Direction::Up;
            route[1] = Direction::Right;
            route[2] = Direction::Down;
            route[3] = Direction::Left;
        } else {
            route[0] = Direction::Right;
            route[1] = Direction::Up;
            route[2] = Direction::Left;
            route[3] = Direction::Down;
        }
    } else if i > x_factor && j > y_factor {
        if i < j {
            route[0] = Direction::Down;
            route[1] = Direction::Right;
            route[2] = Direction::Up;
            route[3] = Direction::Left;
        } else {
            route[0] = Direction::Right;
            route[1] = Direction::Down;
            route[2] = Direction::Up;
            route[3] = Direction::Left;
        }
    }
    route
}

fn parse_input<'a>(input: String) -> Forest {
    let mut trees: Vec<Vec<Tree>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<Tree> = Vec::new();
        for c in line.split("") {
            if c == "" {
                continue;
            }
            let tree = Tree {
                height: c.parse::<u8>().unwrap(),
                is_visible: false,
                scenic_score: 1,
            };
            row.push(tree);
        }
        trees.push(row);
    }
    let size: (usize, usize) = (trees.len().to_owned(), trees[0].len().to_owned());
    Forest {
        grid: trees,
        size: size,
        highest_scenic_score: 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let TEST_INPUT = "C:/rust/advent_2022/Day8/input/test_input.txt";
        let test_input = std::fs::read_to_string(TEST_INPUT).unwrap();
        let mut test_trees = parse_input(test_input);
        let test_visible_trees = test_trees.run_quadcopter();
        println!("Test Visible trees: {}", test_visible_trees);
        println!(
            "Test Highest Scenic Score: {}",
            test_trees.highest_scenic_score
        );
        assert_eq!(test_visible_trees, 21);
        assert_eq!(test_trees.highest_scenic_score, 8);

    }
}
