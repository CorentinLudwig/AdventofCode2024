use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

fn read_file(path_input: &str) -> Vec<Vec<char>> {
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
    let mut data: Vec<Vec<char>> = Vec::new();

    for l in lines {
        match l {
            Ok(content) => {
                data.push(content.chars().collect());
            } // Successfully read a line
            Err(why) => eprintln!("Error reading a line: {}", why), // Handle errors in reading lines
        }
    }
    return data;
}

fn search_xmas(file: &Vec<Vec<char>>, i: i32, j: i32, di: i32, dj: i32) -> bool {
    for e in 1..XMAS.len() {
        let i_test: usize = (i + e as i32 * di) as usize;
        let j_test: usize = (j + e as i32 * dj) as usize;

        if i_test >= file.len() || j_test >= file[i_test].len() || file[i_test][j_test] != XMAS[e] {
            return false;
        }

        if file[i_test][j_test] != XMAS[e] {
            return false;
        }
    }
    return true;
}

fn find_xmas(file: &Vec<Vec<char>>) -> usize {
    let mut res = 0;

    let nb_colone = file[0].len();
    let nb_ligne = file.len();
    for i in 0..nb_ligne {
        for j in 0..nb_colone {
            if file[i][j] == 'X' {
                for di in -1..2 {
                    for dj in -1..2 {
                        if search_xmas(&file, i as i32, j as i32, di, dj) {
                            res += 1;
                        }
                    }
                }
            }
        }
    }

    return res;
}

fn find_x_mas(file: &Vec<Vec<char>>) -> usize {
    let mut res = 0;

    let nb_ligne = file.len();
    let nb_colone = file[0].len();

    for i in 1..nb_ligne - 1 {
        for j in 1..nb_colone - 1 {
            if file[i][j] == 'A' {
                // Check diagonals for "X MAS" pattern
                if file[i - 1][j - 1] == 'M' && file[i + 1][j + 1] == 'S' {
                    if file[i + 1][j - 1] == 'M' && file[i - 1][j + 1] == 'S' {
                        res += 1;
                        println!(" XMAX found to {},{}",i,j);
                    } else if file[i - 1][j + 1] == 'M' && file[i + 1][j - 1] == 'S' {
                        res += 1;
                        println!(" XMAX found to {},{}",i,j);
                    }
                }
                else if file[i - 1][j - 1] == 'S' && file[i + 1][j + 1] == 'M' {
                        if file[i + 1][j - 1] == 'M' && file[i - 1][j + 1] == 'S' {
                            res += 1;
                            println!(" XMAX found to {},{}",i,j);
                        } else if file[i - 1][j + 1] == 'M' && file[i + 1][j - 1] == 'S' {
                            res += 1;
                            println!(" XMAX found to {},{}",i,j);
                        }
                    }
                }
            }
        }
    return res;
}

pub fn day4(path_input: &str) {
    let file = read_file(path_input);

    println!(
        "day 4 result: number of XMAS = {}, number of X-MAS {}",
        find_xmas(&file),
        find_x_mas(&file)
    );
}
