use core::fmt;
use std::{ops::{Deref, DerefMut}, collections::HashMap, time::Instant, fmt::Display};

struct Graph {
    nodes: HashMap<String, Vec<Edge>>,
}

impl Graph {
    fn traverse(&self, input: &str) -> Vec<String> {
        let mut edges: Vec<Edge> = self.get(input).unwrap().clone();
        let mut nodes = vec![input.to_string()];
        while let Some(e) = edges.pop() {
            if !nodes.contains(&e.b) {
                nodes.push(e.b.to_string());
                edges.append(&mut self.get(&e.b).unwrap().clone());
            }
        }
        #[cfg(test)]
        println!("{:?}", nodes);
        nodes
    }

    fn get_dfs_parameters(&self, n: usize) -> (usize, Vec<String>, HashMap<String, Vec<String>>) {
        let mut path = vec![self.values().next().unwrap().first().unwrap().a.clone()];
        #[cfg(test)]
        {
            path = vec!["hfx".to_string()];
        }
        let mut hm: HashMap<String, Vec<String>> = HashMap::new();
        self.get(path.last().unwrap()).unwrap().iter().for_each(|e| {
            hm.entry(e.b.clone())
                .and_modify(|v| v.push(e.a.clone()))
                .or_insert(vec![e.a.clone()]);
        });

        (n, path, hm)
    }

    fn dfs_bridge_find(&self, n: usize, trail: &Vec<String>, neighbors: &HashMap<String, Vec<String>>) -> Option<Vec<Edge>> {
        #[cfg(test)]
        {
            println!("{:?}", trail);
            println!("--{neighbors:?}\n");
        }
        let mut next_edges: Vec<&Edge> = self.get(trail.last().unwrap()).unwrap().iter()
            .filter_map(|n| {
                if !trail.contains(&n.b) && neighbors.contains_key(&n.b) {
                    Some(n)
                } else {
                    None
                }
            })
        .collect();

        let neighbors_count = neighbors.values().map(|v| v.len()).sum::<usize>();
        if neighbors_count <= n {
            #[cfg(test)]
            println!("no next edges");
            Some(neighbors.iter().flat_map(|(k, v)| {
                v.iter().map(|end| {
                    Edge {
                        a: k.to_string(),
                        b: end.to_string(),
                    }
                })
                .collect::<Vec<Edge>>()
            }).collect())
        } else {
            let mut output = None;
            while let Some(ne) = next_edges.pop() {
                if output == None {
                    let mut neighbors = neighbors.clone();
                    neighbors.remove(&ne.b);
                    let mut trail = trail.clone();
                    trail.push(ne.b.clone());
                    self.get(trail.last().unwrap()).unwrap().iter().for_each(|e| {
                        if !trail.contains(&e.b) {
                            neighbors.entry(e.b.clone())
                                .and_modify(|v| v.push(e.a.clone()))
                                .or_insert(vec![e.a.clone()]);
                        }
                    });
                    let bridge_finds = self.dfs_bridge_find(n, &trail, &neighbors);
                    
                    match bridge_finds {
                        Some(a) => output = Some(a),
                        None => {},
                    }
                }
            }
            output
        }
    }

    fn remove_connection(&mut self, edge: (&str, &str)) -> Result<(), &str> {
        if !self.contains_key(edge.0) || !self.contains_key(edge.1) {
            Result::Err("No such edge")
        } else {
            let n = self.get_mut(edge.0).unwrap();
            *n = n.iter().filter_map(|e| {
                if &e.b == edge.1 {
                    None
                } else {
                    Some(e.clone())
                }
            })
            .collect();
            let n = self.get_mut(edge.1).unwrap();
            *n = n.iter().filter_map(|e| {
                if &e.b == edge.0 {
                    None
                } else {
                    Some(e.clone())
                }
            })
            .collect();
            Result::Ok(())
        }
    }
}

impl Deref for Graph {
    type Target = HashMap<String, Vec<Edge>>;
    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}

impl DerefMut for Graph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.nodes
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Edge {
    a: String,
    b: String,
}

impl From<&str> for Graph {
    fn from(input: &str) -> Graph {
        let mut graph = Graph {
            nodes: HashMap::<String, Vec<Edge>>::new(),
        };
        input.lines().for_each(|line| {
            let line: Vec<&str> = line.split_terminator(": ").collect();
            let a = line[0].to_string();
            let connections: Vec<&str> = line[1].split_whitespace().collect();
            connections.into_iter().for_each(|c| {
                let edge = Edge {
                    a: a.to_string(),
                    b: c.to_string(),
                };
                let edge_2 = Edge {
                    a: c.to_string(),
                    b: a.to_string(),
                };
                graph.entry(a.clone()).and_modify(|val| val.push(edge.clone()))
                    .or_insert(vec![edge]);
                graph.entry(c.to_string()).and_modify(|val| val.push(edge_2.clone()))
                    .or_insert(vec![edge_2]);
            });
        });
        graph
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        self.iter().for_each(|(k, v)| {
            let mut a = k.to_string();
            a += "\n";
            v.iter().for_each(|b| {
                a += "-->";
                a += &b.b;
                a += "\n";
            });
            s += &a;
        });
                
        write!(f, "{s}")
    }
}

fn solve_1(input: &str) -> usize {
    let mut graph = Graph::from(input);
    //println!("{graph}");
    let dfs_parameters = graph.get_dfs_parameters(3);
    let bridges = graph.dfs_bridge_find(dfs_parameters.0, &dfs_parameters.1, &dfs_parameters.2).unwrap();
    println!("{}", bridges.len());
    println!("bridges:\n---{:?}", bridges);
    bridges.iter().for_each(|b| graph.remove_connection((&b.a, &b.b)).unwrap());
    graph.traverse(&bridges.last().unwrap().a).len() * graph.traverse(&bridges.last().unwrap().b).len()
}

fn main() {
    let now = Instant::now();
    let input = include_str!("input.txt");
    let answer = solve_1(input);
    println!("Part 1 answer: {}", answer);
    println!("Calculated in {} milliseconds", now.elapsed().as_millis());

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
    let input = include_str!("testinput.txt");
    let answer = solve_1(input);
    assert_eq!(answer, 54);
    }
}
