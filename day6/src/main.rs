fn main() {
    let INPUT = "C:/rust/advent_2022/day6/input/input.txt";
    let input = std::fs::read_to_string(INPUT).unwrap();
    // let pos = get_sop_pos(&input);
    // println!("SOP pos: {}", pos);
    let msg_pos = get_somessage_pos(&input);
    println!("SOM pos: {}", msg_pos);
}

fn get_sop_pos(signal: &str) -> usize {
    let mut pos = 0;
    let chars = signal.chars().collect::<Vec<char>>();
    let windows_vec = chars.windows(4);
    for (index, candidate) in windows_vec.enumerate() {
        if !has_repeated_chars(&candidate.to_vec()) {
            println!("Candidate: {:?}", candidate);
            pos = 4 + index;
            break;
        }
    }
    pos
}
fn get_somessage_pos(signal: &str) -> usize {
    let mut pos = 0;
    let chars = signal.chars().collect::<Vec<char>>();
    let windows_vec = chars.windows(14);
    for (index, candidate) in windows_vec.enumerate() {
        if !has_repeated_chars(&candidate.to_vec()) {
            println!("Candidate: {:?}", candidate);
            pos = 14 + index;
            break;
        }
    }
    pos
}

fn has_repeated_chars(candidate: &Vec<char>) -> bool {
    let mut seen_chars = std::collections::HashSet::new();
    candidate.iter().any(|c| {
        if seen_chars.contains(c) {
            true
        } else {
            seen_chars.insert(c);
            false
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(get_sop_pos("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(get_sop_pos("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(get_sop_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(get_sop_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test2() {
        assert_eq!(get_somessage_pos("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(get_somessage_pos("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(get_somessage_pos("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(get_somessage_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    }
}
