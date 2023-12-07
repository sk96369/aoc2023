use std::fs;

fn read_input(filename: &str) -> Vec<(u64, u64)> {
    let lines: Vec<String> = fs::read_to_string(filename).unwrap().lines()
        .map(|l| l.to_owned()).collect();
    let times: Vec<u64> = lines[0].split_whitespace().filter_map(|s| s.parse::<u64>().ok())
        .collect();
    let distances: Vec<u64> = lines[1].split_whitespace().filter_map(|s| s.parse::<u64>().ok())
        .collect();
    times.into_iter().zip(distances.into_iter()).collect()
}

fn read_input_part2(filename: &str) -> (u64, u64) {
    let lines: Vec<String> = fs::read_to_string(filename).unwrap().lines()
        .map(|l| l.to_owned()).collect();
    let time = lines[0].chars().filter(|c| c.is_numeric()).collect::<String>()
        .parse::<u64>().unwrap();
    let distance = lines[1].chars().filter(|c| c.is_numeric()).collect::<String>()
        .parse::<u64>().unwrap();
    (time, distance)
}

fn find_winning_strategies(race: &(u64, u64)) -> usize {
    (1..race.0 - 1).filter(|s| {
        let time = race.0 - s;
        time * s > race.1
    })
    .count()
}


fn main() {
    let races = read_input("input.txt");
    println!("{:?}", races);
    let product = races.iter().map(|r| {
        find_winning_strategies(r)
    })
    .product::<usize>();

    println!("{product}");

    //part2 
    let record = read_input_part2("input.txt");
    println!("{record:?}");
    let wins = find_winning_strategies(&record);
    println!("winning strategies: {wins}");
}
