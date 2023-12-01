use std::{collections::HashMap, fs};
use regex::{Captures, Regex};

fn read_input(filename: &str) -> Vec<String> {
    let file = fs::read_to_string(filename)
        .expect("Could not read file {filename}");
    file.split_terminator("\r\n")
        .filter_map(|x| {
            if x != "" {
                Some(x.to_owned())
            } else {
                None
            }
        })
        .collect()
}

fn fix_line(line: &str) -> u32 {
    let digits = line.chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();
    (digits.first().unwrap_or(&0) * 10) + digits.last().unwrap_or(&0)
}

fn fix_numerals(line: &str, numeral_replacements: &HashMap<&str, &str>) -> String {
    //all the input text seems to be in lower case
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|zero)").unwrap();
    let replacement = |caps: &Captures| {
        numeral_to_digit(&caps[0], numeral_replacements)
    };
    re.replace_all(&line, &replacement).into_owned()
}

fn numeral_to_digit(numeral: &str, dict: &HashMap<&str, &str>) -> String {
    format!("{}", dict.get(numeral).unwrap())
}

fn main() {
    let inputs = read_input("input.txt");
    let numeral_replacements = HashMap::from([
        ("one", "1ne"),
        ("two", "2wo"),
        ("three", "3hree"),
        ("four", "4our"),
        ("five", "5ive"),
        ("six", "6ix"),
        ("seven", "7even"),
        ("eight", "8ight"),
        ("nine", "9ine"),
    ]);

    // Second task: edit the input to turn all numerals into digits
    let numbers = inputs.iter()
        .map(|line| {
            let line_fixed_numerals = fix_numerals(line, &numeral_replacements);
            // Stupid and ugly solution for overlapping typed numbers: run it twice
            let line_fixed_numerals = fix_numerals(&line_fixed_numerals, &numeral_replacements);
            //println!("{line_fixed_numerals}");
            let fixed_line = fix_line(&line_fixed_numerals);
            //println!("{fixed_line}\n");
            fixed_line
        })
        .collect::<Vec<u32>>();
    println!("{}", numbers.iter().sum::<u32>());
}
