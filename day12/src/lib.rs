pub mod pieces;

#[derive(Debug)]
pub struct Arrangement {
    pub springs: String,
    pub broken_lengths: Vec<usize>,
}

pub struct FoldedArrangement {
    pub springs: String,
    pub broken_lengths: Vec<usize>,
}

impl From<&str> for FoldedArrangement {
    fn from(line: &str) -> FoldedArrangement {
        let mut split = line.split_terminator(' ');
        FoldedArrangement {
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

impl From<FoldedArrangement> for Arrangement {
    fn from(other: FoldedArrangement) -> Arrangement {
        Arrangement {
            springs: other.springs,
            broken_lengths: other.broken_lengths,
        }
    }
}
