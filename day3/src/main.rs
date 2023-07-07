#![feature(iter_array_chunks)]
use std::collections::HashSet;

fn main() {
    // let bytes = "aaabcczABCZ".as_bytes();
    // let bytesv: Vec<u16> = vec![96, 95, 97, 122, 123, 124, 63, 64, 65, 90, 91, 92];
    // let string = String::from_utf8(bytesv).unwrap();
    // println!("String: {}", string);
    // println!("{:?}", bytes);
    let TEST_PATH = "C:/rust/advent_2022/day3/src/test/test_input.txt";
    let PATH = "C:/rust/advent_2022/day3/src/test/challenge_input1.txt";
    let input = std::fs::read_to_string(PATH).unwrap();
    let rucksacks = parse_rucksacks(input);
    // let mut priority_sum = 0
    // priority_sum += rucksack.shared_item.unwrap().priority;
    let result: Vec<&Item> = rucksacks
        .iter()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let binding = a.get_unique_items();
            let common_item: Vec<&&Item> = binding
                .iter()
                .filter(|item| {
                    b.get_unique_items().contains(item) && c.get_unique_items().contains(item)
                })
                .collect::<Vec<&&Item>>();
            common_item[0].clone()
        })
        .collect::<Vec<&Item>>();

    // println!("Priority sum: {}", priority_sum);
    println!("Result: {:?}", result);
    let result_priority_sum = result.iter().fold(0, |acc, item| acc + item.priority);
    println!("Result priority sum: {}", result_priority_sum);
}

#[derive(Debug)]
struct Rucksack {
    compartments: Vec<Compartment>,
    shared_item: Option<Item>,
}

impl Rucksack {
    fn get_unique_items(&self) -> Vec<&Item> {
        let mut unique_items = HashSet::new();
        let mut items = Vec::new();
        for compartment in self.compartments.iter() {
            for item in compartment.items.iter() {
                unique_items.insert(item);
            }
        }
        unique_items.into_iter().for_each(|item| items.push(item));
        items
    }
}

#[derive(Debug)]
struct Compartment {
    items: Vec<Item>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Item {
    name: char,
    priority: u16,
}

fn parse_rucksacks(input: String) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = Vec::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let compartment_size = line.len() / 2;
        let mut compartments: Vec<Compartment> = Vec::new();
        let items: Vec<Item> = line
            .chars()
            .map(|item| {
                let mut byte_offset: u16 = 96;
                if item.is_uppercase() {
                    byte_offset = 38;
                }
                let priority = item as u16 - byte_offset;
                let unit = Item {
                    name: item,
                    priority: priority,
                };
                unit
            })
            .collect();
        compartments.push(Compartment {
            items: items[0..compartment_size].to_vec(),
        });
        compartments.push(Compartment {
            items: items[compartment_size..].to_vec(),
        });
        let mut shared_item = None;
        for item in &compartments[0].items {
            if compartments[1].items.contains(&item) {
                shared_item = Some(item.clone());
            }
        }
        rucksacks.push(Rucksack {
            compartments: compartments,
            shared_item: shared_item,
        });
    }
    rucksacks
}
