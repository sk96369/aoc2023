use std::{time::Instant, fs, cmp::{min, max}, ops::RangeFrom};

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
        // source numbers that aren't mapped correspond to the same destination
        // number, so we don't need to do anything here
    });
    if true {
        println!("{path:?}");
    }
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
        (a, a + b)
    })
    .collect()
}

//Turns the map into the format (start, end, difference) so my brain can 
//read it
fn get_range(map: (u64, u64, u64)) -> (u64, u64, i32) {
    (map.1, map.1 + map.2, map.2 - map.1)
}

fn get_next_layer(prev: &Vec<(u64, u64)>, category: &Vec<(u64, u64, u64)>) -> Vec<(u64, u64, u64)> {

}

fn get_candidates(seed_ranges: &Vec<(u64, u64)>, maps: &Maps) -> Vec<(u64, u64)> {
    let mut candidates = seed_ranges.clone();
    maps.iter().for_each(|m| {
        let mut new_candidates = candidates.clone();
        candidates.for_each(|c| {
            match compare_ranges((m.1, m.1 + m.2), (c.0, c.0 + c.1)) {
                Inside => new_candidates.push((c.0 + (m.1 - m.0)


    candidates
}

//Creates new ranges of seeds based on the overlap of the seed range and map
fn map_range(seeds: (u64, u64), map: (u64, u64, i32)) -> Option<Vec<(u64, u64)>> {
    if seeds.1 < map.0 || seeds.0 > map.1 {
        None
    }
    match (seeds.0 <= map.0, seeds.1 <= map.1) {
        (true, true) => {
            vec![(seeds.0, map.0 - 1), (map.0 + map.2, seeds.1 + map.2)]
        },
        (true, false) => {
            vec![(seeds.0, map.0 - 1),
            (map.0 + map.2, map.1 + map.2),
            (map.2 + 1, seeds.1)]
        },
        (false, true) => {
            vec![(seeds.0 + map.2, 
        },
        (false, false) => {

        }
}


fn get_lowest_candidates(seed_ranges: &Vec<(u64, u64)>, maps: &Maps) -> Vec<u64> {
    let mut candidates = vec![];
    maps.iter().for_each(|c| {
        c.iter().for_each(|m| {
            seed_ranges.iter().for_each(|s| {
                if m.1 >= s.0 && m.1 <= s.0 + s.1 {
                    candidates.push(m.1);
                }

                if m.0 >= s.0 && m.0 <= s.0 + s.1 {
                    candidates.push(m.0);
                }
            })
        })
    });
    let min_candidate = candidates.iter().min().unwrap().clone();
    seed_ranges.iter().for_each(|r| {
        if r.0 < min_candidate {
            (r.0..min(min_candidate, r.1)).for_each(|s| {
                candidates.push(s);
            });
        }
    });
    candidates
}

fn main() {
    let now = Instant::now();
    let (seeds, maps) = read_input("testinput.txt");
    println!("number of seeds: {}", seeds.len());
    let read_elapsed = now.elapsed();
    println!("Time taken to read data into maps: {read_elapsed:?}");
    
    let now = Instant::now();
    let min_location = seeds.iter().map(|s| {
        find_location(s, &maps)
    })
    .min()
    .unwrap();
    let part1_elapsed = read_elapsed + now.elapsed();
    println!("Total time taken to complete part 1: {:?}", part1_elapsed);
    println!("Lowest location number: {min_location}");

    let now = Instant::now();

    //part2 
    let seeds = get_seed_ranges(seeds).iter().map(|r| {
        get_range(r)
    })
    .collect()
    .sort_by_key(|r| r.0);

    let mut ranges = vec![];
    maps.iter().map(|c| {
        let mut next_layer = vec![];
        c.iter().for_each(|m| {
            next_layer.push(get_range(m));
        });
        ranges.push(next_layer.sort_by_key(|r| r.0));
    });


    
    let part2_elapsed = read_elapsed + now.elapsed();
    println!("Total time taken to complete part 1: {:?}", part2_elapsed);
    println!("Lowest location number: {min_location}");
}
