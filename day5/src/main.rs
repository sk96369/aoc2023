use std::{time::Instant, fs, cmp::min};

type Maps = Vec<Vec<Map>>;
type Map = (u64, u64, u64);

fn read_input(filename: &str) -> (Vec<u64>, Maps) {
    let file = fs::read_to_string(filename).unwrap();
    let mut file_iter = file.lines();
    let mut maps = vec![
        Vec::<Map>::new(),
        Vec::<Map>::new(),
        Vec::<Map>::new(), 
        Vec::<Map>::new(),
        Vec::<Map>::new(),
        Vec::<Map>::new(),
        Vec::<Map>::new(),
    ];

    let seeds = file_iter.next().unwrap().split_whitespace().filter_map(|x| {
        x.parse::<u64>().ok()
    })
    .collect();

    let mut i = 0;
    // Skip lines until we are at the start of the first line of mapping data
    // ie. first non-seed number
    file_iter.skip_while(|line| {
        line.is_empty() || !line.chars().next().unwrap().is_numeric()
    })
    .for_each(|line| {
        if !line.is_empty() {
            //Switch to the next set of maps if the line doesn't contain numbers
            if !line.chars().next().unwrap().is_numeric() {
                i += 1;
            } else if i < 7 {
                maps[i].push( {
                    let items: Vec<u64> = line.split_whitespace().filter_map(|x| {
                        x.parse::<u64>().ok()
                    }).collect();

                    if items.len() != 3 {
                        println!("A line in the input data is misformatted");
                        std::process::exit(1)
                    } else {
                        (items[0], items[1], items[2])
                    }
                });
            }
        }
    });
    //sort the maps by the start of the source range
    maps.iter_mut().for_each(|c| c.sort_by_key(|k| k.1));
    (seeds, maps)
}

fn find_location(seed: &u64, maps: &Maps) -> u64 {
    let mut val = *seed;
    let mut path = vec![val];
    maps.iter().for_each(|c| { // c = category
        let mut match_found = false;
        c.iter().for_each(|&m| { // m = map 
            if !match_found && val >= m.1 && val < m.1 + m.2 {
                val = m.0 + (val - m.1);
                match_found = true;
            }
        });
        path.push(val);
    });
    val
}

fn get_seed_ranges(seeds: &Vec<u64>) -> Vec<(u64, u64)> {
    let starts: Vec<u64> = seeds.iter().step_by(2).map(|x| x.to_owned())
        .collect();
    let lengths: Vec<u64> = seeds.iter().skip(1).step_by(2)
        .map(|x| x.to_owned()).collect();
    //I dont like the format of (start, length), so we'll convert the ranges 
    //to (start, start+length)
    starts.iter().zip(lengths.iter()).map(|(a, b)| {
        (a.to_owned(), (a + b).to_owned())
    })
    .collect()
}

//Turns the map into the format (start, end, difference) so my brain can 
//read it
fn get_range(map: (u64, u64, u64)) -> (u64, u64, i64) {
    (map.1, map.1 + map.2 - 1, map.0 as i64 - map.1 as i64)
}

fn get_next_layer(prev: &Vec<(u64, u64)>, category: &Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    let mut next_layer: Vec<(u64, u64)> = Vec::new();

    let category: Vec<(u64, u64, i64)> = category.iter().map(|c| get_range(*c)).collect();

    let mut _flag = true;
    prev.iter().for_each(|l| {
        let mut remaining_range = Some(*l);
        category.iter().for_each(|c| {
            if let Some(r) = remaining_range {
                if let Some(mapped_range) = map_range(&r, c) {
                    remaining_range = mapped_range.1;
                    mapped_range.0.iter().for_each(|a| {
                        next_layer.push(*a);
                    });
                };
            }
        });
        if let Some(r) = remaining_range {
            next_layer.push(r)
        }
    });
    next_layer.sort_by_key(|k| k.0);
    next_layer
}

//Creates new ranges of seeds based on the overlap of the seed range and map
//If the range of seeds and the map overlap, and the range of seeds exceeds 
//beyond the range of the map, return the exceeding positions separately
fn map_range(seeds: &(u64, u64), map: &(u64, u64, i64))
    -> Option<(Vec<(u64, u64)>, Option<(u64, u64)>)> {
    if seeds.1 < map.0 || seeds.0 > map.1 {
        return None;
    }
    // pray to the lord we don't get overflows here
    match (seeds.0 < map.0, seeds.1 > map.1) {
        (true, true) => {
            Some((vec![(seeds.0, map.0 - 1),
            (map.0.wrapping_add_signed(map.2), map.1.wrapping_add_signed(map.2))],
            Some((map.1 + 1, seeds.1))))
        },
        (true, false) => {
            Some((vec![(seeds.0, map.0 - 1), (map.0.wrapping_add_signed(map.2), seeds.1.wrapping_add_signed(map.2))], None))
        },
        (false, true) => {
            Some((vec![(seeds.0.wrapping_add_signed(map.2), map.1.wrapping_add_signed(map.2))], Some((map.1 + 1, seeds.1))))
        },
        (false, false) => {
            Some((vec![(seeds.0.wrapping_add_signed(map.2), seeds.1.wrapping_add_signed(map.2))], None))
        }
    }
}

fn main() {
    //Read input
    let now = Instant::now();
    let (seeds, maps) = read_input("input.txt");
    println!("number of seeds: {}", seeds.len());
    let read_elapsed = now.elapsed();
    println!("Time taken to read data into maps: {read_elapsed:?}");
    
    //part 1
    let now = Instant::now();
    let min_location = seeds.iter().map(|s| {
        find_location(s, &maps)
    })
    .min()
    .unwrap();
    let part1_elapsed = read_elapsed + now.elapsed();
    println!("Total time taken to complete part 1: {:?}", part1_elapsed);
    println!("Lowest location number: {min_location}\n");

    let now = Instant::now();

    //part2 
    let seeds = get_seed_ranges(&seeds);
    let mut count = 1;

    let mut layer = seeds;
    maps.iter().for_each(|c| {
        layer = get_next_layer(&layer, c);
        count += 1;
    });
    
    let part2_elapsed = read_elapsed + now.elapsed();
    println!("Total time taken to complete part 1: {:?}", part2_elapsed);
    println!("Lowest range: {:#?}", layer.iter().min().unwrap());
}
