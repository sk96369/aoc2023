use std::{fs, collections::HashMap};

fn read_input(filename: &str) -> (Vec<(i64, i64)>, Vec<usize>, Vec<usize>) {
    let input = fs::read_to_string(filename).unwrap();
    let x = input.lines().peekable().peek().unwrap().len();
    let input: String = input.lines().filter(|l| !l.is_empty()).collect();
    let mut empty_x: Vec<usize> = vec![1; x];
    let mut empty_y: Vec<usize> = vec![1; input.len() / x];
    let universe = input.chars().enumerate().filter_map(|(i, c)| {
        if c == '#' {
            empty_x[i % x] = 0;
            empty_y[i / x] = 0;
            Some(((i / x) as i64, (i % x) as i64))
        } else {
            None
        }
    })
    .collect::<Vec<(i64, i64)>>();

    (universe, empty_x, empty_y)
}

fn get_expanded(universe_parameters: &(Vec<(i64, i64)>, Vec<usize>, Vec<usize>), factor: i64) -> Vec<(i64, i64)> {
    let mut universe = universe_parameters.0.clone();
    let mut universe_delta = HashMap::new();
    universe.iter().for_each(|(y, b)| {
        universe_delta.insert((*y, *b), (0, 0));
    });
    let empty_x = &universe_parameters.1;
    let empty_y = &universe_parameters.2;
    empty_x.iter().enumerate().for_each(|(i, x)| {
        if *x == 1 {
            universe.iter_mut().for_each(|(uy, ux)| {
                if *ux > i as i64 {
                     universe_delta.entry((*uy, *ux)).and_modify(|(_, b)| {
                        *b += 1;
                     });
                }
            })
        }
    });

    empty_y.iter().enumerate().for_each(|(i, y)| {
        if *y == 1 {
            universe.iter_mut().for_each(|(uy, ux)| {
                if *uy > i as i64 {
                     universe_delta.entry((*uy, *ux)).and_modify(|(a, _)| {
                        *a += 1;
                     });
                }
            })
        }
    });
    
    universe.iter_mut().for_each(|(y, x)| {
        let deltas = universe_delta.get(&(*y, *x)).unwrap();
        *y += deltas.0 * factor - deltas.0;
        *x += deltas.1 * factor - deltas.1;
    });
    universe
}


fn calculate_all_distances(universe: &Vec<(i64, i64)>) -> i64 {
    universe[0..universe.len() - 1].iter().enumerate().map(|(i,(ay, ax))| {
        universe[i + 1..].iter().map(|(by, bx)| {
            let distance = (ay - by).abs() + (ax - bx).abs();
            distance
        })
        .sum::<i64>()
    })
    .sum::<i64>()
}

fn main() {
    let universe = read_input("input.txt");
    let expanded_universe = get_expanded(&universe, 2);
    let sum_of_distances = calculate_all_distances(&expanded_universe);
    println!("Sum of all distances: {sum_of_distances}");

    let mega_expanded_universe = get_expanded(&universe, 1_000_000);
    let sum_of_distances = calculate_all_distances(&mega_expanded_universe);
    println!("Sum of all distances: {sum_of_distances}");
}
