use std::collections::HashMap;

fn get_label(input: &str) -> u8 {
    let mut current = 0;
    input.chars().for_each(|c| {
        if !['\n', '\r'].contains(&c) {
            current += usize::from(c as u8);
            current *= 17;
            current %= 256;
        }
    });
       current as u8
}

fn solve_1(input: &str) -> usize {
    input.split_terminator(',').map(|s| {
        get_label(s) as usize
    })
    .sum::<usize>()
}

fn partial_position(a: &Vec<String>, b: &str) -> Option<usize> {
    let matches = a.iter().enumerate().filter_map(|(idx, s)| {
        if s.contains(b) {
            Some(idx)
        } else {
            None
        }
    })
    .collect::<Vec<usize>>();
    if matches.is_empty() {
        None
    } else {
        Some(matches[0].clone())
    }
}

fn solve_2(input: &str) -> usize {
    let input = input.trim_end();
    let mut boxes: HashMap<u8, Vec<String>> = HashMap::new();
    input.split_terminator(',').for_each(|s| {
        match s.contains('-') {
            true => {
                let sequence: Vec<&str> = s.split_terminator('-').collect();
                let boxnr = get_label(sequence[0]);
                if boxes.contains_key(&boxnr) {
                    let _ = boxes.entry(boxnr).and_modify(|v| {
                        #[cfg(test)]
                        println!("removed {} from {}", s, boxnr);
                        if let Some(idx) = partial_position(&v, sequence[0]) {
                            v.remove(idx);
                        }
                    });
                }
            },
            false => {
                let sequence: Vec<&str> = s.split_terminator('=').collect();
                let boxnr = get_label(sequence[0]);
                let new_entry = format!("{} {}", sequence[0], sequence[1]);
                boxes.entry(boxnr).and_modify(|v| {
                    #[cfg(test)]
                    println!("tried to push {} to {}", s, boxnr);
                    match partial_position(v, sequence[0]) {
                        Some(idx) => v[idx] = new_entry.clone(),
                        None => v.push(new_entry.clone()),
                    }
                })
                .or_insert( {
                    #[cfg(test)]
                    println!("created vec with {} to {}", s, boxnr);
                    vec![new_entry]
                });
            },
        }
        #[cfg(test)]
        println!("{boxes:#?}");
    });

    boxes.iter().map(|b| {
        b.1.iter().enumerate().map(|(sn,l)| {
            let lens: Vec<&str> = l.split_terminator(' ').collect();
            let a = 1 + usize::from(get_label(lens[0]));
            let b = sn + 1;
            let c = match lens[1].parse::<usize>() {
                Result::Ok(v) => v,
                Result::Err(_) => panic!("tried to parse {}: {:?}", lens[1], lens),
            };
            #[cfg(test)]
            println!("{}: {} * {} * {} = {}", l, a, b, c, a*b*c);
            a * b * c
        })
        .sum::<usize>()
    })
    .sum::<usize>()
}

fn main() {
    let input = include_str!("input.txt");
    let answer = solve_1(input);
    println!("{}", answer);

    let answer2 = solve_2(input);
    println!("{}", answer2);

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("testinput.txt");
        let answer = solve_1(input);
        assert_eq!(answer, 1320);
    }

    #[test]
    fn test_2() {
        let input = include_str!("testinput.txt");
        let answer = solve_2(input);
        assert_eq!(answer, 145);
    }
}
