use std::{
    collections::HashSet,
    sync::{
        Mutex,
        Arc
    },
    ops::{DerefMut, Deref},
    thread::{
        self, JoinHandle
    }
};

enum Structure {
    Splitter(Position),
    Mirror(Angle),
}

fn reflect(d: &Direction, a: &Angle) -> Direction {
    if a == &Angle::NW {
        match d {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Right=> Direction::Down,
        }
    } else {
        match d {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Right=> Direction::Up,
        }
    }
}

impl Structure {
    fn get_beams(&self, d: &Direction) -> BeamSplit {
        match self {
            Structure::Splitter(p) => {
                if *p == Position::Vertical && [Direction::Up, Direction::Down].contains(d) {
                    BeamSplit::One(*d)
                } else {
                    BeamSplit::Two(Direction::Up, Direction::Down)
                }
            },
            Structure::Mirror(a) => {
                BeamSplit::One(reflect(d, a))
            },
        }
    }
}

#[derive(PartialEq)]
enum Position {
    Vertical,
    Horizontal,
}

#[derive(PartialEq)]
enum Angle {
    NE,
    NW,
}

struct Tile {
    structure: Option<Structure>,
    energized: bool,
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Beam {
    direction: Direction,
    loc: (usize, usize), // x, y
}

struct Grid {
    map: Vec<Vec<Tile>>,
    beams: Vec<Beam>,
}

impl Deref for Grid {
    type Target = Vec<Vec<Tile>>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl From<Vec<Vec<Tile>>> for Grid {
    fn from(v: Vec<Vec<Tile>>) -> Grid {
        Grid {
            map: v,
            beams: vec![Beam { 
                direction: Direction::Right,
                loc: (0, 0),
            }],
        }
    }
}

impl From<&char> for Tile {
    fn from(ch: &char) -> Tile {
        let s = match ch {
            '\\' => Some(Structure::Mirror(Angle::NW)),
            '/' => Some(Structure::Mirror(Angle::NE)),
            '|' => Some(Structure::Splitter(Position::Vertical)),
            '-' => Some(Structure::Splitter(Position::Horizontal)),
            _ => None,
        };
        Tile {
            structure: s,
            energized: false,
        }
    }
}

impl Grid {
    fn energize(&mut self, loc: &(usize, usize)) {
        self[loc.1][loc.0].energized = true;
    }

    fn move_beam(&self, beam: &Beam) -> Vec<Beam> {
        let next_loc = match beam.direction {
            Up => {
                if beam.loc.1 == 0 { None }
                else {
                    Some((beam.loc.0, beam.loc.1 - 1))
                }
            },
            Right => {
                if beam.loc.0 + 1 == self[0].len() { None }
                else {
                    Some((beam.loc.0 + 1, beam.loc.1))
                }
            },
            Down => {
                if beam.loc.1 + 1 >= self.len() { None }
                else {
                    Some((beam.loc.0, beam.loc.1 + 1))
                }
            },
            Left => {
                if beam.loc.0 == 0 { None }
                else {
                    Some((beam.loc.0 - 1, beam.loc.1))
                }
            },
        };
        if let Some(loc) = next_loc {
            let beam_split = {
                match self[loc.1][loc.0].structure {
                    Some(s) => s.get_beams(&beam.direction),
                    None => BeamSplit::One(beam.direction),
                }
            };
            match beam_split {
                BeamSplit::One(d) => vec![Beam {
                    direction: d,
                    loc: loc,
                }],
                BeamSplit::Two(a, b) => vec![Beam {
                        direction: a,
                        loc: loc,
                    },
                    Beam {
                        direction: b,
                        loc: loc,
                    }
                ],
            }
        } else {
            vec![]
        }
    }
}


enum BeamSplit {
    One(Direction),
    Two(Direction, Direction),
}

fn traverse_until_split(grid: Arc<Mutex<Grid>>, beam: Beam, log: Arc<Mutex<HashSet<(usize, usize)>>>) -> Vec<Beam> {
    let mut is_singular = true;
    let mut new_beam = beam;
    let mut new_beams = vec![];
    while is_singular {
        new_beams = grid.lock().unwrap().move_beam(&new_beam);
        let mut visit = log.lock().unwrap();
        new_beams.iter().for_each(|nb| {
            visit.insert(nb.loc);
        });
        if new_beams.len() != 1 {
            is_singular = false;
        } else {
            new_beam = new_beams[0];
        }
    }
    new_beams
}

fn solve_1(input: &str) -> usize {
    let grid = Arc::new(Mutex::new(Grid::from(input.lines().map(|line| {
        line.chars().map(|ch| {
            Tile::from(&ch)
        })
        .collect::<Vec<Tile>>()
    }).collect::<Vec<Vec<Tile>>>())));
    let beams: Arc<Vec<Beam>> = Arc::new(vec![Beam { loc: (0, 0), direction: Direction::Right}]);
    let visited: Arc<Mutex<HashSet<(usize, usize)>>> = Arc::new(Mutex::new(HashSet::new()));
    while !beams.is_empty() {
        let new_beams = Arc::clone(&beams);
        let handles: Vec<JoinHandle<Arc<Vec<Beam>>>> = new_beams.iter().map(|b| {
            let grid = Arc::clone(&grid);
            Arc::downgrade(&grid);
            let visited = Arc::clone(&visited);
            Arc::downgrade(&visited);
            let beam = b.clone();
            let thread_join_handle = thread::spawn(|| {
                traverse_until_split(grid, beam, &visited);
            });
            thread_join_handle
        })
        .collect();
        let new_beams: Vec<Beam> = handles.iter().flat_map(|h| {
            h.join().unwrap()
        })
        .collect();
    }
    visited.lock().unwrap().iter().count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("testinput.txt");
        let answer = solve_1(input);
        assert_eq!(answer, 46);
    }
}


fn main() {
    let input = include_str!("input.txt");
    let answer = solve_1(input);
    println!("part1 answer: {}", answer);
}
