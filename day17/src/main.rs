use std::{hash::{Hash, Hasher}, cmp::{Reverse, Ordering}, collections::{BinaryHeap, HashMap}, ops::{Deref, DerefMut}, time::Instant};

struct City {
    city: Vec<Node>,
    xlen: usize,
    goal: usize,
}

impl Clone for Direction {
    fn clone(&self) -> Direction {
        use Direction::*;
        match self {
            Left(x) => Left(*x),
            Up(x) => Up(*x),
            Right(x) => Right(*x),
            Down(x) => Down(*x),
        }
    }
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

impl Hash for Direction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use Direction::*;
        match self {
            Up(x) => (0, x),
            Right(x) => (1, x),
            Down(x) => (2, x),
            Left(x) => (3, x),
        }.hash(state);
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
    let now = Instant::now();
    //let mut dist: Vec<(_, usize)> = (0..city.len()).map(|_| (usize::MAX, usize::MAX)).collect();
    let mut dist: HashMap<(Direction, usize), usize> = HashMap::new();
    let mut paths = BinaryHeap::new();

    dist.insert((Direction::Up(1), 0), 0);
    dist.insert((Direction::Up(2), 0), 0);
    dist.insert((Direction::Up(3), 0), 0);
    dist.insert((Direction::Left(1), 0), 0);
    dist.insert((Direction::Left(2), 0), 0);
    dist.insert((Direction::Left(3), 0), 0);
    paths.push(Reverse(Step { cost: 0, position: start, d: Direction::Up(0), visited: vec![0] }));
    paths.push(Reverse(Step { cost: 0, position: start, d: Direction::Left(0), visited: vec![0] }));

    let mut lowest_cost = usize::MAX;
    let mut shortest_path = vec![];
    let mut max_pos = 0;
    while let Some(step) = paths.pop() {
        let step = step.0;
        #[cfg(test)]
        if step.position > max_pos {
            max_pos = step.position;
            println!("Farthest point: {}, Time elapsed: {}", max_pos, now.elapsed().as_secs());
        }
        if step.position == goal {
            println!("Time elapsed: {} ms", now.elapsed().as_millis());
            #[cfg(test)]
            step.visited.iter().for_each(|v| {
                let a = v % city.xlen;
                let b = v / city.xlen;
                println!("{a} {b}");
            });
            #[cfg(test)]
            {
                println!("dist:_____________________________________\\");
                let mut distvec = vec![usize::MAX;25];
                dist.iter().for_each(|d| {
                    if &distvec[d.0.1] > d.1 {
                        distvec[d.0.1] = *d.1;
                    }
                });
                distvec.iter().enumerate().for_each(|(idx, d)| {
                    println!("({} {}) = {}", idx % city.xlen, idx / city.xlen, d)
                });
            };
            return step.cost;
            lowest_cost = step.cost;
            shortest_path = step.visited.iter().map(|v| {
                (v % city.xlen, v / city.xlen)
            })
            .collect();
        }

        city.get_neighbors(&step).into_iter().for_each(|next| {
            let next_cost = step.cost + city[next.0].weight;
            if let Some(a) = dist.get_mut(&(next.1.clone(), next.0)) {   
                if *a > next_cost {
                    let mut new_step = Step {
                        d: next.1.clone(),
                        visited: step.visited.clone(),
                        cost: next_cost,
                        position: next.0,
                    };
                    new_step.visited.push(next.0);
                    paths.push(Reverse(new_step));
                    *a = next_cost;
                    dist.insert((next.1, next.0), next_cost);
                }
            } else {
                let mut new_step = Step {
                    d: next.1.clone(),
                    visited: step.visited.clone(),
                    cost: next_cost,
                    position: next.0,
                };
                new_step.visited.push(next.0);
                paths.push(Reverse(new_step));

                dist.insert((next.1, next.0), next_cost);
            }
        });
    }
    #[cfg(test)]
    println!("{:#?}: {}", shortest_path, lowest_cost);
    println!("Time elapsed: {} ms", now.elapsed().as_millis());
    lowest_cost
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

impl Eq for Direction {
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

    #[test]
    fn hashtest() {
        let input = "94128\n29182\n12345\n93885\n22841";
        let city = City::from(input);
        let answer = clumsy_star(&city, 0, city.goal);
        assert_eq!(1, 0);
    }

    #[test]
    fn longtest() {
        let input = include_str!("smallinput.txt");
        let city = City::from(&input[..]);
        let answer = clumsy_star(&city, 0, city.goal);
        println!("{}", answer);
    }
}
