use std::fs;

struct Schematic {
    map: Vec<Vec<char>>,
    xsize: i32,
    ysize: i32,
}

//(X, Y)
type Coordinate = (i32, i32);

trait MapLike {
    fn find_part_numbers(&self) -> Vec<usize>;
    fn find_symbols(&self) -> Vec<Coordinate>;
    fn find_gears(&self) -> Vec<Coordinate>;
    fn get_number(&self, xy: &Coordinate) -> (usize, Coordinate);
    fn find_gear_ratios(&self) -> Vec<usize>;
}

impl MapLike for Schematic {
    fn find_gears(&self) -> Vec<Coordinate> {
        let mut coordinates = vec![];
        self.map.iter().enumerate().for_each(|(y, v)| {
            v.iter().enumerate().for_each(|(x, &item)| {
                if item == '*' {
                    coordinates.push((x as i32, y as i32));
                }
            });
        });

        coordinates
    }

    fn find_gear_ratios(&self) -> Vec<usize> {
        let symbols = self.find_gears();
        let mut gears: Vec<usize> = vec![];
        symbols.iter().for_each(|c| {
            let mut neighbors = vec![];
            let mut gear_ratios: Vec<(usize, Coordinate)> = vec![];
            let x_neighbors = match c.0 {
                0 => vec![0, 1],
                x if x >= self.xsize => vec![-1, 0],
                _ => vec![-1, 0, 1],
            };
            let y_neighbors = match c.1 {
                0 => vec![0, 1],
                y if y >= self.ysize => vec![-1, 0],
                _ => vec![-1, 0, 1],
            };
            for y in &y_neighbors {
                for x in &x_neighbors {
                    neighbors.push((x, y));
                }
            }
            while let Some(next_neighbor) = neighbors.pop() {
                let y = c.1 + next_neighbor.1;
                let x = c.0 + next_neighbor.0;
                if self.map[y as usize][x as usize].is_numeric() {
                    let new_number = self.get_number(&(x, y));
                    if gear_ratios.is_empty() ||
                        gear_ratios.last().unwrap().1 != new_number.1 {
                            gear_ratios.push(new_number);
                        }
                }
            }
            if gear_ratios.len() == 2 {
                gears.push(gear_ratios[0].0 * gear_ratios[1].0);
            }
        });
        gears
    }

    fn find_part_numbers(&self) -> Vec<usize> {
        let symbols = self.find_symbols();
        let mut numbers: Vec<(usize, Coordinate)> = vec![];
        symbols.iter().for_each(|c| {
            let mut neighbors = vec![];
            let x_neighbors = match c.0 {
                0 => vec![0, 1],
                x if x >= self.xsize => vec![-1, 0],
                _ => vec![-1, 0, 1],
            };
            let y_neighbors = match c.1 {
                0 => vec![0, 1],
                y if y >= self.ysize => vec![-1, 0],
                _ => vec![-1, 0, 1],
            };
            for y in &y_neighbors {
                for x in &x_neighbors {
                    neighbors.push((x, y));
                }
            }
            while let Some(next_neighbor) = neighbors.pop() {
                let y = c.1 + next_neighbor.1;
                let x = c.0 + next_neighbor.0;
                if self.map[y as usize][x as usize].is_numeric() {
                    let new_number = self.get_number(&(x, y));
                    if numbers.is_empty() ||
                        numbers.last().unwrap().1 != new_number.1 {
                            numbers.push(new_number);
                        }
                }
            }
        });
        numbers.iter().map(|(pn, _)| *pn).collect()
    }

    fn find_symbols(&self) -> Vec<Coordinate> {
        let mut coordinates = vec![];
        self.map.iter().enumerate().for_each(|(y, v)| {
            v.iter().enumerate().for_each(|(x, &item)| {
                if !(item.is_numeric() || item == '.') {
                    coordinates.push((x as i32, y as i32));
                }
            });
        });

        coordinates
    }

    //Finds all the adjacent digits on the x-axis in the schematic and returns 
    //the part number they compose
    //ALSO RETURNS COORDINATE OF THE FIRST DIGIT TO IDENTIFY UNIQUE NUMBERS!!!
    fn get_number(&self, xy: &Coordinate) -> (usize, Coordinate) {
        let mut digit_picker = xy.0;
        let mut number_string = "".to_string();
        //Move the digit picker, aka the index of the x-coordinate to the most
        //significant digit (ie. leftmost) in the part number
        while digit_picker - 1 >= 0 && self.map[xy.1 as usize][digit_picker as usize - 1].is_numeric() {
            digit_picker -= 1;
        }

        let coordinate = (digit_picker, xy.1);
        number_string.push(self.map[xy.1 as usize][digit_picker as usize]);

        //Append all the digits to the part number
        while digit_picker + 1 < self.xsize && self.map[xy.1 as usize][digit_picker as usize + 1].is_numeric() {
            digit_picker += 1;
            number_string.push(self.map[xy.1 as usize][digit_picker as usize]);
        }
        (number_string.parse::<usize>().unwrap(), coordinate)
    }
}

fn read_input(input: &str) -> Schematic {
    let map: Vec<Vec<char>> = fs::read_to_string(input).expect("Could not open file {input}")
        .lines()
        .filter_map(|line| {
            if line != "" {
                Some(line.chars().collect::<Vec<char>>())
            } else {
                None
            }
        })
        .collect();
    Schematic {
        ysize: map.len() as i32,
        xsize: map[0].len() as i32,
        map: map,
    }
}


fn main() {
    let input = read_input("input.txt");
    let numbers = input.find_part_numbers();
    println!("Sum of all part numbers: {}", numbers.iter().sum::<usize>());

    let gear_ratios = input.find_gear_ratios();
    println!("Sum of all gear ratios: {}", gear_ratios.iter().sum::<usize>());
}
