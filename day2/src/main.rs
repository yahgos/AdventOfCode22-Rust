use std::fs;
fn main() {
    let INPUT = fs::read_to_string("./src/input/day2_input.txt").expect("Couldnt read input file");
    let games = parse_games(INPUT);
    let score = calc_score(games);
    // let score = calc_score_p1(games);
    println!("Score: {}", score);
}
#[derive(Debug)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
enum GameResult {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

//changes for part 2
fn parse_games(input: String) -> Vec<(RPS, GameResult)> {
    let games = input.lines().map(|line| {
        let mut parts = line.split(" ");
        println!("parts: {:?}", parts);
        
        let player1 = match parts.next().unwrap() {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => panic!("Invalid input"),
        };
        let player2 = match parts.next().unwrap() {
            "X"  => GameResult::Loss,
            "Y"  => GameResult::Draw,
            "Z"  => GameResult::Win,
            _ => panic!("Invalid input"),
        };
        (player1, player2)
    });
    games.collect()
}

fn calc_score(games: Vec<(RPS, GameResult)>) -> u32 {
    let mut score = 0;
    for game in games {
        let game_score = match game.0 {
          RPS::Rock => match game.1 {
                GameResult::Loss => RPS::Scissors as u32 + GameResult::Loss as u32,
                GameResult::Draw => RPS::Rock as u32 + GameResult::Draw as u32, 
                GameResult::Win => RPS::Paper as u32 + GameResult::Win as u32, 
            },
            RPS::Paper => match game.1 {
                GameResult::Loss => RPS::Rock as u32 + GameResult::Loss as u32,
                GameResult::Draw => RPS::Paper as u32 + GameResult::Draw as u32, 
                GameResult::Win => RPS::Scissors as u32 + GameResult::Win as u32,
            },
            RPS::Scissors => match game.1 {
                GameResult::Loss => RPS::Paper as u32 + GameResult::Loss as u32,
                GameResult::Draw => RPS::Scissors as u32 + GameResult::Draw as u32, 
                GameResult::Win => RPS::Rock as u32 + GameResult::Win as u32,
            },
        };
        score += game_score;
    }
    println!("Score: {}", score);
    score
}

fn calc_score_p1(games: Vec<(RPS, RPS)>) -> u32 {
    let mut score = 0;
    let WIN = 6;
    let LOSS = 0;
    let DRAW = 3;
    for game in games {
        let game_score = match game.0 {
          RPS::Rock => match game.1 {
                RPS::Rock => DRAW + 1,
                RPS::Paper => WIN + 2,
                RPS::Scissors => LOSS + 3,
            },
            RPS::Paper => match game.1 {
                RPS::Rock => LOSS + 1,
                RPS::Paper => DRAW + 2,
                RPS::Scissors => WIN + 3,
            },
            RPS::Scissors => match game.1 {
                RPS::Rock => WIN + 1,
                RPS::Paper => LOSS + 2,
                RPS::Scissors => DRAW + 3,
            },
        };
        score += game_score;
    }
    println!("Score: {}", score);
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let TEST_INPUT = fs::read_to_string("./src/input/test_input.txt").expect("Couldnt read input file");
        let games = parse_games(TEST_INPUT);
        assert_eq!(calc_score(games), 12);
    }
}
