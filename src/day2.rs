use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


fn problem_dampler(line: &Vec<i32>, indices_to_try: &Vec<usize>) -> bool {
    let mut is_safe = 0;
    for index in indices_to_try {
        let mut line_dampler = line.clone();
        line_dampler.remove(*index);
        is_safe = is_safe_level(&line_dampler, false);
        if is_safe != 0 {
            break; // Break out of the loop as soon as we find a safe state
        }
    }


    return if is_safe == 1 { true } else { false };
}

fn is_safe_level(line: &Vec<i32>, use_problem_dampler: bool) -> i32 {
    let size = line.len();
    let mut is_safe = true;
    let is_increasing = if line[0] < line[1] { true } else { false };


    if line[0] == line[1] {
        if use_problem_dampler {
            let indice_to_test = vec![0];
            is_safe = problem_dampler(line, &indice_to_test);
            return if is_safe { 1 } else { 0 };
        } else {
            return 0;
        }
        
    }

    for i in 0..(size - 1) {
        let test = line[i] - line[i + 1];


        // test if the level is only increasing or decreasing
        if is_increasing && test > 0 || !is_increasing && test < 0 {
            if use_problem_dampler {
                // Try different indices: i, i+1, 0, and 1
                let indices_to_try = vec![i, i + 1, 0, 1];
                    is_safe = problem_dampler(line, &indices_to_try);
                } else {
                    is_safe = false;
                }
            if !is_safe {
                break; // Exit the loop if the sequence is unsafe
            }
        }

        //test if there is not too much difference beetween 2 level
        if test.abs() > 3 || test.abs() < 1 {
            if use_problem_dampler {
                let indices_to_test = vec![i, i+1];
                is_safe = problem_dampler(line, &indices_to_test);
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
            "couldn't open {}: {}", path_input,
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
