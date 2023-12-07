use std::{collections::{BinaryHeap, HashMap}, fs, cmp::{Ordering, Reverse}};

#[derive(Eq, Debug)]
struct Hand {
    //cards: Vec<u64>,
    bid: u64,
    score: u64,
}

impl Hand {
    fn new(cards: Vec<u64>, bid: u64) -> Hand {
        Hand {
            //cards: cards,
            score: score_hand(&cards),
            bid: bid,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score 
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

fn read_input(filename: &str) -> Vec<Hand> {
    let char_to_card: HashMap<char, u64> = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14)
    ]);

    let mut ordered_hands = BinaryHeap::new();
    fs::read_to_string(filename).unwrap().lines().filter(|l| !l.is_empty())
        .for_each(|s| {
            let hand: Vec<&str> = s.split_whitespace().collect();
            ordered_hands.push(Hand::new(hand[0].chars().map(|c| {
                char_to_card.get(&c).unwrap().to_owned()
            }).collect(),
                hand[1].parse().unwrap()))
    });
    ordered_hands.into_sorted_vec()
}

//Returns a score, which is calculated by the poker rules, and a hash value
//appended to it
fn score_hand(cards: &Vec<u64>) -> u64 {
    let mut counts: HashMap<u64, usize> = HashMap::new();
    let mut score = 0;
    
    cards.iter().rev().enumerate().for_each(|(i, c)| {
        score += (100_u64.pow(i as u32)) * c;
        counts.entry(*c).and_modify(|sum| *sum += 1).or_insert(1);
    });

    score += 10_u64.pow(10) * match counts.len() {
        1 => 6,
        2 => {
            //arbitrarily chosen value
            let av = *counts.values().next().unwrap();
            if av == 1 || av == 4 {
                5
            } else {
                4
            }
        },
        3 => {
            let values: Vec<&usize> = counts.values().collect();
            if values[0] == &3 || values[1] == &3 || values[2] == &3 {
                3
            } else {
                2
            }
        },
        4 => 1,
        5 => 0,
        _ => std::process::exit(1),
    };
    score
}

fn calculate_scores(hands: Vec<Hand>) -> Vec<u64> {
    let mut rank = hands.len();
    hands.into_iter().rev().map(|h| {
        let score = h.bid * rank as u64;
        rank -= 1;
        score
    })
    .collect()
}


fn main() {
    let hands = read_input("input.txt");
    let scores = calculate_scores(hands);
    println!("Total score = {}", scores.iter().sum::<u64>());
}
