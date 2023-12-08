use std::{collections::HashMap, fs, time::Instant};
use regex;

fn read_input(filename: &str) -> (String, HashMap<String, [String; 2]>) {
    let mut map: HashMap<String, [String; 2]> = HashMap::new();
    let file = fs::read_to_string(filename).unwrap();
    let mut lines = file.lines();
    let moves = lines.next().unwrap().to_owned();
    let re = regex::Regex::new(r" = \(|, |\)").unwrap();

    lines.for_each(|l| {
        let codes:Vec<&str> = re.split(l).collect();
        if codes.len() >= 3 {
            map.entry(codes[0].to_owned()).or_insert([codes[1].to_owned(), codes[2].to_owned()]);
        }
    });
    (moves.to_owned(), map)
}


fn find_steps(instructions: &(String, HashMap<String, [String; 2]>)) -> usize {
    let mut steps = 0;
    let directions = &instructions.1;
    let path: Vec<usize> = instructions.0.chars().map(|c| {
        if c == 'L' {
            0
        } else {
            1
        }
    })
    .collect();

    let mut loc = "AAA".to_string();
    loop {
        loc = directions.get(&loc).unwrap()[path[steps % path.len()]].clone();
        steps += 1;
        if &loc == "ZZZ" {
            break;
        }
    };
    steps
}

//([non-repeating end points], first_repeating_end_point, [steps_between_end_points]
type TravelPattern = (Vec<usize>, usize, Vec<usize>);

fn get_pattern(start: &str, directions: &Vec<usize>, map: &HashMap<String, [String; 2]>) -> TravelPattern {
    let mut end_points: Vec<(&str, usize, usize)> = vec![];
    let mut step = 0;
    let mut current = start;
    loop {
        current = &map.get(current).unwrap()[directions[step % directions.len()]];
        step += 1;
        if current.chars().last().unwrap() == 'Z' {
            let new_end_point = (current, step % directions.len(), step);
            end_points.push(new_end_point);
            if end_points.len() > 1 {
                let duplicate_end_point = end_points.iter()
                    .position(|(k, p, s)| {
                        k == &new_end_point.0 &&
                            p == &new_end_point.1 &&
                            s != &new_end_point.2
                    });
                if let Some(d) = duplicate_end_point {
                    return (end_points[..d].into_iter().map(|ep| ep.2).collect::<Vec<usize>>(),
                    end_points[d].2,
                    end_points[d..(end_points.len() - 1)]
                    .into_iter().zip(end_points[(d + 1)..(end_points.len())].iter())
                    .map(|(a, b)| b.2 - a.2)
                    .collect::<Vec<usize>>()
                    )
                }
            }
        }
    }
}

fn find_first_intersection(paths: &mut Vec<TravelPattern>) -> usize {
    paths.sort_by_key(|p| p.1);
    let mut combined_nrp = paths[0].0.clone();
    combined_nrp.push(paths[0].1);
    let first_common_end_point = combined_nrp.iter().find_map(|end| {
        let all_comparisons: Vec<bool> = paths[1..].iter().map(|(onrp, os, _)| {
            let mut same_end_point = false;
            let mut other_ends = onrp.clone();
            other_ends.push(*os);
            other_ends.iter().for_each(|oend| {
                if end == oend {
                    same_end_point = true;
                }
            });
            same_end_point
        })
        .collect();
        if all_comparisons.into_iter().all(|x| x) {
            Some(*end)
        } else {
            None
        }
    });

    match first_common_end_point {
        //Found and easy match
        Some(ep) => ep,
        //If (lol) we need to iterate over the matches 
        None => {
            let mut first_path = (0_usize, paths[0].clone());
            let mut other_paths: Vec<(usize, TravelPattern)> = paths[1..].iter()
                .map(|p| (0_usize, p.clone())).collect();
            while !other_paths.iter().all(|(_, (_, c, _))| c == &first_path.1.1) {
                first_path.1.1 += first_path.1.2[first_path.0 % first_path.1.2.len()];
                first_path.0 += 1;
                other_paths.iter_mut().for_each(|p| {
                    while p.1.1 < first_path.1.1 {
                        p.1.1 += p.1.2[p.0 % p.1.2.len()];
                        p.0 += 1;
                    }
                });
                println!("value : {:?}", first_path.1.1);
            }
            first_path.1.1
        }
    }
}

fn find_all_steps(instructions: &(String, HashMap<String, [String; 2]>)) -> usize {
    let directions = &instructions.1;
    let path: Vec<usize> = instructions.0.chars().map(|c| {
        if c == 'L' {
            0
        } else {
            1
        }
    })
    .collect();

    let starts: Vec<String> = directions.keys()
        .filter(|k| {
            k.chars().last().unwrap() == 'A'
        }).map(|k| k.to_owned())
        .collect();

    //find repeating patterns for all paths and calculate the intersection
    //where they all finish on Z
    let mut all_patterns = starts.iter().map(|s| get_pattern(s, &path, directions))
        .collect();
    find_first_intersection(&mut all_patterns)
}

fn main() {
    let now = Instant::now();
    let instructions = read_input("testinput.txt");
    println!("total steps: {}", find_steps(&instructions));
    let elapsed_time = now.elapsed().as_micros();
    println!("Time taken for part1: {}", elapsed_time);


    let instructions2 = read_input("input.txt");
    println!("total steps for part 2: {}", find_all_steps(&instructions2));
    let elapsed_time = now.elapsed().as_micros();
    println!("Time taken for part2: {}", elapsed_time);
}
