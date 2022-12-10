use std::fs;

pub(crate) fn part_1() -> u32 {
    let mut max_calories = 0;
    let mut current_calories = 0;

    for calories in get_input() {
        if calories.is_empty() {
            max_calories = max_calories.max(current_calories);
            current_calories = 0;
        } else {
            let calories: u32 = calories.parse().expect("Unable to parse calorie input");
            current_calories += calories;
        }
    }
    max_calories
}

pub(crate) fn part_2() -> u32 {
    let mut max_calories = [0; 4];

    for calories in get_input() {
        if calories.is_empty() {
            max_calories.sort();
            max_calories[0] = 0;
        } else {
            let calories: u32 = calories.parse().expect("Unable to parse calorie input");
            max_calories[0] += calories;
        }
    }
    max_calories[1..].iter().sum()
}

fn get_input() -> Vec<String> {
    fs::read_to_string("inputs/day_1.txt")
        .expect("Input file should be readable")
        .lines()
        .map(|s| s.to_owned())
        .collect()
}
