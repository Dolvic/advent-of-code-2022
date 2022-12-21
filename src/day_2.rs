use std::fs::File;
use std::io::{BufRead, BufReader};

const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOSE: u32 = 0;

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

pub(crate) fn part_1() -> u32 {
    let mut score = 0;
    for line in get_input() {
        let mut pieces = line.split_ascii_whitespace();
        let opponent = pieces.next().expect("First piece should exist");
        let player = pieces.next().expect("Second piece should exist");
        let match_score = match (opponent, player) {
            ("A", "Y") => WIN,
            ("A", "Z") => LOSE,
            ("B", "Z") => WIN,
            ("B", "X") => LOSE,
            ("C", "X") => WIN,
            ("C", "Y") => LOSE,
            _ => DRAW,
        };
        let piece_score = match player {
            "X" => ROCK,
            "Y" => PAPER,
            "Z" => SCISSORS,
            _ => panic!("Unrecognized player piece: {player}"),
        };
        score += match_score + piece_score;
    }
    score
}

pub(crate) fn part_2() -> u32 {
    let mut score = 0;
    for line in get_input() {
        let mut pieces = line.split_ascii_whitespace();
        let opponent = pieces.next().expect("First piece should exist");
        let outcome = pieces.next().expect("Outcome should exist");
        let match_score = match outcome {
            "X" => LOSE,
            "Y" => DRAW,
            "Z" => WIN,
            _ => panic!("Unrecognized outcome: {outcome}"),
        };
        let piece_score = match (opponent, outcome) {
            ("A", "X") => SCISSORS,
            ("A", "Y") => ROCK,
            ("A", "Z") => PAPER,
            ("B", "X") => ROCK,
            ("B", "Y") => PAPER,
            ("B", "Z") => SCISSORS,
            ("C", "X") => PAPER,
            ("C", "Y") => SCISSORS,
            ("C", "Z") => ROCK,
            _ => panic!("Unrecognized game: {opponent} {outcome}"),
        };
        score += match_score + piece_score;
    }
    score
}

fn get_input() -> impl Iterator<Item = String> {
    BufReader::new(File::open("inputs/day_2.txt").expect("Input file should be readable"))
        .lines()
        .map(|line| line.expect("Input should contain readable lines"))
}
