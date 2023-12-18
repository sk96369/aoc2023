use std::ops::{
    Deref,
    DerefMut
};

#[derive(Debug)]
pub struct Pieces {
    pieces: Vec<usize>,
}

impl From<Vec<usize>> for Pieces {
    fn from(p: Vec<usize>) -> Pieces {
        if !p.is_empty() {
            Pieces {
                pieces: p,
            }
        } else {
            Pieces {
                pieces: vec![],
            }
        }
    }
}

impl From<&Vec<usize>> for Pieces {
    fn from(p: &Vec<usize>) -> Pieces {
        if !p.is_empty() {
            Pieces {
                pieces: p.clone(),
            }
        } else {
            Pieces {
                pieces: vec![],
            }
        }
    }
}

impl From<&[usize]> for Pieces {
    fn from(p: &[usize]) -> Pieces {
        if !p.is_empty() {
            Pieces {
                pieces: p.to_owned(),
            }
        } else {
            Pieces {
                pieces: vec![],
            }
        }
    }
}

impl Deref for Pieces {
    type Target = Vec<usize>;
    fn deref(&self) -> &Self::Target {
        &self.pieces
    }
}

impl DerefMut for Pieces {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pieces
    }
}

pub fn find_all_permutations(seq: &[char], pieces: &[usize]) -> usize {
    let mut sum = 0;
    println!("{:?} - {:?}", seq, pieces);
    if pieces.len() > 0 && seq.len() < pieces.iter().sum::<usize>() + pieces.len() - 1 {
        println!("errrooooor");
        return 0;
    }
    if let Some(p) = pieces.first() {
        if let Some(start_pos) = seq.iter().position(|ch| ['?', '#'].contains(&ch)) {
            if seq.len() >= start_pos + p {
                #[cfg(test)]
                println!("{:?}", seq);

                let seq_slice = &seq[start_pos..start_pos+p];
                if seq_slice.iter().all(|ch| ['?', '#'].contains(&ch)) {
                    if pieces.len() == 1 || seq.len() > start_pos + p {
                        sum += find_all_permutations( if seq.len() <= start_pos + p + 1 {
                            &[]
                        } else {
                            &seq[start_pos + p + 1..]
                        },
                        &pieces[1..]);
                        if &seq[start_pos] != &'#' {
                            if let Some(next_start_pos) = &seq[1..].iter().position(|ch| ['?', '#'].contains(&ch)) {
                                sum += find_all_permutations(&seq[1 + *next_start_pos..], &pieces[..]);
                            } 
                        } else if seq.len() > 2 {
                            if let Some(next_start_pos) = &seq[2..].iter().position(|ch| ['?', '#'].contains(&ch)) {
                                sum += find_all_permutations(&seq[2 + *next_start_pos..], &pieces[..]);
                            }
                        }
                    }
                }
            }
        }
    } else {
        println!(" + 1! {:?} - {:?}", seq, pieces);
        sum += 1;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn recursion_test_one() {
        let seq = "???.###????.###????.###????.###????.###";
        let pieces = Pieces::from(vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]);
        assert_eq!(find_all_permutations(&seq.chars().collect::<Vec<char>>()[..], &pieces), 1);
    }

    #[test]
    fn recursion_test_lots() {
        let seq = "????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####.?";
        let pieces = Pieces::from(vec![1,6,5,1,6,5,1,6,5,1,6,5,1,6,5]);
        assert_eq!(find_all_permutations(&seq.chars().collect::<Vec<char>>()[..], &pieces), 2500);
    }
}
