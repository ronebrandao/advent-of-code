use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, i32};

fn main() {
    let current_dir = env::current_dir().unwrap();
    let file_path = current_dir.join("src/bin/01/input.txt");

    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);

    let mut biggest = vec![];
    let mut sum: i32 = 0;

    for line in reader.lines() {
        let line_str = line.unwrap();

        if line_str != "" {
            sum += line_str
                .parse::<i32>()
                .expect("Deu ruim na hora de fazer o parse da linha");
        } else {
            biggest.push(sum);
            sum = 0;
        }
    }

    biggest.sort_by_key(|&x| -x);

    println!("The greatest amount of calories is: {}", biggest[0]);

    println!(
        "Thre sum of the 3 greatest is: {}",
        biggest[0] + biggest[1] + biggest[2]
    );
}
