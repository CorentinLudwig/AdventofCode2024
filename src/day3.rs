use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_file(path_input: &str) -> usize {
    let file = match File::open(path_input) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!(
            "couldn't open inputDay2.txt: {}",
            <dyn Error>::to_string(&why)
        ),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut sum: usize = 0;
    let mut is_do: bool = true;

    for l in lines {
        match l {
            Ok(content) => {
                sum += find_mul(&content, &mut is_do);
            } // Successfully read a line
            Err(why) => eprintln!("Error reading a line: {}", why), // Handle errors in reading lines
        }
    }
    return sum;
}

fn find_mul(l: &str, is_do: &mut bool) -> usize {
    let mut sum: usize = 0;
    let pattern_mul = Regex::new(r"(mul\(([0-9]{1,3}),([0-9]{1,3})\)|(do\(\))|(don't\(\)))").unwrap();

    for caps in pattern_mul.captures_iter(l) {
        // check if it match a do()
        if caps.get(4).is_some() {
            *is_do = true;
            // check if it match a don't()
        } else if caps.get(5).is_some() {
            *is_do = false;
        } else if *is_do {
            // check if it match a mul
            if caps.get(1).is_some() {
                let number1: usize = caps[2].parse().unwrap();
                let number2: usize = caps[3].parse().unwrap();
                sum += number1 * number2;
            }
        }
    }

    return sum;
}

pub fn day3(path_input: &str) {
    println!(
        "day 3 result: sum of multiplications = {}",
        read_file(path_input)
    );
}
