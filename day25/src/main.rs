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
        let path = vec![self.values().next().unwrap().first().unwrap().a.clone()];
        let hm = HashMap::new();
        (n, path, hm)
    }

    fn dfs_bridge_find(&self, n: usize, trail: Vec<String>, neighbors: &HashMap<String, Vec<String>>) -> Option<Vec<Edge>> {
        let mut found_bridges = false;
        let mut neighbors = neighbors.clone();
        let next_edges: Vec<&Edge> = self.get(trail.last().unwrap()).unwrap().iter()
            .filter_map(|n| {
                if !trail.contains(&n.b) && !neighbors.contains_key(&n.b) {
                    neighbors.insert(n.b.clone(), vec![n.a.clone()]);
                    Some(n)
                } else {
                    None
                }
            })
        .collect();
        let neighbor_count = neighbors.values().map(|val| val.len()).sum::<usize>();
        //println!("n: {:?}, n_count: {neighbor_count}\n---trail: {trail:?}", neighbors.keys().collect::<Vec<&String>>());
        if neighbor_count > n {
            let res: Vec<Vec<Edge>> = next_edges.iter().filter_map(|ne| {
                if !found_bridges {
                    let mut next_neighbors = neighbors.clone();
                    let mut next_trail = trail.clone();
                    next_trail.push(ne.b.clone());
                    next_edges.iter().for_each(|nn| {
                        if nn != ne && !trail.contains(&nn.b) {
                            next_neighbors.entry(nn.b.clone())
                                .and_modify(|val| val.push(ne.a.clone()))
                                .or_insert(vec![ne.a.clone()]);
                        }
                    });
                    next_neighbors.remove(&ne.b);
                    let output = self.dfs_bridge_find(n, next_trail, &next_neighbors);
                    match output {
                        Some(_) => found_bridges = true,
                        None => {}
                    };
                    output
                } else {
                    None
                }
            })
            .collect();
            if res.is_empty() {
                None
            } else {
                Some(res[0].clone())
            }
        } else {
            println!("{}", neighbor_count);
            Some(neighbors.iter().flat_map(|(k, v)| {
                v.iter().map(|end| {
                    Edge {
                        a: k.to_string(),
                        b: end.to_string(),
                    }
                })
                .collect::<Vec<Edge>>()
            }).collect())
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
    println!("{:?}", dfs_parameters.1);
    let bridges = graph.dfs_bridge_find(dfs_parameters.0, dfs_parameters.1, dfs_parameters.2).unwrap();
    println!("{}", bridges.len());
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
