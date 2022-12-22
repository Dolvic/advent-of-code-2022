use std::char;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn part_1() -> u32 {
    let mut priority_score = 0;
    for line in get_input() {
        let length = line.len();
        let compartment_1: HashSet<char> = line[..length / 2].chars().collect();
        let compartment_2 = line[length / 2..].chars().collect();
        let common_letter = compartment_1
            .intersection(&compartment_2)
            .next()
            .expect("Should have 1 common element");
        priority_score += priority(*common_letter)
    }
    priority_score
}

fn priority(letter: char) -> u32 {
    match letter {
        'a'..='z' => (letter as u32) - ('a' as u32) + 1,
        'A'..='Z' => (letter as u32) - ('A' as u32) + 27,
        _ => panic!(),
    }
}

fn get_input() -> impl Iterator<Item = String> {
    BufReader::new(File::open("inputs/day_3.txt").expect("Input file should be readable"))
        .lines()
        .map(|line| line.expect("Input should contain readable lines"))
}
