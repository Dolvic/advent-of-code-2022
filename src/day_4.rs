use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn part_1() -> u32 {
    let mut contained_ranges = 0;
    for line in get_input() {
        let mut ranges = line
            .splitn(4, ['-', ','])
            .map(|bound| bound.parse().expect("Range bound to be a u32"));

        let range1_lower = ranges.next().expect("Range bound to exist");
        let range1_upper = ranges.next().expect("Range bound to exist");
        let range2_lower = ranges.next().expect("Range bound to exist");
        let range2_upper = ranges.next().expect("Range bound to exist");

        if range_contained(range1_lower, range1_upper, range2_lower, range2_upper) {
            contained_ranges += 1;
        }
    }
    contained_ranges
}

fn range_contained(r1_lower: u32, r1_upper: u32, r2_lower: u32, r2_upper: u32) -> bool {
    (r1_lower <= r2_lower && r1_upper >= r2_upper) || (r2_lower <= r1_lower && r2_upper >= r1_upper)
}

fn get_input() -> Vec<String> {
    BufReader::new(File::open("inputs/day_4.txt").expect("Input file should be readable"))
        .lines()
        .map(|line| line.expect("Input should contain readable lines"))
        .collect()
}
