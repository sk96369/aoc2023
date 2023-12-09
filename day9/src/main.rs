use std::{fs, time::Instant};

fn read_input(filename: &str) -> Vec<Vec<i64>> {
    fs::read_to_string(filename).unwrap().lines().map(|l| {
        l.split_whitespace().filter_map(|s| s.parse::<i64>().ok()).collect()
    })
    .collect()
}

fn get_differences(values: &Vec<i64>) -> Vec<i64> {
    values[0..values.len() - 1].iter().zip(values[1..].iter()).map(|(v1, v2)| {
        v2 - v1
    })
    .collect()
}

fn get_next_value(values: &Vec<i64>) -> i64 {
    let mut layers = vec![values.clone()];
    loop {
        let differences = get_differences(layers.iter().last().unwrap());
        layers.push(differences);
        if layers.iter().last().unwrap().iter().all(|v| v == &0) {
            break;
        }
    }
    let mut last = 0;
    layers[0..layers.len() - 1].iter().rev().for_each(|l| {
        last = l.iter().last().unwrap() + last;
    });
    last
}

fn get_previous_value(values: &Vec<i64>) -> i64 {
    let mut layers = vec![values.clone()];
    loop {
        let differences = get_differences(layers.iter().last().unwrap());
        layers.push(differences);
        if layers.iter().last().unwrap().iter().all(|v| v == &0) {
            break;
        }
    }
    let mut last = 0;
    layers[0..layers.len() - 1].iter().rev().for_each(|l| {
        last = l.iter().next().unwrap() - last;
    });
    last
}

fn main() {
    let now = Instant::now();
    let input = read_input("input.txt");
    let elapsed = now.elapsed();
    println!("Inputs read in {} microseconds", elapsed.as_micros());

    let next_values: Vec<i64> = input.iter().map(|v| get_next_value(v)).collect();
    println!("Sum: {}", next_values.iter().sum::<i64>());
    let elapsed = now.elapsed();
    println!("Part 1 calculated in {} microseconds", elapsed.as_micros());

    let previous_values: Vec<i64> = input.iter().map(|v| get_previous_value(v)).collect();
    println!("Sum: {}", previous_values.iter().sum::<i64>());
    let elapsed = now.elapsed();
    println!("Part 2 calculated in {} microseconds", elapsed.as_micros());

}
