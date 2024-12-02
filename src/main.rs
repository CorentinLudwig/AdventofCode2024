mod day2;
mod day1;
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
        | 1 => day1::day1("input/inputDay1.txt"),
        | 2 => day2::day2("input/inputDay2.txt"),
        | _ => panic!("this day is not impemented or don't exist: {}\n", day),
    }
}
