use std::fs;
use std::str::FromStr;

#[derive(Debug)]
pub enum OpponentChoice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for OpponentChoice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(OpponentChoice::Rock),
            "B" => Ok(OpponentChoice::Paper),
            "C" => Ok(OpponentChoice::Scissors),
            _ => Err(format!("Invalid opponent choice: {}", s)),
        }
    }
}

const ROCK_SCORE: i32 = 1;
const PAPER_SCORE: i32 = 2;
const SCISSORS_SCORE: i32 = 3;

#[derive(Debug)]
pub enum OldPlayerChoice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for OldPlayerChoice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(OldPlayerChoice::Rock),
            "Y" => Ok(OldPlayerChoice::Paper),
            "Z" => Ok(OldPlayerChoice::Scissors),
            _ => Err(format!("Invalid player choice: {}", s)),
        }
    }
}

#[derive(Debug)]
pub enum PlayerChoice {
    Lose,
    Draw,
    Win,
}

impl FromStr for PlayerChoice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(PlayerChoice::Lose),
            "Y" => Ok(PlayerChoice::Draw),
            "Z" => Ok(PlayerChoice::Win),
            _ => Err(format!("Invalid player choice: {}", s)),
        }
    }
}

pub fn calculate_score(opponent_choice: OpponentChoice, player_choice: PlayerChoice) -> i32 {
    let mut score = 0;

    match player_choice {
        PlayerChoice::Lose => match opponent_choice {
            OpponentChoice::Rock => score += SCISSORS_SCORE,
            OpponentChoice::Paper => score += ROCK_SCORE,
            OpponentChoice::Scissors => score += PAPER_SCORE,
        },
        PlayerChoice::Draw => {
            score += 3;
            match opponent_choice {
                OpponentChoice::Rock => score += ROCK_SCORE,
                OpponentChoice::Paper => score += PAPER_SCORE,
                OpponentChoice::Scissors => score += SCISSORS_SCORE,
            }
        }
        PlayerChoice::Win => {
            score += 6;
            match opponent_choice {
                OpponentChoice::Rock => score += PAPER_SCORE,
                OpponentChoice::Paper => score += SCISSORS_SCORE,
                OpponentChoice::Scissors => score += ROCK_SCORE,
            }
        }
    }

    score
}

fn main() {
    let contents = include_str!("input.txt");

    let mut total_score = 0;

    for line in contents.lines() {
        let mut iter = line.split_whitespace();
        let opponent_choice = iter.next().unwrap().parse::<OpponentChoice>().unwrap();
        let player_choice = iter.next().unwrap().parse::<PlayerChoice>().unwrap();
        total_score += calculate_score(opponent_choice, player_choice)
    }

    println!("{}", total_score)
}
