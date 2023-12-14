use std::{collections::HashSet, ops::ControlFlow};

fn find_mirrors(pattern: &str, xlen: &usize) -> Vec<usize> {
    //println!("testprint:\n{pattern}");
    let pattern_as_vec: Vec<Vec<char>> = pattern.lines()
        .map(|l| l.chars().collect()).collect();
    let ylen = pattern_as_vec.len();
    let mut mirrors = check_direction(&pattern_as_vec, xlen);
    let mut transposed_pattern_as_vec = vec![vec!['.';ylen];*xlen];
    pattern_as_vec.iter().enumerate().for_each(|(i, y)| {
        y.iter().enumerate().for_each(|(j, x)| {
            transposed_pattern_as_vec[j][i] = *x;
        });
    });
    mirrors.append(&mut check_direction(&transposed_pattern_as_vec, &ylen)
        .iter().map(|m| m * 100).collect());
    mirrors
}

fn check_direction(pattern: &Vec<Vec<char>>, xlen: &usize) -> Vec<usize> {
    let mut candidates: Vec<usize> = (1..*xlen).collect();
    pattern.iter().for_each(|line| {
        candidates = candidates.iter().filter_map(|c| {
            let res = (0..*c).rev().zip(*c..*xlen).try_for_each(|(a, b)| {
                if line[a] != line[b] {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            });
            if res == ControlFlow::Continue(()) {
                Some(*c)
            } else {
                None
            }
        })
        .collect();
    });
    candidates
}

fn find_smudged_mirrors(pattern: &str, xlen: &usize) -> Vec<((usize, usize), (usize, usize))> {
    let pattern_as_vec: Vec<Vec<char>> = pattern.lines()
        .map(|l| l.chars().collect()).collect();
    let ylen = pattern_as_vec.len();
    let mut smudges = find_smudges(&pattern_as_vec, xlen, 1);
    let mut transposed_pattern_as_vec = vec![vec!['.';ylen];*xlen];
    pattern_as_vec.iter().enumerate().for_each(|(i, y)| {
        y.iter().enumerate().for_each(|(j, x)| {
            transposed_pattern_as_vec[j][i] = *x;
        });
    });
    if smudges.len() < 1 {
        smudges.append(&mut find_smudges(&transposed_pattern_as_vec, &ylen, 1).iter()
            .map(|(left, right)| {
                ((left.1, left.0), (right.1, right.0))
            })
            .collect());
    }
    
    smudges
}

fn find_smudges(pattern: &Vec<Vec<char>>, xlen: &usize, smudge_count: usize) -> Vec<((usize, usize), (usize, usize))> {
    let mut candidates: Vec<(usize, Vec<(usize, usize)>)> = (1..*xlen).map(|x| (x, vec![])).collect();
    pattern.iter().enumerate().for_each(|(lidx, line)| {
        candidates = candidates.iter_mut().filter_map(|(c, s)| {
            let res = (0..*c).rev().zip(*c..*xlen).try_for_each(|(a, b)| {
                if line[a] != line[b] {
                    if s.len() < smudge_count {
                        s.push((b - *c, lidx));
                        ControlFlow::Continue(())
                    } else {
                        ControlFlow::Break(())
                    }
                } else {
                        ControlFlow::Continue(())
                }
            });
            if res == ControlFlow::Continue(()) {
                Some((*c, s.clone()))
            } else {
                None
            }
        })
        .collect();
    });
    candidates.into_iter().filter_map(|(mirror, loc)| {
        if loc.len() == smudge_count {
            let loc = loc.first().unwrap();
            let left = (loc.1, mirror - (1 + loc.0));
            let right = (loc.1, mirror + loc.0);
            Some((left, right))
        } else {
            None
        }
    }).collect()
}

fn clean_smudges(pattern: &str, smudges: &Vec<((usize, usize), (usize, usize))>) -> Vec<String> {
    let pattern_as_vec: Vec<Vec<char>> = pattern.lines()
        .map(|l| l.chars().collect()).collect();

    let mut a: Vec<String> = vec![];
    smudges.iter().for_each(|s| {
        println!("smudges: {:?}", s);
        a.push(pattern_as_vec.iter().enumerate().map(|(i, line)| {
            let mut newline = line.iter().enumerate().map(|(j, ch)| {
                if i == s.0.0 && j == s.0.1 {
                    if *ch == '#' {
                        '.'
                    } else {
                        '#'
                    }
                } else {
                    *ch
                }
            })
            .collect::<String>();
            newline += "\n";
            newline
        })
        .collect());
        println!("{}\n", a.iter().last().unwrap());

        a.push(pattern_as_vec.iter().enumerate().map(|(i, line)| {
            let mut newline = line.iter().enumerate().map(|(j, ch)| {
                if i == s.1.0 && j == s.1.1 {
                    if *ch == '#' {
                        '.'
                    } else {
                        '#'
                    }
                } else {
                    *ch
                }
            })
            .collect::<String>();
            newline += "\n";
            newline
        })
        .collect());
        println!("{}", a.iter().last().unwrap());
    });
    a
}


fn solve_1(input: &str) -> usize {
    let patterns:Vec<&str> = input.split_terminator("\r\n\r\n").collect();
    #[cfg(test)]
    println!("inputs: \n{:#?}", patterns);
    patterns.iter().enumerate().map(|(i, p)| {
        let xlen = p.lines().next().unwrap().len();
        let score = find_mirrors(p, &xlen).iter().sum::<usize>();
        p.lines().for_each(|l| println!("{l}"));
        println!("score #{i}: {score}");
        println!();
        score
    }).sum::<usize>()
}

fn solve_2(input: &str) -> usize {
    let patterns:Vec<&str> = input.split_terminator("\r\n\r\n").collect();
    patterns.iter().enumerate().map(|(_i, p)| {
        let xlen = p.lines().next().unwrap().len();
        let smudges = find_smudged_mirrors(p, &xlen);
        let original_scores = find_mirrors(p, &xlen);
        let cleaned_p = clean_smudges(p, &smudges);
        #[cfg(test)]
        original_scores.iter().for_each(|os| {
            println!("______________\noriginal:");
            visualize(p, os);
        });
        println!();
        let new_scores: Vec<usize> = cleaned_p.iter().map(|c| {
            find_mirrors(c, &xlen).iter().filter_map(|s| {
                if !original_scores.contains(s) {
                    #[cfg(test)]
                    {
                        println!("cleaned:");
                        visualize(c, s);
                        println!("score: {:?}\n", s);
                    }
                    Some(*s)
                } else {
                    None
                }
            })
            .sum::<usize>()
        })
        .collect();
        let mut filtered_score: HashSet<usize> = HashSet::new();
        new_scores.iter().for_each(|m| {
            filtered_score.insert(*m);
        });

        println!();
        filtered_score.iter().sum::<usize>()
    }).sum::<usize>()
}

fn visualize(pattern: &str, divisor: &usize) {
    let xlen = pattern.lines().next().unwrap().len();
    if *divisor >= 100 {
        pattern.lines().enumerate().for_each(|(i, l)| {
            if divisor / 100 == i {
                (0..xlen).for_each(|_| print!("-"));
                println!();
            }
            println!("{}", l);
        });
    } else {
        pattern.chars().enumerate().for_each(|(i, c)| {
            if *divisor == i % (xlen + 2) {
                print!("|{c}");
            } else {
                print!("{c}");
            }
        });
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = include_str!("testinput.txt");
        assert_eq!(solve_1(input), 405);
    }

    #[test]
    fn test_2() {
        let input = include_str!("testinput.txt");
        assert_eq!(solve_2(input), 400);
    }

    #[test]
    fn test_vertical() {
        let input = include_str!("verticaltest.txt");
        assert_eq!(solve_1(input), 5);
    }

    #[test]
    fn smudge_finding_test() {
        let input = ".####..#.#.#.##..\r\n........#..##....\r\n..##..#.....#..##\r\n......##.##.#####\r\n######.#.####....\r\n..##....#..##.#..\r\n.#..#..#####.#...\r\n..##...#..#...#.#\r\n#######.#....####\r\n".to_string();
        let smudges = find_smudges(&input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>(), &17, 1);
        assert_eq!(smudges.first().unwrap(), &((7, 15), (7, 16)));
    }
}

fn main() {
    let input = include_str!("input.txt");
    //println!("Solution 1: {}", solve_1(input));

    println!("Solution 2: {}", solve_2(input));

}
