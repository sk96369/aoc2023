

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum Node {
    Slope(Direction),
    Path,
    Forest,
}

fn solve_1(input: &str) -> usize {
    let trailmap = input.lines().filter_map(|line| {
        if line != "" {
            Some(
                line.chars().map(|ch| {
                    ch 
                )
        } else {
            None
        }
    })
    .collect();
}

fn main() {
    let input = include_str!("input.txt");
    let answer = solve_1(input);
    println!("Solution for part 1: {}"; answer);

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
}
