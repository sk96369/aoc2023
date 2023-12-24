use std::{time::Instant, collections::HashMap, ops::{Deref, DerefMut}, cmp::Ordering};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    start: usize,
    weight: usize,
    node: usize,
}

#[derive(Debug)]
struct ForestGraph {
    adj_list: HashMap<usize, Vec<Edge>>,
    start: usize,
    goal: usize,
}

impl ForestGraph {
    fn find_longest_trail_len(&self) -> usize {
        let mut max_cost = 0;
        let mut max_cost_trail = vec![];
        let mut all_trails = vec![(vec![self.start], 0_usize)];
        while let Some(trail) = all_trails.pop() {
            if !(trail.0.last().unwrap() == &self.goal) {
                let mut neighbors = self.adj_list.get(trail.0.last().unwrap()).unwrap()
                    .iter()
                    .filter_map(|adj| {
                        if !trail.0.contains(&adj.node) {
                            let mut new_trail = trail.clone();
                            new_trail.1 += adj.weight;
                            new_trail.0.push(adj.node);
                            Some(new_trail)
                        } else {
                            None
                        }
                    })
                .collect();
                all_trails.append(&mut neighbors);
            } else {
                if trail.1 > max_cost {
                    max_cost = trail.1;
                    max_cost_trail = trail.0;
                }
            }
        }
        let max_cost_trail: Vec<(usize, usize)> = max_cost_trail.iter().map(|val| ((val / 23) + 1, (val % 23) + 1)).collect();
        println!("{:#?}", max_cost_trail);
        max_cost
    }
}

impl From<&TrailMap> for ForestGraph {
    fn from(input: &TrailMap) -> ForestGraph {
        let start = 1;
        let end = input.len() - 2;
        let mut checked_points = vec![start];
        let mut graphs_to_be_found = true;
        let mut fg = ForestGraph {
            adj_list: HashMap::new(),
            start: start,
            goal: end,
        };
        let mut all_trails = vec![vec![start]];
        let mut all_costs: HashMap<(usize, usize), usize> = HashMap::new();
        let mut _counter = 0;
        while graphs_to_be_found {
            let mut new_trails = vec![];
            while let Some(trail) = all_trails.pop() {
                _counter += 1;
                if _counter % 1000 == 0 {
                    println!("counter- fg.len(): {}", fg.adj_list.len());
                }
                //println!("{:?}", trail);
                let current = trail.last().unwrap();
                //println!("{:?}", trail);
                let neighbors = input.get_available_paths(&current, false);
                let mut neighbors: Vec<Vec<usize>> = neighbors.into_iter()
                    .filter_map(|n| {
                        if !trail.contains(&n) {
                            let mut new_trail = trail.clone();
                            new_trail.push(n);
                            if let Node::Slope(_) = input[n] {
                                if !checked_points.contains(&n) {
                                    checked_points.push(n);
                                    new_trails.push(vec![n]);
                                }

                                all_costs.entry((trail[0], *new_trail.last().unwrap()))
                                    .and_modify(|val| {
                                        if *val < trail.len() {
                                            *val = trail.len();
                                        }
                                    })
                                .or_insert( {
                                    trail.len()
                                });
                                None
                            } else if n == end {
                                all_costs.entry((trail[0], *new_trail.last().unwrap()))
                                    .and_modify(|val| {
                                        if *val < new_trail.len() {
                                            *val = new_trail.len();
                                        }
                                    })
                                .or_insert( {
                                    trail.len()
                                });
                                None
                            } else {
                                Some(new_trail)
                            }
                        } else {
                            None
                        }
                    }).collect();
                if !neighbors.is_empty() {
                    all_trails.append(&mut neighbors);
                }
            }
            if graphs_to_be_found {
                let mut added_new = false;
                all_costs.iter().for_each(|(k, v)| {
                    let new_edge = Edge {
                        start: k.0,
                        node: k.1,
                        weight: *v,
                    };

                    if new_edge.start != new_edge.node {
                        if let Some(val) = fg.adj_list.get_mut(&k.0) {
                            if !val.contains(&new_edge) {
                                val.push(new_edge.clone());
                                added_new = true;
                            }
                        } else {
                            added_new = true;
                            fg.adj_list.insert(k.0, vec![new_edge]);
                        }
                    }
                });
                if added_new {
                    graphs_to_be_found = true;
                } else {
                    graphs_to_be_found = false;
                }
                
                all_trails.append(&mut new_trails);
            }
        }
        fg
    }
}

impl TrailMap {
    fn bfs(&self, slippery: bool) -> HashMap<(String, usize), usize> {
        let now = Instant::now();
        use Node::*;

        let mut all_costs: HashMap<(String, usize), usize> = HashMap::new();
        //all_visited_nodes, slope_order
        let mut all_trails = vec![(vec![1], String::new())];
        let mut iterations = 0;
        while let Some(trail) = all_trails.pop() {
            iterations += 1;
            let current = trail.0.last().unwrap();
            //println!("{:?}", trail);
            let neighbors = self.get_available_paths(&current, slippery);
            let mut neighbors: Vec<(Vec<usize>, String)> = neighbors.into_iter()
                .filter_map(|n| {
                    if !trail.0.contains(&n) {
                        let mut new_trail = trail.clone();
                        new_trail.0.push(n);
                        if let Slope(x) = self[n] {
                            new_trail.1 += &n.to_string();
                            let current_dir = Direction::from((*trail.0.last().unwrap(), n));
                            if !slippery || current_dir == x {
                                //println!("is slippery: {}", slippery);
                                if let Some(old_cost) = all_costs.get_mut(&(new_trail.1.clone(), n)) {
                                    if *old_cost < new_trail.0.len() {
                                        *old_cost = new_trail.0.len();
                                        Some(new_trail)
                                    } else {
                                        None
                                    }
                                } else {
                                    all_costs.insert((new_trail.1.clone(), n), new_trail.0.len());
                                    Some(new_trail)
                                }
                            } else {
                                Some(new_trail)
                            }
                        } else if n == self.len() - 2 {
                            if let Some(old_cost) = all_costs.get_mut(&(String::new(), n)) {
                                if *old_cost < new_trail.0.len() {
                                    *old_cost = new_trail.0.len();
                                }
                            } else {
                                all_costs.insert((String::new(), n), new_trail.0.len());
                            }
                            None
                        } else {
                            Some(new_trail)
                        }
                    } else {
                        //println!("trail {:?} already contains {}", trail, n);
                        None
                    }
                }).collect();
            if !neighbors.is_empty() {
                all_trails.append(&mut neighbors);
            }
            if iterations % 1000 == 0 {
                println!("Time taken: {} milliseconds", now.elapsed().as_millis());
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
    let costs = trailmap.bfs(true);
    *costs.get(&(String::new(), trailmap.len() - 2)).unwrap()
}

fn solve_2(input: &str) -> usize {
    let trailmap = TrailMap::from(input);
    let fg = ForestGraph::from(&trailmap);
    fg.find_longest_trail_len()
}

fn main() {
    let input = include_str!("input.txt");
    let answer = solve_1(input);
    println!("Solution for part 1: {}", answer);

    let answer = solve_2(input);
    println!("Solution for part 2: {}", answer);
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
