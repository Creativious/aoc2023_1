use std::fs::File;
use std::io;
use std::io::{BufRead, read_to_string};
fn main() {
    let filename = "puzzle_input.txt";
    let file_lines = read_lines(filename);
    let mut total = 0;
    for line in file_lines {
        let first_number_str = get_first_number_in_string(line.as_str());
        let last_number_str = get_last_number_in_string(line.as_str());
        let mut combined_number_str = "".to_string();
        combined_number_str.push(first_number_str.chars().next().unwrap());
        combined_number_str.push(last_number_str.chars().next().unwrap());
        let combined_number: i32 = combined_number_str.parse().unwrap();
        total += combined_number;
    }
    println!("{}", total);
}

fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename);
    let lines = io::BufReader::new(file.unwrap()).lines();
    let mut lines_in_string = Vec::new();
    for line in lines {
        lines_in_string.push(line.unwrap());
    }
    lines_in_string
}

fn get_first_number_in_string(line: &str) -> String {
    let chars = get_chars_in_line(line);
    let mut first_number = "".to_string();
    let mut i = 0;
    let mut x = 0;
    for character in chars {
        if (i != 0) {
            i -= 1;
            continue;
        }
        let number = get_number_from_char(character);
        if number != 10 {
            first_number = character.to_string();
            break;
        }
        else {
            if is_char_a_starter_char_for_a_number_word(character) {
                let maybe_num = get_number_from_position_in_line(line, x);
                if (maybe_num != 10) {
                    first_number = maybe_num.to_string();
                    break;
                }
            }
        }
        x += 1;
    }
    return first_number;
}

fn get_last_number_in_string(line: &str) -> String {
    let chars = get_chars_in_line(line);
    let mut last_number = "".to_string();
    let mut i = 0;
    let mut x = 0;
    for character in chars {
        if (i != 0) {
            i -= 1;
            continue;
        }
        let number = get_number_from_char(character);
        if number != 10 {
            last_number = character.to_string();
        }
        else {
            if is_char_a_starter_char_for_a_number_word(character) {
                let maybe_num = get_number_from_position_in_line(line, x);
                if (maybe_num != 10) {
                    last_number = maybe_num.to_string();
                }
            }
        }
        x += 1;
    }
    return last_number;
}

fn get_number_of_indexes_to_skip_from_number(number: i8) -> i8 {
    match number {
        1 => {
            3
        }
        2 => {
            3
        }
        3 => {
            5
        }
        4 => {
            4
        }
        5 => {
            4
        }
        6 => {
            3
        }
        7 => {
            5
        }
        8 => {
            5
        }
        9 => {
            4
        }
        _ => {
            0
        }
    }
}

fn get_number_from_position_in_line(line: &str, position: i8) -> i8 {
    let mut i = 0;
    let mut found_start_position = false;
    let mut makeup_string = "".to_string();
    let mut result_num = 0;
    for character in line.chars() {
        if position == i || found_start_position{
            found_start_position = true;
            makeup_string.push(character);
        }
        result_num = match makeup_string.as_str() {
            "one" => {
                1
            }
            "two" => {
                2
            }
            "three" => {
                3
            }
            "four" => {
                4
            }
            "five" => {
                5
            }
            "six" => {
                6
            }
            "seven" => {
                7
            }
            "eight" => {
                8
            }
            "nine" => {
                9
            }
            _ => {
                10
            }
        };
        if (result_num != 10) {
            break;
        }
        i += 1;
    }
    result_num
}

fn is_char_a_starter_char_for_a_number_word(character: char) -> bool {
    match character.to_string().as_str() {
        "o" => {
            true
        }
        "t" => {
            true
        }
        "f" => {
            true
        }
        "s" => {
            true
        }
        "e" => {
            true
        }
        "n" => {
            true
        }
        _ => {
            false
        }
    }
}

fn get_number_from_char(character: char) -> i8 {
    match character.to_string().as_str() {
        "0" => {
            0
        }
        "1" => {
            1
        }
        "2" => {
            2
        }
        "3" => {
            3
        }
        "4" => {
            4
        }
        "5" => {
            5
        }
        "6" => {
            6
        }
        "7" => {
            7
        }
        "8" => {
            8
        }
        "9" => {
            9
        }
        _ => {
            10
        }
    }
}

fn get_chars_in_line(line: &str) -> Vec<char> {
    let mut chars = Vec::new();
    for char in line.chars() {
        chars.push(char);
    }
    return chars;
}