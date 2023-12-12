use day12::*;

fn solve_1(input: String) -> usize {
    let arrangements: Vec<Arrangement> = input.lines()
        .filter_map(|l| if l != "" {
            Some(Arrangement::from(l))
        } else {
            None
        })
    .collect();

    arrangements.iter().map(|a| {
        let springs_to_break = a.broken_lengths.iter().sum::<usize>() - 
            a.springs.chars().filter(|c| c == &'#').count();
        let unknown_indices: Vec<usize> = a.springs.chars().enumerate()
            .filter_map(|(i, c)| if c == '?' {
                Some(i)
            } else {
                None
            })
        .collect();
        #[cfg(test)]
        println!("{:?}", unknown_indices);
        (0..(2_u32.pow(unknown_indices.len() as u32)) as usize).filter(|p| {
            let mask = format!("{:b}", p);
            if mask.chars().filter(|c| c == &'1')
                .count() == springs_to_break {
                    #[cfg(test)]
                    println!("mask: {:?}", mask);
                    let mut mask_right = mask.chars().map(|c| {
                        c as u8 - '0' as u8
                    }).collect::<Vec<u8>>();
                    let mut mask = vec![
                        0_u8;
                        unknown_indices.len() - mask.len()
                    ];
                    mask.append(&mut mask_right);
                    #[cfg(test)]
                    println!("mask after padding: {:?}", mask);
                    let mut mask_position = 0;
                    let candidate = a.springs.chars().map(|c| {
                        if c == '?' {
                            if mask[mask_position] == 1 {
                                mask_position += 1;
                                '#'
                            } else {
                                mask_position += 1;
                                '.'
                            }
                        } else {
                            c
                        }
                    }).collect::<String>();
                    check_correctness(&candidate, &a.broken_lengths)
                } else {
                    false
                }
        }).count()
    }).sum::<usize>()
}

fn solve_2(input: String) -> usize {
    let arrangements: Vec<FoldedArrangement> = input.lines()
        .filter_map(|l| if l != "" {
            Some(FoldedArrangement::from(l))
        } else {
            None
        })
    .collect();

    arrangements.iter().map(|a| {
        let springs_to_break = a.broken_lengths.iter().sum::<usize>() - 
            a.springs.chars().filter(|c| c == &'#').count();
        let unknown_indices: Vec<usize> = a.springs.chars().enumerate()
            .filter_map(|(i, c)| if c == '?' {
                Some(i)
            } else {
                None
            })
        .collect();
        #[cfg(test)]
        println!("{:?}", unknown_indices);
        (0..(2_u32.pow(unknown_indices.len() as u32)) as usize).filter(|p| {
            let mask = format!("{:b}", p);
            if mask.chars().filter(|c| c == &'1')
                .count() == springs_to_break {
                    #[cfg(test)]
                    println!("mask: {:?}", mask);
                    let mut mask_right = mask.chars().map(|c| {
                        c as u8 - '0' as u8
                    }).collect::<Vec<u8>>();
                    let mut mask = vec![
                        0_u8;
                        unknown_indices.len() - mask.len()
                    ];
                    mask.append(&mut mask_right);
                    #[cfg(test)]
                    println!("mask after padding: {:?}", mask);
                    let mut mask_position = 0;
                    let candidate = a.springs.chars().map(|c| {
                        if c == '?' {
                            if mask[mask_position] == 1 {
                                mask_position += 1;
                                '#'
                            } else {
                                mask_position += 1;
                                '.'
                            }
                        } else {
                            c
                        }
                    }).collect::<String>();
                    check_correctness(&candidate, &a.broken_lengths)
                } else {
                    false
                }
        }).count()
    }).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_part1() {
        let input = include_str!("testinput.txt").to_owned();
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
