use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, u32};

fn main() {
    let current_dir = env::current_dir().unwrap();
    let file_path = current_dir.join("src/bin/01/input.txt");

    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);

    let mut biggest_amount_of_calories: u32 = 0;
    let mut sum: u32 = 0;

    for line in reader.lines() {
        let line_str = line.unwrap();

        if line_str != "" {
            sum += line_str
                .parse::<u32>()
                .expect("Deu ruim na hora de fazer o parse da linha");
        } else {
            sum = 0;
        }

        if sum > biggest_amount_of_calories {
            biggest_amount_of_calories = sum;
        }
    }

    println!(
        "The greatest amount of calories is: {}",
        biggest_amount_of_calories,
    );
}
