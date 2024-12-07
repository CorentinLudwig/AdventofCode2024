mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("use: {} <day number>\n (optionnal<path to file)", args[0]);
    }


    let day:i32;


    match args[1].parse::<i32>() {
        Ok(value) => day = value,
        Err(e) => panic!("Failed to parse the string: {}\n", e),
    }



    let mut file: String = String::new(); 

    if args.len() < 3 {
        file = format!("input/inputDay{}.text", day);
    } else {
        file = args[2].clone();
    }

    match day {
        | 1 => day1::day1(&file),
        | 2 => day2::day2(&file),
        | 3 => day3::day3(&file),
        | 4 => day4::day4(&file),
        | 5 => day5::day5(&file),
        | _ => panic!("this day is not impemented or don't exist: {}\n", day),
    }
}
