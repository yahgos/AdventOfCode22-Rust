fn main() {
    let start_time = std::time::Instant::now();
    let INPUT = "C:/rust/advent_2022/day4/input/input.txt";
    let TEST_INPUT = "C:/rust/advent_2022/day4/input/test_input.txt";
    let input = std::fs::read_to_string(INPUT).unwrap();
    let test_input = std::fs::read_to_string(TEST_INPUT).unwrap();
    let elves_pairs = parse_input(input);
    let mut fully_contained_sum = 0;
    for pair in elves_pairs {
        if pair.is_overlap() {
            fully_contained_sum += 1;
        }
    }
    println!("sum: {:?}", fully_contained_sum);
    let end_time = std::time::Instant::now();
    println!("Time: {:?}", end_time - start_time);
}

fn parse_input(input: String) -> Vec<elves_pair> {
    let mut elves_pairs: Vec<elves_pair> = Vec::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let mut pairs = line.split(",");
        let mut sections = pairs.next().unwrap().split("-");
        let section1 = section(
            sections.next().unwrap().parse::<u16>().unwrap(),
            sections.next().unwrap().parse::<u16>().unwrap(),
        );
        sections = pairs.next().unwrap().split("-");
        let section2 = section(
            sections.next().unwrap().parse::<u16>().unwrap(),
            sections.next().unwrap().parse::<u16>().unwrap(),
        );
        elves_pairs.push(elves_pair(section1, section2));
    }
    return elves_pairs;
}

#[derive(Debug)]
struct section(u16, u16);
#[derive(Debug)]
struct elves_pair(section, section);

impl elves_pair {
    fn is_overlap(&self) -> bool {
        let section1 = &self.0;
        let section2 = &self.1;

        return !(section2.0 > section1.1 || section1.0 > section2.1);
    }
}


