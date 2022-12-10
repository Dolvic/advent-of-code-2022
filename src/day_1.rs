use std::io;

pub(crate) fn part_1() {
    let mut max_calories = 0;
    let mut current_calories = 0;

    for calories in io::stdin().lines() {
        let calories = calories.expect("Unable to read calorie input");

        if calories.is_empty() {
            max_calories = max_calories.max(current_calories);
            current_calories = 0;
        } else {
            let calories: u32 = calories.parse().expect("Unable to parse calorie input");
            current_calories += calories;
        }
    }
    println!("{max_calories}")
}
