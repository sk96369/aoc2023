pub struct Arrangement {
    pub springs: String,
    pub broken_lengths: Vec<usize>,
}

pub struct FoldedArrangement {}

impl FoldedArrangement {
    fn from(line: &str) -> Arrangement {
        let mut split = line.split_terminator(' ');
        Arrangement {
            springs: {
                format!("{0}?{0}?{0}?{0}?{0}", split.next().unwrap())
            },
            broken_lengths: {
                let broken_lengths_as_str = format!("{0},{0},{0},{0},{0}", split.next().unwrap());
                broken_lengths_as_str.split_terminator(',').filter_map(|c| c.parse::<usize>().ok()).collect()
            },
        }
    }
}

enum Spring {
    Broken(usize),
    Unknown(usize),
}

fn get_permutations(shapes: &Vec<usize>, holes: &Vec<Spring>) -> Result<usize, ()> {
    if let Some(s) = shapes.first() {
        if let Some(h) = holes.first() {
            match h {
                Broken(l) => l <= h {
                    get_permutations(shapes[1..], vec![


impl Arrangement {
    fn count_fixes(&self) -> usize {
        let mut broken_iter = self.broken_lengths.iter();
        let mut current_pattern = broken_iter.next().unwrap();
        let mut broken = true;
        let mut min_sections = 0;
        let max_sections = self.broken_lengths.len();
        self.springs.chars().for_each(|c| {
            if ['#', '?'].contains(&c) {
                if !broken {
                    min_sections += 1;
                    broken = true;
                }
            } else if broken {
                broken = false;
            }
        });
        
        let mut current_broken = 0;

        let mut unknowns = vec![(vec![0], vec![0])];
        self.springs.chars().for_each(|c| {
            if c == '#' {
                if unknowns.last().unwrap().0.last().unwrap() == &0 {
                    current_broken += 1;
                } else {
                    *unknowns.last_mut().unwrap().0.last().unwrap() += 1;
                }
            } else if c == '?' {
                if current_broken > 0 {
                    if &current_broken < current_pattern {
                        current_broken += 1;
                    } else if &current_broken == current_pattern {
                        current_broken = 0;
                        current_pattern = broken_iter.next().unwrap();
                    } else {
                        //IF NO WORK TRY DIS:
                        //IF NO WORK BECAUSE THE PATTERNS DONT LINE UP,
                        //CHECK UNKNOWN'S TOP'S BOTH SIDES MATCHING AT THIS POINT
                        while &current_broken > current_pattern {
                            unknowns.last().unwrap().1.push(*current_pattern);
                            current_pattern = broken_iter.next().unwrap();
                        }
                    }
                } else {
                }
            } else {
                if unknowns.last().unwrap().0.last().unwrap() > &0 {
                    if current_broken > 0 {
                        if current_broken + unknowns.last().unwrap().0
                            .last().unwrap() == current_broken {
                            

                    unknowns.push((vec![], vec![]))
                }
                if current_broken > 0 {
                    if &current_broken == current_pattern {
                        current_pattern = broken_iter.next().unwrap();
                    } else 
                    current_broken = 0;
                    unknown = 0;
                    current_pattern = broken_iter.next().unwrap();
                }
            }
        });
    }
}

//Checks if the sections can fit the patterns
fn fits(sections: &Vec<usize>, patterns: &Vec<usize>) -> bool {
    if let Some(p) = patterns.last() {
        if let Some(s) = sections.last() {
            if p <= s {
                if 
            } else {
                fits(
        } else {
            false
        }
    } else {
        true
    }
}


    

impl From<&str> for Arrangement {
    fn from(line: &str) -> Arrangement {
        let mut split = line.split_terminator(' ');
        Arrangement {
            springs: split.next().unwrap().chars().collect(),
            broken_lengths: split.next().unwrap().split_terminator(',')
                .filter_map(|c| c.parse::<usize>().ok()).collect(),
        }
    }
}
