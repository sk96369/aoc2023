use std::{sync::{Arc, Mutex}, thread, time::Duration};

use day12::{*, pieces};

fn solve_1(input: String) -> usize {
    let arrangements: Vec<Arrangement> = input.lines()
        .filter_map(|l| if l != "" {
            Some(Arrangement::from(l))
        } else {
            None
        })
    .collect();
    arrangements.iter().map(|a| {
        let answer = pieces::find_all_permutations(&a.springs.chars().collect::<Vec<char>>()[..], &a.broken_lengths[..]);
        answer
    }).sum::<usize>()
}

fn solve_2(input: String) -> usize {
    let arrangements: Vec<Arrangement> = input.lines().filter_map(|l| {
        if l != "" {
            Some(Arrangement::from(FoldedArrangement::from(l)))
        } else {
            None
        }
    })
    .collect();
    let shared_count = Arc::new(Mutex::new(0_usize));
    let shared_line_counter = Arc::new(Mutex::new(0_usize));
    let mut handles = vec![];

    arrangements.iter().enumerate().for_each(|(_idx, a)| {
        thread::sleep(Duration::from_secs(8));
        let shared_count = Arc::clone(&shared_count);
        let shared_line_counter = Arc::clone(&shared_line_counter);
        let pieces = a.broken_lengths.clone();
        let seq = a.springs.chars().collect::<Vec<char>>();
        let handle = thread::spawn(move || {
            //println!("{a:?}:");
            let answer = pieces::find_all_permutations(&seq[..], &pieces[..]);
            let mut lock = shared_count.lock().unwrap();
            *lock += answer;
            let mut lock = shared_line_counter.lock().unwrap();
            *lock += 1;
            println!("{}: {answer} ({_idx})", *lock);
        });
        handles.push(handle);
    });

    loop {
        if *shared_line_counter.lock().unwrap() == 1000 {
            let sum = *shared_count.lock().unwrap();
            println!("Done! Answer: {}", sum);
            return sum;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_part1() {
        let input = include_str!("testinput.txt").to_owned();
        #[cfg(test)]
        assert_eq!(solve_1(input), 21);
    }

    #[test]
    fn test_input_part2() {
        let input = include_str!("testinput.txt").to_owned();
        assert_eq!(solve_2(input), 525152);
    }
}

fn main() {
    let input = include_str!("input.txt").to_owned();
    println!("Solution for part 1: {}", solve_1(input));

    let input = include_str!("input.txt").to_owned();
    println!("Solution for part 2: {}", solve_2(input));

}
