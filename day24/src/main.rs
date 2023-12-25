use std::time::Instant;

#[derive(Debug)]
struct Ray2D {
    x_direction: i8,
    slope: f64,
    offset: f64,
    start: (f64, f64, f64),
}

fn get_intersection(a: &Ray2D, b: &Ray2D) -> (f64, f64) {
    let x = (1_f64 / (b.slope - a.slope)) * (a.offset - b.offset);
    let y = a.slope * x + a.offset;
    (x, y)
}

fn check_intersection(a: &Ray2D, b: &Ray2D, area: (f64, f64)) -> (bool, (f64, f64)) {
    let intersection = get_intersection(a, b);
    let intersects = match (a.x_direction, b.x_direction) {
        (-1, -1) => intersection.0 <= a.start.0 && intersection.0 <= b.start.0,
        (-1, 1) => intersection.0 <= a.start.0 && intersection.0 >= b.start.0,
        (1, -1) => intersection.0 >= a.start.0 && intersection.0 <= b.start.0,
        (1, 1) => intersection.0 >= a.start.0 && intersection.0 >= b.start.0,
        _ => panic!("x_directions not set correctly!"),
    };
    let inside_area = area.0 <= intersection.0 &&
                    area.0 <= intersection.1 &&
                    area.1 >= intersection.0 &&
                    area.1 >= intersection.1;
    #[cfg(test)]
    {
        println!("{:?}", area);
        println!("{} {}", intersects, inside_area);
        println!("{:#?}", (intersects, intersection));
    }
    (intersects && inside_area, intersection)
}

impl From<&str> for Ray2D {
    fn from(input: &str) -> Ray2D {
        let a_b: Vec<_> = input.split_terminator(" @ ").collect();
        let point_a: Vec<f64> = a_b[0].split_terminator(", ").map(|c| c.trim().parse::<f64>().unwrap()).collect();
        let delta_point_a: Vec<f64> = a_b[1].split_terminator(", ").map(|c| c.trim().parse::<f64>().unwrap()).collect();
        let point_b: Vec<f64> = point_a.iter().zip(delta_point_a.iter()).map(|(a, b)| a + b).collect();
        
        let slope = (point_b[1] - point_a[1]) / (point_b[0] - point_a[0]);
        let offset = point_a[1] - (slope * point_a[0]);
        Ray2D {
            x_direction: match delta_point_a[0] > 0.0 { true => 1, _ => -1, },
            slope: slope,
            offset: offset,
            start: (point_a[0], point_a[1], point_a[2]),
        }
    }
}

fn solve_1(input: &str, area: (f64, f64)) -> usize {
    let hailstones: Vec<Ray2D> = input.lines().map(|line| {
        Ray2D::from(line)
    })
    .collect();
    let intersections = hailstones[0..hailstones.len() - 1].iter().enumerate().flat_map(|(idx, a)| {
        hailstones[(idx + 1)..].iter().map(|b| {
            check_intersection(a, b, area)
        })
        .collect::<Vec<(bool, (f64, f64))>>()
    })
    .collect::<Vec<(bool, (f64, f64))>>();
    intersections.iter().filter(|i| i.0 == true).count()
}

fn main() {
    let now = Instant::now();
    let input = include_str!("input.txt");
    let answer = solve_1(input, (200_000_000_000_000.0, 400_000_000_000_000.0));
    println!("Answer for part 1: {answer}");
    println!("Time taken: {} milliseconds", now.elapsed().as_millis());

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("testinput.txt");
        let answer = solve_1(input, (7.0, 27.0));
        println!("");
        assert_eq!(answer, 2);
    }
}

