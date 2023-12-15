use std::{ops::{Deref, DerefMut}, fmt::{self, Display}};

#[derive(Debug)]
struct Platform {
    map: Vec<char>,
    xlen: usize,
    max_load: usize,
}

impl Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.map.iter().enumerate().map(|(idx, c)| {
            if idx >= self.xlen && idx % self.xlen == 0 {
                format!("\n{c}")
            } else {
                format!("{c}")
            }
        })
        .collect::<String>()
        )
    }
}

impl Deref for Platform {
    type Target = Vec<char>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for Platform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}


impl Platform {
    fn tilt_north(&mut self) {
        (self.xlen..self.map.len()).for_each(|idx| {
            let mut current = idx;
            while self.map[current] == 'O' && current > self.xlen &&
                self.map[current - self.xlen] == '.' {
                    self.map[current] = '.';
                    current = current - self.xlen;
                    self.map[current] = 'O';
                }
        });
    }

    fn rotate(&mut self) {
        (self.xlen..self.map.len()).for_each(|idx| {
            let mut current = idx;
            while self.map[current] == 'O' && current > self.xlen
                && self.map[current - self.xlen] == '.' {
                self.map[current] = '.';
                current = current - self.xlen;
                self.map[current] = 'O';
            }
        });
        println!("north:\n{self}");
        (1..self.map.len()).for_each(|idx| {
            let mut current = idx;
            while self.map[current] == 'O' && (current - 1) % self.xlen != 0
                && self.map[current - 1] == '.' {
                self.map[current] = '.';
                current = current - 1;
                self.map[current] = 'O';
            }
        });
        println!("west:\n{self}");
        ((0..self.map.len() - self.xlen).rev()).for_each(|idx| {
            let mut current = idx;
            while self.map[current] == 'O'
                && current + self.xlen < self.map.len()
                    && self.map[current + self.xlen] == '.' {
                self.map[current] = '.';
                current = current + self.xlen;
                self.map[current] = 'O';
            }
        });
        println!("south:\n{self}");
        (0..self.map.len() - 1).rev().for_each(|idx| {
            let mut current = idx;
            while self.map[current] == 'O' && (current + 1) % self.xlen != 0
                && self.map[current + 1] == '.' {
                self.map[current] = '.';
                current = current + 1;
                self.map[current] = 'O';
            }
        });
    }

    fn calculate_load(&self) -> usize {
        self.map.iter().enumerate().filter_map(|(idx, c)| {
            if *c == 'O' {
                Some(self.max_load - (idx / self.xlen))
            } else {
                None
            }
        }).sum::<usize>()
    }
}

impl From<&str> for Platform {
    fn from(input: &str) -> Platform {
        let lines = input.lines().count();
        Platform {
            xlen: input.lines().map(|l| l.len()).sum::<usize>() / lines,
            map: input.lines().flat_map(|l| l.chars()).collect(),
            max_load: lines,
        }
    }
}


fn solve_1(input: &str) -> usize {
    let mut p = Platform::from(input);
    //println!("{}", p);
    p.tilt_north();
    println!("tilted:\n{}", p);
    p.calculate_load()
}

fn solve_2(input: &str) -> usize {
    let cycles = 5;
    let mut p = Platform::from(input);
    //println!("{}", p);
    (0..cycles).for_each(|i| {
        p.rotate();
        p.calculate_load();
        println!("rotation: {i} load: {}", p.calculate_load());
        println!("east:\n{p}");
    });
    p.calculate_load()
}

fn main() {
    let input = include_str!("testinput.txt");
    let answer = solve_1(input);
    println!("part1: {answer}");
    println!("_______________________________________");

    let answer = solve_2(input);
    println!("part1: {answer}");

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("testinput.txt");
        let answer = solve_1(input);
        assert_eq!(136, answer);
    }

    #[test]
    fn test_2() {
        let input = include_str!("testinput.txt");
        let answer = solve_2(input);
        assert_eq!(64, answer);
    }
}

