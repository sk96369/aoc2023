use std::{fs, ops::{Deref, DerefMut}};

struct Pipes {
    map: Vec<char>,
    xsize: usize,
    start: usize,
}

impl Deref for Pipes {
    type Target = Vec<char>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for Pipes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}


impl Pipes {
    fn new(input: String) -> Pipes {
        let mut start = 0;
        let mut pipe_map = Vec::new();
        let lines: Vec<&str> = input.lines().filter(|l| l != &"").collect();
        let xsize = lines[0].len();
        lines.iter().enumerate().for_each(|(i, l)| {
            l.chars().enumerate()
            .for_each(|(j, c)| {
                if c == 'S' {
                    start = i*xsize + j;
                }
                pipe_map.push(c);
            });
        });
        let pipes = Pipes {
            map: pipe_map,
            xsize: xsize,
            start: start,
        };
        pipes
    }

    fn find_farthest_point(&self) -> usize {
        let mut distance = 1;
        let _: Vec<usize> = self.get_neighbors(self.start).into_iter().take_while(|n| {
            //Assume it's a loop
            let mut is_loop = true;
            distance = 1;
            let mut previous = self.start;
            let mut current = *n;
            let mut neighbors = self.get_neighbors(current);
            if !self.get_neighbors(current).contains(&previous) {
                is_loop = false;
            }
            //Iterate until one of the connected locations is not connected 
            //to the previous one, meaning it's not a loop
            while is_loop {
                if !self.get_neighbors(current).contains(&previous) {
                    is_loop = false;
                } else {
                    let prev = current.clone();
                    current = neighbors.iter().filter_map(|n| {
                        if n != &previous {
                            Some(*n)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<usize>>()[0];
                    previous = prev;
                    neighbors = self.get_neighbors(current);
                    distance += 1;
                    if self.map[current] == 'S' {
                        break;
                    }
                }
            }
            !is_loop
        })
        .collect();
        distance / 2
    }

    fn find_enclosed(&self) -> usize {
        let mut loop_sections = Vec::new();
        let _: Vec<usize> = self.get_neighbors(self.start).into_iter().take_while(|n| {
            let mut new_loop_sections = vec![];
            //Assume it's a loop
            let mut is_loop = true;
            let mut previous = self.start;
            let mut current = *n;
            let mut neighbors = self.get_neighbors(current);
            if !self.get_neighbors(current).contains(&previous) {
                is_loop = false;
            }
            //Iterate until one of the connected locations is not connected 
            //to the previous one, meaning it's not a loop
            while is_loop {
                new_loop_sections.push(current);
                if !self.get_neighbors(current).contains(&previous) {
                    is_loop = false;
                } else {
                    let prev = current.clone();
                    current = neighbors.iter().filter_map(|n| {
                        if n != &previous {
                            Some(*n)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<usize>>()[0];
                    previous = prev;
                    neighbors = self.get_neighbors(current);
                    if self.map[current] == 'S' {
                        break;
                    }
                }
            }
            println!("{}", new_loop_sections.len());
            if is_loop {
                loop_sections = new_loop_sections;
            }
            !is_loop
        })
        .collect();
        loop_sections.push(self.start);

        

        //Traverse the loop and save all the left and right hand side non-loop 
        //neighbors along the way
        let mut previous = self.start;
        let mut left_side = Vec::new();
        let mut right_side = Vec::new();

        loop_sections.iter().for_each(|current| {
            let direction = get_direction(&previous, current);
                match (self[*current], direction) {
                    ('|', Direction::Up) => {
                        if !loop_sections.contains(&(current - 1)) {
                            left_side.push(current - 1);
                        }
                        if !loop_sections.contains(&(current + 1)) {
                            right_side.push(current + 1);
                        }
                    },
                    ('|', Direction::Down) => {
                        if !loop_sections.contains(&(current + 1)) {
                            left_side.push(current + 1);
                        }
                        if !loop_sections.contains(&(current - 1)) {
                            right_side.push(current - 1);
                        }
                    },
                    ('-', Direction::Left) => {
                        if current > &self.xsize {
                            if !loop_sections.contains(&(current - self.xsize)) {
                                right_side.push(current - self.xsize);
                            }
                        }
                        if current < &(self.len() - self.xsize) {
                            if !loop_sections.contains(&(current + self.xsize)) {
                                left_side.push(current + self.xsize);
                            }
                        }
                    },
                    ('-', Direction::Right) => {
                        if current > &self.xsize {
                            if !loop_sections.contains(&(current - self.xsize)) {
                                left_side.push(current - self.xsize);
                            }
                        }
                        if current < &(self.len() - self.xsize) {
                            if !loop_sections.contains(&(current + self.xsize)) {
                                right_side.push(current + self.xsize);
                            }
                        }
                    },
                    ('L', Direction::Left) => {
                        if !loop_sections.contains(&(current - 1)) {
                            left_side.push(current - 1);
                        }
                        if current < &(self.len() - self.xsize) {
                            if !loop_sections.contains(&(current + self.xsize)) {
                                left_side.push(current + self.xsize);
                            }
                        }
                    },
                    ('L', Direction::Down) => {
                        if !loop_sections.contains(&(current - 1)) {
                            right_side.push(current - 1);
                        }

                        if current < &(self.len() - self.xsize) {
                            if !loop_sections.contains(&(current + self.xsize)) {
                                right_side.push(current + self.xsize);
                            }
                        }
                    },
                    ('F', Direction::Up) => {
                        if !loop_sections.contains(&(current - 1)) {
                            left_side.push(current - 1);
                        }
                        if current > &self.xsize {
                            if !loop_sections.contains(&(current - self.xsize)) {
                                left_side.push(current - self.xsize);
                            }
                        }
                    },
                    ('F', Direction::Left) => {
                        if !loop_sections.contains(&(current - 1)) {
                            right_side.push(current - 1);
                        }
                        if current > &self.xsize {
                            if !loop_sections.contains(&(current - self.xsize)) {
                                right_side.push(current - self.xsize);
                            }
                        }
                    },
                    ('7', Direction::Up) => {
                        if !loop_sections.contains(&(current + 1)) {
                            right_side.push(current + 1);
                        }
                        if current > &self.xsize {
                            if !loop_sections.contains(&(current - self.xsize)) {
                                right_side.push(current - self.xsize);
                            }
                        }
                    },
                    ('7', Direction::Right) => {
                        if !loop_sections.contains(&(current + 1)) {
                            left_side.push(current + 1);
                        }
                        if current > &self.xsize {
                            if !loop_sections.contains(&(current - self.xsize)) {
                                left_side.push(current - self.xsize);
                            }
                        }
                    },
                    ('J', Direction::Right) => {
                        if !loop_sections.contains(&(current + 1)) {
                            right_side.push(current + 1);
                        }
                        if current < &(self.len() - self.xsize) {
                            if !loop_sections.contains(&(current + self.xsize)) {
                                right_side.push(current + self.xsize);
                            }
                        }
                    },
                    ('J', Direction::Down) => {
                        if !loop_sections.contains(&(current + 1)) {
                            left_side.push(current + 1);
                        }
                        if current < &(self.len() - self.xsize) {
                            if !loop_sections.contains(&(current + self.xsize)) {
                                left_side.push(current + self.xsize);
                            }
                        }
                    },
                    _ => {},
                }
                previous = *current;
        });

        let mut enclosure = Vec::new();

        left_side.iter().for_each(|l| {
            let mut unchecked_area: Vec<usize> = Vec::new();
            unchecked_area.push(*l);
            while !unchecked_area.is_empty() {
                let current = unchecked_area.pop().unwrap();
                if !enclosure.contains(&current) {
                    enclosure.push(current);
                }
                let new_neighbors = self.get_all_neighbors(&current);
                
                if new_neighbors.is_empty() {
                    enclosure.clear();
                    unchecked_area.clear();
                }
                new_neighbors.iter().for_each(|n| {
                    if !loop_sections.contains(n) &&
                        !enclosure.contains(n) {
                        enclosure.push(*n);
                        unchecked_area.push(*n);
                    }
                });
            }
        });
        if !enclosure.is_empty() {
        //    (0..self.len()).for_each(|i| {
        //        if i % self.xsize == 0 {
        //            println!();
        //        }
        //        if loop_sections.contains(&i) {
        //            print!("{}", self[i]);
        //        } else if enclosure.contains(&i) {
        //            print!("I");
        //       } else {
        //            print!(".");
        //        }
        //    });
        //    println!();
        //    enclosure.iter().for_each(|e| println!("{} {} : {e}", (e/self.xsize) + 1, (e % self.xsize) + 1));
            return enclosure.len();
        }

        right_side.iter().for_each(|l| {
            if !enclosure.contains(l) {
                enclosure.push(*l);
                let mut unchecked_area: Vec<usize> = Vec::new();
                unchecked_area.push(*l);
                while !unchecked_area.is_empty() {
                    let current = unchecked_area.pop().unwrap();
                    let new_neighbors = self.get_all_neighbors(&current);
                    if new_neighbors.is_empty() {
                        enclosure.clear();
                        unchecked_area.clear();
                    }
                    new_neighbors.iter().for_each(|n| {
                        if !right_side.contains(n) &&
                            !loop_sections.contains(n) &&
                                !enclosure.contains(&n) {
                            unchecked_area.push(*n);
                            enclosure.push(*n);
                        }
                    });
                }
            }
        });
        if !enclosure.is_empty() {
            enclosure.len()
        } else {
            0
        }
    }

    fn get_all_neighbors(&self, loc: &usize) -> Vec<usize> {
        if loc % self.xsize > 1 &&
            loc % self.xsize < self.xsize - 2 &&
                loc > &(self.xsize * 2) &&
                loc < &(self.len() - (self.xsize * 2)) {
                    vec![loc - 1, loc + 1, loc + self.xsize, loc - self.xsize]
        } else {
            vec![]
        }
    }

    fn get_neighbors(&self, loc: usize) -> Vec<usize> {
        match self.map[loc] {
            '|' => vec![loc - self.xsize, loc + self.xsize],
            '-' => vec![loc - 1, loc + 1],
            'L' => vec![loc - self.xsize, loc + 1],
            'J' => vec![loc - self.xsize, loc - 1],
            '7' => vec![loc + self.xsize, loc - 1],
            'F' => vec![loc + self.xsize, loc + 1],
            '.' => vec![],
            'S' => {
                if loc > self.xsize {
                    vec![loc - 1, loc + 1, loc - self.xsize, loc + self.xsize]
                    } else {
                        vec![loc - 1, loc + 1, loc + self.xsize]
                    }
            },
            _ => std::process::exit(1),
        }
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn get_direction(start: &usize, end: &usize) -> Direction {
    match *start as i64 - *end as i64 {
        d if d == -1 => Direction::Right,
        d if d == 1 => Direction::Left,
        d if d < -1 => Direction::Down,
        d if d > 1 => Direction::Up,
        _ => std::process::exit(1),
    }
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

fn main() {
    let input = read_input("input.txt");
    let pipes = Pipes::new(input);
    let farthest_distance = pipes.find_farthest_point();
    println!("Farthest point in the loop: {farthest_distance}");

    let enclosure_size = pipes.find_enclosed();
    println!("enclosure size: {}", enclosure_size);
}
