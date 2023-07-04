use std::fs;

fn main() {
    //make sure you input file is LF not CRLF
    let INPUT = fs::read_to_string("./input/part_1_input.txt").expect("Couldnt read input file");
    let mut elf_calories: Vec<u32> = parse_input(INPUT);
    println!("{:?}", elf_calories);
    println!("{:}", sum_top_three(elf_calories));

}

fn parse_input(input: String) -> Vec<u32> {
    //make sure you input file is LF not CRLF
    let parsed = input.split("\n\n").map(|elf_cargo| {
        elf_cargo
            .lines()
            .map(|calorie| {
                calorie.parse::<u32>().unwrap()
            })
            .sum::<u32>()
    });
    let mut ordered_res = parsed.collect::<Vec<u32>>();
    ordered_res.sort_by(|a, b| b.cmp(a));
    return ordered_res
}

fn sum_top_three(elves: Vec<u32>) -> String {
    let sum: u32 = elves.iter().take(3).sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let TEST_INPUT =
            fs::read_to_string("./input/test_input.txt").expect("Couldnt read input file");
        println!("{:?}", parse_input(TEST_INPUT));
    }
}
