use std::{cmp::{Reverse, Ordering}, collections::BinaryHeap, ops::{Deref, DerefMut}};

struct City {
    city: Vec<Node>,
    xlen: usize,
    goal: usize,
}

impl From<&str> for City {
    fn from(input: &str) -> City {
        let mut idx = 0;
        let xlen = input.lines().next().unwrap().len();
        let city = input.lines().filter(|line| !line.is_empty()).flat_map(|line| {
            line.chars().map(|ch| {
                let new_node = Node {
                    weight: usize::from(ch as u8 - '0' as u8),
                };
                idx += 1;
                new_node
            })
            .collect::<Vec<Node>>()
        })
        .collect();
        City {
            city: city,
            xlen: xlen,
            goal: idx - 1,
        }
    }
}

impl Direction {
    fn get_streak(&self) -> usize {
        use Direction::*;
        match self {
            Left(x) => usize::from(*x),
            Right(x) => usize::from(*x),
            Up(x) => usize::from(*x),
            Down(x) => usize::from(*x),
        }
    }
}

impl City {
    fn get_neighbors(&self, step: &Step) -> Vec<(usize, Direction)> {
        use Direction::*;
        let mut neighbors = vec![];

        match step.d {
            Up(_) | Down(_) => {
                if step.position % self.xlen != 0 {
                    neighbors.push((step.position - 1, Left(1)));
                }
                if (step.position + 1) % self.xlen != 0 {
                    neighbors.push((step.position + 1, Right(1)));               
                }
                if let Up(x) = step.d {
                    if x < 3 && step.position > self.xlen {
                        neighbors.push((step.position - self.xlen, Up(x + 1)));
                    }
                } else if let Down(x) = step.d {
                    if x < 3 && step.position < self.len() - self.xlen {
                        neighbors.push((step.position + self.xlen, Down(x + 1)));
                    }
                }
            },
            Left(_) | Right(_) => {
                if step.position > self.xlen {
                    neighbors.push((step.position - self.xlen, Up(1)));
                }
                if step.position < self.len() - self.xlen {
                    neighbors.push((step.position + self.xlen, Down(1)));               
                }
                if let Left(x) = step.d {
                    if x < 3 && step.position % self.xlen != 0 {
                        neighbors.push((step.position - 1, Left(x + 1)));
                    }
                } else if let Right(x) = step.d {
                    if x < 3 && (step.position + 1) % self.xlen != 0 {
                        neighbors.push((step.position + 1, Right(x + 1)));
                    }
                }
            },
        };
        #[cfg(test)]
        if step.position == 20 {
            println!("{:?}", step);
            println!("{:?}", neighbors);
        }
        neighbors
    }
}

impl Deref for City {
    type Target = Vec<Node>;
    fn deref(&self) -> &Self::Target {
        &self.city
    }
}

impl DerefMut for City {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.city
    }
}

struct Node {
    weight: usize,
}

#[derive(Debug)]
enum Direction {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
}

fn clumsy_star(city: &City, start: usize, goal: usize) -> usize {
    let mut dist: Vec<(_, usize)> = (0..city.len()).map(|_| (usize::MAX, usize::MAX)).collect();
    let mut paths = BinaryHeap::new();

    dist[start] = (0, 0);
    paths.push(Reverse(Step { cost: 0, position: start, d: Direction::Up(0), visited: vec![0] }));

    while let Some(step) = paths.pop() {
        let step = step.0;
        println!("{}: {}", step.position, step.cost);
        if step.position == goal {
            return step.cost;
        }

        city.get_neighbors(&step).into_iter().for_each(|next| {
            let next_cost = step.cost + city[next.0].weight;
            #[cfg(test)]
            println!("{:?} current dir: {:?}",step.visited, step.d);
            if !step.visited.contains(&next.0) && (dist[next.0].1 > next.1.get_streak() || dist[next.0].0 > next_cost) {
                if dist[next.0].0 > next_cost {
                    dist[next.0].0 = next_cost;
                    dist[next.0].1 = next.1.get_streak();
                }

                let mut new_step = Step {
                    d: next.1,
                    visited: step.visited.clone(),
                    cost: next_cost,
                    position: next.0,
                };
                new_step.visited.push(next.0);
                paths.push(Reverse(new_step));
            }
        });
        #[cfg(test)]
        println!("heap: {:?}\n", paths);
    }
    println!("asd");
    0
}

impl PartialOrd for Direction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Direction::*;
        match (self, other) {
            (&Up(s), &Up(t)) => Some(s.cmp(&t)),
            (&Down(s), &Down(t)) => Some(s.cmp(&t)),
            (&Left(s), &Left(t)) => Some(s.cmp(&t)),
            (&Right(s), &Right(t)) => Some(s.cmp(&t)),
            _ => None, 
        }
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        use Direction::*;
        match (self, other) {
            (&Up(s), &Up(t)) => s == t,
            (&Down(s), &Down(t)) => s == t,
            (&Left(s), &Left(t)) => s == t,
            (&Right(s), &Right(t)) => s == t,
            _ => false
        }
    }
}
     
impl Step {
    fn better_than(&self, other: &Self) -> bool {
        if self.cost <= other.cost {
            if self.d <= other.d {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
struct Step {
    d: Direction,
    cost: usize,
    position: usize,
    visited: Vec<usize>,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Step {

}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some (match (self.cost, other.cost) {
            (a, b) if a < b => Ordering::Less,
            (a, b) if a == b => Ordering::Equal,
            _ => Ordering::Greater,
        })
    }
}

fn main() {
    let input = include_str!("input.txt");
    let city = City::from(&input[..]);
    println!("Shortest path to goal: {}", clumsy_star(&city, 0, city.goal));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("testinput.txt");
        let city = City::from(&input[..]);
        assert_eq!(clumsy_star(&city, 0, city.goal), 102);
    }
}
