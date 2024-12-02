mod day2;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("use: {} <day number>\n", args[0]);
    }


    let day:i32;

    match args[1].parse::<i32>() {
        Ok(value) => day = value,
        Err(e) => panic!("Failed to parse the string: {}\n", e),
    }

    match day {
        |2 => println!("day 2 result: number of safe level = {}", day2::day2("input/inputDay2.txt")),
        | _ => panic!("this day is not impemented or don't exist: {}\n", day),
    }
}
