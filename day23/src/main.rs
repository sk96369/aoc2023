use std::{time::Instant, collections::BinaryHeap, ops::{Deref, DerefMut}, cmp::Ordering};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Node {
    Slope(Direction),
    Path,
    Forest,
}

struct TrailMap {
    map: Vec<Node>,
    xlen: usize,
}

impl From<(usize, usize)> for Direction {
    fn from((prev, next): (usize, usize)) -> Direction {
        use Direction::*;
        match prev < next {
            true => {
                if prev + 1 == next {
                    Right
                } else {
                    Down
                }
            },
            false => {
                if next + 1 == prev {
                    Left
                } else {
                    Up
                }
            },
            _ => panic!("Invalid direction"),
        }
    }
}

impl From<&str> for TrailMap {
    fn from(input: &str) -> TrailMap {
        use Node::*;
        use Direction::*;

        TrailMap {
            map: {
                input.lines().filter(|line| line != &"").flat_map(|line| {
                    line.chars().map(|ch| {
                        match ch {
                            '#' => Forest,
                            '.' => Path,
                            '>' => Slope(Right),
                            '<' => Slope(Left),
                            '^' => Slope(Up),
                            'v' => Slope(Down),
                            _ => panic!("Unknown map node"),
                        }
                    })
                    .collect::<Vec<_>>()
                })
            .collect()
            },
            xlen: input.lines().next().unwrap().len(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct NodeWeight {
    weight: usize,
    d: Option<Direction>,
}

impl PartialEq for NodeWeight {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight 
    }
}

impl PartialOrd for NodeWeight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.weight.cmp(&other.weight))
    }
}

impl Eq for NodeWeight {}

impl Ord for NodeWeight {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl TrailMap {
    fn bfs(&self, slippery: bool) -> Vec<BinaryHeap<NodeWeight>> {
        use Node::*;

        let mut all_costs: Vec<BinaryHeap<NodeWeight>> = vec![BinaryHeap::new();self.len()];
        let mut all_trails = vec![vec![(1)];self.len()];
        let mut iterations = 0;
        while let Some(trail) = all_trails.pop() {
            iterations += 1;
            //println!("{:?}", trail);
            let neighbors = self.get_available_paths(&trail.last().unwrap(), slippery);
            let mut neighbors: Vec<Vec<usize>> = neighbors.into_iter()
                .filter_map(|n| {
                    if !trail.contains(&n) {
                        let mut new_trail = trail.clone();
                        if let Slope(x) = self[n] {
                            if slippery || Direction::from((*trail.last().unwrap(), n)) == x {
                                //println!("testaaaaaa");
                                new_trail.push(n);
                                let mut add_new = true;
                                all_costs[n].retain(|val| {
                                    if val.d == Some(x) {
                                        if val.weight >= new_trail.len() {
                                            println!("at {:?} retained {:?} vs {:?}", (n % self.xlen, n / self.xlen), val, (new_trail.len(), x)); 
                                            add_new = false;
                                            true
                                        } else {
                                            println!("removed {:?} vs {:?}", val, (new_trail.len(), x)); 
                                            add_new = true;
                                            false
                                        }
                                    }
                                    else {
                                        true
                                    }
                                });
                                if add_new {
                                    all_costs[n].push(NodeWeight { weight: new_trail.len(), d: Some(x)});
                                    Some(new_trail)
                                } else {
                                    None
                                }
                            } else {
                                all_costs[n].push(NodeWeight { weight: new_trail.len(), d: Some(x) });
                                Some(new_trail)
                            }
                        } else {
                            all_costs[n].push(NodeWeight {weight: new_trail.len(), d:None});
                            new_trail.push(n);
                            Some(new_trail)
                        }
                    } else {
                        None
                    }
                }).collect();
            if !neighbors.is_empty() {
                all_trails.append(&mut neighbors);
            }
            if iterations % 1000 == 0 {
                println!("trails: {}", all_trails.len());
            }
        }
        all_costs
    }

    fn get_available_paths(&self, loc: &usize, slippery: bool) -> Vec<usize> {
        use Node::*;
        use Direction::*;

        let mut neighbors: Vec<usize> = vec![];
        match self[*loc] {
            Slope(x) => {
                if slippery {
                    match x {
                        Up => if *loc >= self.xlen { neighbors.push(loc - self.xlen); },
                        Right => if loc + 1 % self.xlen != 0 { neighbors.push(loc + 1); },
                        Down => if *loc < self.len() - self.xlen { neighbors.push(loc + self.xlen); },
                        Left => if loc % self.xlen != 0 { neighbors.push(loc - 1); },
                    }
                } else {
                    if *loc >= self.xlen { neighbors.push(loc - self.xlen); }
                    if loc + 1 % self.xlen != 0 { neighbors.push(loc + 1); }
                    if *loc < self.len() - self.xlen { neighbors.push(loc + self.xlen); }
                    if loc % self.xlen != 0 { neighbors.push(loc - 1); }
                }
            },
            Path => {
                if *loc >= self.xlen { neighbors.push(loc - self.xlen); }
                if loc + 1 % self.xlen != 0 { neighbors.push(loc + 1); }
                if *loc < self.len() - self.xlen { neighbors.push(loc + self.xlen); }
                if loc % self.xlen != 0 { neighbors.push(loc - 1); }
            },
            Forest => panic!("Current location is a forest"),
        }
        
        neighbors.into_iter().filter(|n| self[*n] != Forest).collect()
    }
}


impl Deref for TrailMap {
    type Target = Vec<Node>;
    fn deref(&self) -> &Self::Target {
        &self.map 
    }
}

impl DerefMut for TrailMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}


fn solve_1(input: &str) -> usize {
    let trailmap = TrailMap::from(input);
    let mut costs = trailmap.bfs(true);
    costs[trailmap.len() - 2].pop().unwrap().weight
}

fn solve_2(input: &str) -> usize {
    let trailmap = TrailMap::from(input);
    let mut costs = trailmap.bfs(false);
    costs[trailmap.len() - 2].pop().unwrap().weight
}

fn main() {
    let now = Instant::now();
    let input = include_str!("input.txt");
    let answer = solve_1(input);
    println!("Solution for part 1: {}", answer);
    println!("Time taken: {} seconds", now.elapsed().as_secs());

    let answer = solve_2(input);
    println!("Solution for part 2: {}", answer);
    println!("Time taken: {} seconds", now.elapsed().as_secs());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("testinput.txt");
        let answer = solve_1(input);
        assert_eq!(answer, 94);
    }

    #[test]
    fn test_2() {
        let input = include_str!("testinput.txt");
        let answer = solve_2(input);
        assert_eq!(answer, 154);
    }
}
