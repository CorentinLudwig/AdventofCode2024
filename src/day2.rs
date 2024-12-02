use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


fn problem_dampler(line: &Vec<i32>, bad_level: usize) -> bool {
    let mut line_dampler = line.clone();
    line_dampler.remove(bad_level);

    let is_safe = is_safe_level(&line_dampler, false);

    return if is_safe == 1 { true } else { false };
}

fn is_safe_level(numbers: &Vec<i32>, use_problem_dampler: bool) -> i32 {
    let size = numbers.len();
    let mut is_safe = true;
    let is_increasing = if numbers[0] < numbers[1] { true } else { false };


    if numbers[0] == numbers[1] {
        if use_problem_dampler {
            is_safe = problem_dampler(numbers, 0);
            return if is_safe { 1 } else { 0 };
        } else {
            return 0;
        }
        
    }

    for i in 0..(size - 1) {
        let test = numbers[i] - numbers[i + 1];

        if is_increasing && test > 0 || !is_increasing && test < 0 {
            if use_problem_dampler {
                // Try different indices: i, i+1, 0, and 1
                let indices_to_try = vec![i, i + 1, 0, 1];
                for &index in &indices_to_try {
                    is_safe = problem_dampler(numbers, index);
                    if is_safe {
                        break; // Break out of the loop as soon as we find a safe state
                    }
                }
            } else {
                is_safe = false;
            }
            if !is_safe {
                break; // Exit the loop if the sequence is unsafe
            }
        }

        if test.abs() > 3 || test.abs() < 1 {
            if use_problem_dampler {
                is_safe = problem_dampler(numbers, i);
                if !is_safe {
                    is_safe = problem_dampler(numbers, i + 1);
                }
            } else {
                is_safe = false;
            }
            if !is_safe {
                break;
            }
        }
    }

    return if is_safe { 1 } else { 0 };
}


pub fn day2(path_input: &str) {
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
    let mut safe_level = 0;

    for l in lines {
        match l {
            Ok(content) => {
                let numbers: Vec<i32> = content
                    .split_whitespace() // Split the string by whitespace
                    .filter_map(|s| s.parse::<i32>().ok()) // Parse each part into an i32, ignoring errors
                    .collect();
                    safe_level += is_safe_level(&numbers, true);

                
            } // Successfully read a line
            Err(why) => eprintln!("Error reading a line: {}", why), // Handle errors in reading lines
        }
    }

    println!("day 2 result: number of safe level = {}", safe_level);
}
