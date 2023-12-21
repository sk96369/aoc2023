use std::{collections::HashMap, hash::{Hash, Hasher}, rc::Rc};

#[derive(Debug)]
enum Pulse {
    Weak,
    Strong,
}

#[derive(Debug)]
struct Module {
    mtype: ModuleType,
    received: Vec<Pulse>,
    connections: Vec<Rc<Module>>,
}

#[derive(Debug)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction,
    Broadcaster,
    Button,
}

fn solve_1(input: &str) -> usize {
    let mut components: Vec<(String, (Rc<Module>, Vec<&str>))> = input.lines().filter_map(|l| { 
        if !l.is_empty() {
            let parts: Vec<&str> = l.split_terminator(" -> ").collect();
            match parts[0] {
                "broadcaster" => {
                    let name = parts[0].to_string();
                    let m = Module {
                        mtype: ModuleType::Broadcaster,
                        received: vec![],
                        connections: vec![],
                    };
                    Some((name, (Rc::new(m), parts[1].split_terminator(", ").collect())))
                },
                x => {
                    match x.chars().next().unwrap() {
                        '%' => {
                            let name: String = parts[0].chars().filter(|ch| ch != &'%')
                                .collect();
                            let m = Module {
                                mtype: ModuleType::FlipFlop(false),
                                received: vec![],
                                connections: vec![],
                            };
                            Some((name, (Rc::new(m), parts[1].split_terminator(", ").collect())))
                        },
                        '&' => {
                            let name: String = parts[0].chars().filter(|ch| ch != &'&')
                                .collect();
                            let m = Module {
                                mtype: ModuleType::Conjunction,
                                received: vec![],
                                connections: vec![],
                            };
                            Some((name, (Rc::new(m), parts[1].split_terminator(", ").collect())))
                        },
                        _ => panic!("rip"),
                    }
                },
            }
        } else {
            None
        }
    }).collect();
    let mut module_vec: Vec<(Rc<Module>, Vec<&str>)> = components.iter().map(|(_, (m, names))| {
        (Rc::clone(&m), names.clone())
    })
    .collect();
    let mut component_map: HashMap<String, Rc<Module>> = HashMap::new();
    components.into_iter().for_each(|c| {
        component_map.insert(c.0, Rc::clone(&c.1.0));
    });

    let bc = component_map.get_mut("broadcaster").unwrap();
    module_vec.iter_mut().for_each(|(m, names)| {
        names.iter().for_each(|name| {
            Rc::get_mut(m).unwrap().connections.push(Rc::clone(&component_map.get(&name[..]).unwrap()))
        });
    });

    let signal_counts = press_button(bc, 1000);
    signal_counts.0 * signal_counts.1   
}

impl Module {
    fn receive_signal(&mut self, pulse: Pulse) {
        self.received.push(pulse);
        match self.mtype {
            ModuleType::FlipFlop(&mut state) => {
                if pulse == Pulse::Weak {
                    state ^= true;
                }
            },
            _ => {}
        }
    }

    fn send_signal(&mut self) -> (usize, usize) {
        use ModuleType::*;
        let mut weak_count = 0;
        let mut strong_count = 0;
        match self.mtype {
            Broadcaster => {
                self.connections.iter().for_each(|c| {
                c.receive_signal(Pulse::Weak);
                });
                self.connections.iter().for_each(|c| {
                    let output = c.send_signal();
                    weak_count += output.0;
                    strong_count += output.1;
                });
            },
            FlipFlop(state) => {
                let pulse = match state {
                    true => Pulse::Strong,
                    false => Pulse::Weak,
                };
                self.connections.iter().for_each(|c| {
                    c.receive_signal(pulse);
                });
                self.connections.iter().for_each(|c| {
                    let output = c.send_signal();
                    weak_count += output.0;
                    strong_count += output.1;
                });
            },
            Conjunction => {
                if self.received.iter().all(|r| {
                    r == 
                self.connections.iter().for_each(|c| {
                    

        }
    }
}

fn press_button(bc: &mut Rc<Module>, button_presses: usize) -> (usize, usize) {
    let mut weak_count = 0;
    let mut strong_count = 0;
    (0..button_presses).for_each(|| {
        let output = bc.send_signal();
        weak_count += output.0;
        strong_count += output.1;
    });
    (weak_count, strong_count)
}


fn main() {
    let input = include_str!("testinput.txt");
    let answer = solve_1(input);
    println!("Part 1 answer: {}", answer);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("testinput.txt");
        let answer = solve_1(input);
        assert_eq!(answer, 32000000);
    }

    #[test]
    fn test_2() {
        let input = include_str!("testinput2.txt");
        let answer = solve_1(input);
        assert_eq!(answer, 11687500);
    }
}
