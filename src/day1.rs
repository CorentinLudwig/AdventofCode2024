use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;



fn read_file(path_input: &str) -> (Vec<i32>, Vec<i32>) {
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
    let mut vec1: Vec<i32> = vec![];
    let mut vec2: Vec<i32> = vec![];

    for l in lines {
        match l {
            Ok(content) => {
                let numbers: Vec<i32> = content
                    .split_whitespace() // Split the string by whitespace
                    .filter_map(|s| s.parse::<i32>().ok()) // Parse each part into an i32, ignoring errors
                    .collect();
                    
                    vec1.push(numbers[0]);
                    vec2.push(numbers[1]);

                
            } // Successfully read a line
            Err(why) => eprintln!("Error reading a line: {}", why), // Handle errors in reading lines
        }
    }
    vec1.sort();
    vec2.sort();

    ( vec1, vec2 )

}

fn similarity(vec1:Vec<i32>, vec2:Vec<i32>) -> i32 {
    let mut similarity_score = 0;

    for i in 0..vec1.len() {
        let count: i32 = vec2.iter().filter(|&&x| x == vec1[i]).count() as i32;
        similarity_score += count * vec1[i];
    }
    return similarity_score;
}

pub fn day1(path_input: &str){



    let result = read_file(path_input);
    let mut distance = 0;

    for i in 0..result.0.len() {
        distance += (result.0[i] - result.1[i]).abs();
    }

    let similarity_score = similarity(result.0, result.1);


    println!("day 1 result: distance from the to list are {}, and the similarity score are {}",distance, similarity_score);

}