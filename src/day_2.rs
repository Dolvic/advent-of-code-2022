use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn part_1() -> u32 {
    let mut score = 0;
    for line in get_input() {
        let mut pieces = line.split_ascii_whitespace();
        let opponent = pieces.next().expect("First piece should exist");
        let player = pieces.next().expect("Second piece should exist");
        let match_score = match (opponent, player) {
            ("A", "Y") => 6,
            ("A", "Z") => 0,
            ("B", "Z") => 6,
            ("B", "X") => 0,
            ("C", "X") => 6,
            ("C", "Y") => 0,
            _ => 3,
        };
        let piece_score = match player {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("Unrecognized player piece: {player}"),
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
