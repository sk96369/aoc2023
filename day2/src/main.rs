use std::{collections::HashMap, iter::zip, fs, cmp};
use regex::{Regex, Captures};

fn read_input(filename: &str) -> Vec<(u8, String)> {
    let input = fs::read_to_string(filename).expect("Could not read file {filename}")
        .split_terminator("\r\n")
        .filter_map(|x| {
            if x != "" {
                Some(x.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();
    zip(1.., input).collect()
}

//Not proud of this one
fn remove_impossible_games(games: &Vec<(u8, String)>) -> Vec<(u8, String)> {
    let color_rules = HashMap::from([
        ("red", 12_u8),
        ("green", 13_u8),
        ("blue", 14_u8),
    ]);
    let games: Vec<(u8, String)> = games.iter().map(|(id, line)| {
        let new_line = line.split_terminator(": ").last().unwrap().to_owned();
        (*id, new_line)
    })
    .collect();
    let re = Regex::new(r"([0-9]*[\s]*(red|blue|green))").unwrap();
    games.into_iter().filter(|(_, line)| {
        match re.captures_iter(line).try_for_each(|cap| {
            let draw: Vec<&str> = cap[0].split_terminator(" ").collect();
            //Compares the number of balls of a color to the maximum
            //number allowed by the color rules.
            //If the number of balls drawn is greater than the maximum allowed,
            //returns None and the try_for_each iteration ends, returning
            //None
            if &draw[0].parse::<u8>().unwrap() > color_rules.get(draw[1]).unwrap() {
                None
            } else {
                Some(())
            }
        }) {
            None => false,
            _ => true,
        }
    })
    .collect()

}

fn get_minimums(games: Vec<(u8, String)>) -> Vec<(u8, u8, u8)> {
    let re = Regex::new(r"([0-9]*[\s]*(red|blue|green))").unwrap();

    games.iter().map(|(_, line)| {
        println!("{:#?}", line);
        let (mut rmax, mut bmax, mut gmax) = (0, 0, 0);
        re.captures_iter(line).for_each(|cap| {
            let draw: Vec<&str> = cap[0].split_terminator(" ").collect();
            match draw[1] {
                "red" => rmax = cmp::max(draw[0].parse::<u8>().unwrap(), rmax),
                "green" => gmax = cmp::max(draw[0].parse::<u8>().unwrap(), gmax),
                "blue" => bmax = cmp::max(draw[0].parse::<u8>().unwrap(), bmax),
                _ => {
                    println!("COULD NOT PARSE NAME OF COLOR");
                    std::process::exit(0);
                },
            };
        });
        (rmax, bmax, gmax)
    })
    .collect()
}

fn main() {
    let input = read_input("input.txt");

    let possible_games = remove_impossible_games(&input);

    let mut sum_of_ids = 0;
    possible_games.iter().for_each(|(id, _)| sum_of_ids += usize::from(*id));
    println!("sum of ids: {sum_of_ids}");

    let minimums: Vec<(u8, u8, u8)> = get_minimums(input);
    let sum_of_powers: usize = minimums.iter().map(|(r, g, b)| {
        usize::from(*r) * usize::from(*g) * usize::from(*b)
    })
    .sum();
    println!("Sum of powers: {}", sum_of_powers);
}
