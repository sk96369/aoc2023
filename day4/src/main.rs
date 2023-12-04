use std::fs;

fn read_input(input: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    let file: Vec<String> = fs::read_to_string(input).expect("could not open file")
        .lines()
        .filter_map(|line| {
            if line != "" {
                Some(line.split_terminator('.').last().unwrap().to_owned())
            } else {
                None
            }
        })
        .collect();

    file.iter().map(|line| {
        let game: Vec<Vec<u32>> = line.split_terminator('|')
            .map(|g| {
                g.split_terminator(' ')
                    .filter_map(|n| n.parse::<u32>().ok())
                    .collect()
            })
            .collect();
        (game[0].clone(), game[1].clone())
    })
    .collect()
}

fn calculate_win(winning_numbers: &Vec<u32>, player_numbers: &Vec<u32>) -> u32 {
    let mut score = 0;
    player_numbers.iter().for_each(|n| {
        if winning_numbers.contains(n) {
            if score == 0 {
                score = 1;
            } else {
                score = score * 2;
            }
        }
    });
    score
}

fn calculate_new_tickets(winning_numbers: &Vec<u32>, player_numbers: &Vec<u32>) -> u32 {
    let mut score = 0;
    player_numbers.iter().for_each(|n| {
        if winning_numbers.contains(n) {
            score += 1;
        }
    });
    score
}

fn main() {
    let games = read_input("input.txt");
    let wins: Vec<u32> = games.iter().map(|g| calculate_win(&g.0, &g.1))
        .collect();
    println!("Total winnings for round 1: {}", wins.iter().sum::<u32>());
    
    //Using the correct rules from part 2
    let wins: Vec<u32> = games.iter().map(|g| calculate_new_tickets(&g.0, &g.1))
        .collect();

    let mut ticket_counts = vec![1_u32; games.len()];
    wins.iter().enumerate().for_each(|(i, t)| {
        let t_as_usize = usize::try_from(*t).unwrap();
        for j in (i + 1)..(i + 1 + t_as_usize) {
            ticket_counts[j] += ticket_counts[i]
        }
    });
    println!("Total number of tickets: {}", ticket_counts.iter().sum::<u32>());
}
