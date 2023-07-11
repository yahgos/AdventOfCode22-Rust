fn main() {
    let INPUT = "C:/rust/advent_2022/day5/input/input.txt";
    let TEST_INPUT = "C:/rust/advent_2022/day5/input/test_input.txt";
    let input = std::fs::read_to_string(INPUT).unwrap();
    let test_input = std::fs::read_to_string(TEST_INPUT).unwrap();
    let mut port: Port = parse_input(input);
    port.execute_instructions();
    let top_crates = port.get_top_crates();
    println!("Top crates: {:?}", top_crates);
}

fn parse_input(input: String) -> Port {
    let mut input_split = input.split("\n\n");
    let mut ship = input_split.next().unwrap().lines().rev();
    let crates_len: usize = ship
        .next()
        .unwrap()
        .trim()
        .char_indices()
        .rev()
        .next()
        .unwrap()
        .1
        .to_digit(10)
        .unwrap() as usize;
    let stack_line_len = crates_len * 3 + crates_len - 1;
    let mut ship_stacks = Vec::new();
    for _ in (0..crates_len) {
        let stack = Vec::new();
        ship_stacks.push(stack);
    }
    let mut pos_count = 0;
    while let Some(stack_line) = ship.next() {
        let line_vec = stack_line.split("").collect::<Vec<&str>>();
        (2..stack_line_len).step_by(4).for_each(|i| {
            if pos_count == crates_len {
                pos_count = 0;
            }
            if !(line_vec[i] == " ") {
                ship_stacks[pos_count].push(line_vec[i].to_string());
            }
            pos_count += 1;
        });
    }

    let instructions: Vec<Instruction> = input_split
        .next()
        .unwrap()
        .lines()
        .map(|line| Instruction::try_from(line).unwrap())
        .collect();

    let port = Port {
        ship_stacks,
        instructions,
    };
    port
}

#[derive(Debug)]
struct Port {
    ship_stacks: Vec<Vec<String>>,
    instructions: Vec<Instruction>,
}

impl Port {
    fn execute_instructions(&mut self) {
        for instruction in self.instructions.iter() {
            let mut crates_to_move = Vec::new();
            {
                let source_stack = &mut self.ship_stacks[instruction.source as usize - 1];
                for _ in 0..instruction.quantity {
                    crates_to_move.push(source_stack.pop().unwrap());
                }
            }
            {
                let dest_stack = &mut self.ship_stacks[instruction.dest as usize - 1];
                //remove rev() for part 1
                for crate_to_move in crates_to_move.iter().rev() {
                    dest_stack.push(crate_to_move.to_string());
                }
            }
        }
    }

    fn get_top_crates(&self) -> Vec<String> {
        let mut top_crates = Vec::new();
        for stack in self.ship_stacks.iter() {
            if stack.len() > 0 {
                top_crates.push(stack[stack.len() - 1].to_string());
            } else {
                top_crates.push("-".to_string());
            }
        }
        top_crates
    }
}

#[derive(Debug)]
struct Instruction {
    quantity: u8,
    source: u8,
    dest: u8,
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split(" ").filter(|char| char.parse::<u8>().is_ok());
        let quantity = split.next().unwrap().parse::<u8>().unwrap();
        let source = split.next().unwrap().parse::<u8>().unwrap();
        let dest = split.next().unwrap().parse::<u8>().unwrap();
        Ok(Instruction {
            quantity,
            source,
            dest,
        })
    }
}
