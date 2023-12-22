use std::{cell::RefCell, collections::{HashMap, VecDeque, HashSet}, rc::{Rc, Weak}};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
struct Module {
    mtype: ModuleType,
    received_signals: VecDeque<Pulse>,
    senders: Vec<Weak<RefCell<Module>>>,
    receivers: Vec<Rc<RefCell<Module>>>,
    next_signal: Option<Pulse>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ModuleType {
    FlipFlop(Pulse),
    Conjunction,
    Broadcaster,
    Untyped,
    Observed,
}

fn solve_1(input: &str) -> usize {
    let mut components: Vec<(String, (Rc<RefCell<Module>>, Vec<&str>))> = input.lines().filter_map(|l| { 
        if !l.is_empty() {
            let parts: Vec<&str> = l.split_terminator(" -> ").collect();
            match parts[0] {
                "broadcaster" => {
                    let name = parts[0].to_string();
                    let m = Module {
                        mtype: ModuleType::Broadcaster,
                        senders: vec![],
                        received_signals: VecDeque::new(),
                        receivers: vec![],
                        next_signal: None,
                    };
                    Some((name, (Rc::new(RefCell::new(m)), parts[1].split_terminator(", ").collect())))
                },
                x => {
                    match x.chars().next().unwrap() {
                        '%' => {
                            let name: String = parts[0].chars().filter(|ch| ch != &'%')
                                .collect();
                            let m = Module {
                                mtype: ModuleType::FlipFlop(Pulse::Low),
                                senders: vec![],
                                received_signals: VecDeque::new(),
                                receivers: vec![],
                                next_signal: None,
                            };
                            Some((name, (Rc::new(RefCell::new(m)), parts[1].split_terminator(", ").collect())))
                        },
                        '&' => {
                            let name: String = parts[0].chars().filter(|ch| ch != &'&')
                                .collect();
                            let m = Module {
                                mtype: ModuleType::Conjunction,
                                senders: vec![],
                                received_signals: VecDeque::new(),
                                receivers: vec![],
                                next_signal: None,
                            };
                            Some((name, (Rc::new(RefCell::new(m)), parts[1].split_terminator(", ").collect())))
                        },
                        _ => panic!("rip"),
                    }
                },
            }
        } else {
            None
        }
    }).collect();
    let mut module_vec: Vec<(Rc<RefCell<Module>>, Vec<&str>)> = components.iter().map(|(_, (m, names))| {
        (Rc::clone(&m), names.clone())
    })
    .collect();
    let mut component_map: HashMap<String, Rc<RefCell<Module>>> = HashMap::new();
    components.into_iter().for_each(|c| {
        component_map.insert(c.0, Rc::clone(&c.1.0));
    });

    println!("component_map: {:#?}", component_map);
    module_vec.iter_mut().for_each(|(m, names)| {
        names.iter().for_each(|name| {
            let other = match component_map.get(&name[..]) {
                Some(o) => Rc::clone(o),
                None => Rc::new(RefCell::new(
                    Module {
                        next_signal: None,
                        senders: vec![],
                        received_signals: VecDeque::new(),
                        receivers: vec![],
                        mtype: ModuleType::Untyped,
                    })),
            };
            connect_components(&m, &other);
        });
    });

    let bc = component_map.remove("broadcaster").unwrap();
    let signal_counts = press_button(&bc, 1000);
    signal_counts.0 * signal_counts.1   
}

fn solve_2(input: &str) -> usize {
    let mut components: Vec<(String, (Rc<RefCell<Module>>, Vec<&str>))> = input.lines().filter_map(|l| { 
        if !l.is_empty() {
            let parts: Vec<&str> = l.split_terminator(" -> ").collect();
            match parts[0] {
                "broadcaster" => {
                    let name = parts[0].to_string();
                    let m = Module {
                        mtype: ModuleType::Broadcaster,
                        senders: vec![],
                        received_signals: VecDeque::new(),
                        receivers: vec![],
                        next_signal: None,
                    };
                    Some((name, (Rc::new(RefCell::new(m)), parts[1].split_terminator(", ").collect())))
                },
                x => {
                    match x.chars().next().unwrap() {
                        '%' => {
                            let name: String = parts[0].chars().filter(|ch| ch != &'%')
                                .collect();
                            let m = Module {
                                mtype: ModuleType::FlipFlop(Pulse::Low),
                                senders: vec![],
                                received_signals: VecDeque::new(),
                                receivers: vec![],
                                next_signal: None,
                            };
                            Some((name, (Rc::new(RefCell::new(m)), parts[1].split_terminator(", ").collect())))
                        },
                        '&' => {
                            let name: String = parts[0].chars().filter(|ch| ch != &'&')
                                .collect();
                            let m = Module {
                                mtype: ModuleType::Conjunction,
                                senders: vec![],
                                received_signals: VecDeque::new(),
                                receivers: vec![],
                                next_signal: None,
                            };
                            Some((name, (Rc::new(RefCell::new(m)), parts[1].split_terminator(", ").collect())))
                        },
                        _ => panic!("rip"),
                    }
                },
            }
        } else {
            None
        }
    }).collect();
    let mut module_vec: Vec<(Rc<RefCell<Module>>, Vec<&str>)> = components.iter().map(|(_, (m, names))| {
        (Rc::clone(&m), names.clone())
    })
    .collect();
    let mut component_map: HashMap<String, Rc<RefCell<Module>>> = HashMap::new();
    components.into_iter().for_each(|c| {
        component_map.insert(c.0, Rc::clone(&c.1.0));
    });

    let mut observed = None;
    println!("component_map: {:#?}", component_map);
    module_vec.iter_mut().for_each(|(m, names)| {
        names.iter().for_each(|name| {
            let other = match component_map.get(&name[..]) {
                Some(o) => Rc::clone(o),
                None => {
                    let untyped = Rc::new(RefCell::new(
                    Module {
                        next_signal: None,
                        senders: vec![],
                        received_signals: VecDeque::new(),
                        receivers: vec![],
                        mtype: ModuleType::Untyped,
                    }));
                    if name == &"rx" {
                        observed = Some(Rc::clone(&untyped));
                        observed.as_ref().unwrap().borrow_mut().mtype = ModuleType::Observed;
                    }
                    untyped
                },
            };
            connect_components(&m, &other);
        });
    });

    let bc = component_map.remove("broadcaster").unwrap();
    press_button_and_observe(bc, component_map)
}

fn connect_components(sender: &Rc<RefCell<Module>>, receiver: &Rc<RefCell<Module>>) {
    sender.borrow_mut().receivers.push(Rc::clone(&receiver));
    receiver.borrow_mut().senders.push(Rc::downgrade(&sender));
}

impl Module {
    fn process_signals(&mut self) {
        if let Some(pulse) = self.received_signals.pop_front() {
            match self.mtype {
                ModuleType::FlipFlop(_) => {
                    if pulse == Pulse::Low {
                        self.mtype = ModuleType::FlipFlop(Pulse::Low);
                        match self.next_signal {
                            Some(s) => match s {
                                Pulse::Low => {
                                    self.next_signal = Some(Pulse::High);
                                },
                                _ => {
                                    self.next_signal = Some(Pulse::Low);
                                },
                            },
                            None => {
                                self.next_signal = Some(Pulse::High);
                            },
                        }
                    } else {
                        self.mtype = ModuleType::FlipFlop(Pulse::High)
                    }
                },
                ModuleType::Conjunction => {
                    if self.senders.iter().all(|s| {
                        if let Some(val) = s.upgrade() {
                            if let Result::Ok(sender) = val.try_borrow() {
                                sender.next_signal.unwrap_or(Pulse::Low) == Pulse::High
                            } else { 
                                pulse == Pulse::High
                            }
                        } else { 
                            pulse == Pulse::High
                        }
                    }) {
                        self.next_signal = Some(Pulse::Low);
                    } else {
                        self.next_signal = Some(Pulse::High);
                    }
                },
                _ => {}
            }
        }
    }

    fn send_signal(&self) -> ((usize, usize), VecDeque<Rc<RefCell<Module>>>) {
        use ModuleType::*;
        let mut weak_count = 0;
        let mut strong_count = 0;
        let receivers: VecDeque<Rc<RefCell<Module>>> = match self.mtype {
            Broadcaster => {
                self.receivers.iter().for_each(|c| {
                    c.borrow_mut().received_signals.push_back(Pulse::Low);
                    if c.borrow().mtype == Conjunction {
                        c.borrow_mut().process_signals();
                    }
                });
                self.receivers.iter().map(|c| {
                    weak_count += 1;
                    Rc::clone(c)
                })
                .collect()
            },
            FlipFlop(state) => {
                match state {
                    Pulse::Low => {
                        self.receivers.iter().for_each(|c| {
                            //println!("self type: {:?} self next_signal: {:?}", c.borrow().mtype, c.borrow().next_signal);
                            c.borrow_mut().received_signals.push_back(self.next_signal.unwrap());
                        });
                        self.receivers.iter().map(|c| {
                            match self.next_signal.unwrap() {
                                Pulse::Low => weak_count += 1,
                                _ => strong_count += 1,
                            }
                            Rc::clone(c)
                        })
                        .collect()
                    },
                    _ => {
                        VecDeque::new()
                    },
                }
            },
            Conjunction => {
                self.receivers.iter().for_each(|r| {
                    r.borrow_mut().received_signals.push_back(self.next_signal.unwrap_or(Pulse::Low));
                });
                self.receivers.iter().map(|r| {
                    match self.next_signal.unwrap_or(Pulse::Low) {
                        Pulse::Low => weak_count += 1,
                        Pulse::High => strong_count += 1,
                    }
                    Rc::clone(r)
                })
                .collect()
            },
            Observed => {VecDeque::new()},
            Untyped => {VecDeque::new()},
        };
        #[cfg(test)]
        println!("{}", receivers.len());
        receivers.iter().for_each(|r| {
            //println!("{:?}", r.borrow().next_signal);
        });
        ((weak_count, strong_count), receivers) 
    }
}

fn press_button(bc: &Rc<RefCell<Module>>, button_presses: usize) -> (usize, usize) {
    let mut weak_count = 0;
    let mut strong_count = 0;
    (0..button_presses).for_each(|a| {
        weak_count += 1;
        bc.borrow_mut().next_signal = Some(Pulse::Low);
        let mut activated_modules = VecDeque::from([Rc::clone(bc)]);
        while let Some(m) = activated_modules.pop_front() {
            m.borrow_mut().process_signals();
            let m = m.borrow_mut();
            #[cfg(test)]
            println!("sending: {:?} from {:?} to {} modules", m.next_signal, m.mtype, m.receivers.len());
            let mut output = m.send_signal();
            weak_count += output.0.0;
            strong_count += output.0.1;
            activated_modules.append(&mut output.1);
        }
    });
    (weak_count, strong_count)
}

fn press_button_and_observe(bc: Rc<RefCell<Module>>, all_modules: HashMap<String, Rc<RefCell<Module>>>) -> usize {
    let mut combinations: HashSet<String> = HashSet::new();
    let mut loops: Vec<usize> = vec![];
    let mut presses = 0;
    let mut end = false;
    let mut last_nr = String::new();
    let mut last_nr_change = 0;
    let mut max_ones = 0;
    let mut conjunction_levels = vec![Rc::clone(all_modules.get("nr").unwrap())];
    let all_conjunctions: Vec<Rc<RefCell<Module>>> = all_modules.values().filter_map(|am| {
        if am.borrow().mtype == ModuleType::Conjunction {
            Some(Rc::clone(&am))
        } else {
            None
        }
    })
    .collect();

    let conjunction_dependents: Vec<(String, Rc<RefCell<Module>>, usize)> = all_modules.iter().filter_map(|am| {
        if am.1.borrow().receivers.iter().all(|s| {
            s.borrow().mtype == ModuleType::Conjunction
        }) {
            Some((am.0.clone(), Rc::clone(&am.1), am.1.borrow().receivers.len()))
        } else {
            None
        }
    })
    .collect();

    conjunction_dependents.iter().for_each(|cd| {
        println!("{}: {:?}\n\n", cd.0, (cd.1.borrow().mtype, cd.2))
    });

    std::process::exit(0);
    println!("{}", all_modules.len());
    while !end{
        presses += 1;
        bc.borrow_mut().next_signal = Some(Pulse::Low);
        let mut activated_modules = VecDeque::from([Rc::clone(&bc)]);
        while let Some(m) = activated_modules.pop_front() {
            let mut m = m.borrow_mut();
            if m.mtype == ModuleType::Observed &&
                !m.received_signals.iter().all(|s| s == &Pulse::High) {
                    end = true;
            }
            m.process_signals();
            #[cfg(test)]
            println!("sending: {:?} from {:?} to {} modules", m.next_signal, m.mtype, m.receivers.len());
            let mut output = m.send_signal();
            activated_modules.append(&mut output.1);
        }

        //let mut ones = 0;
        //#[cfg(test)]
        //let new_combination = all_modules.iter().map(|(am, _)| {
        //    if am.borrow().next_signal == Some(Pulse::High) {
        //        ones += 1;
        //        '1'
        //    } else {
        //        '0'
        //    }
        //})
        //.collect::<String>();

        let new_combination = all_modules.get("nr").unwrap().borrow().senders
            .iter().map(|s| {
                s.upgrade().unwrap().borrow().senders.iter().filter_map(|ss| {
                    if ss.upgrade().unwrap().borrow().mtype == ModuleType::Conjunction {
                        Some(ss.upgrade().unwrap().borrow().senders.iter().map(|sss| {
                            if sss.upgrade().unwrap().borrow().next_signal.unwrap_or(Pulse::Low) == Pulse::High {
                                '1'
                            } else {
                                '0'
                            }
                        }).collect::<String>() + " ")
                    } else {
                        None
                    }
                }).collect::<String>() + " "
            })
        .collect::<String>();

        #[cfg(test)]
        if new_combination != last_nr {
            println!("nr changed to {} at step {}", new_combination, last_nr_change);
            last_nr = new_combination.clone();
        }

        #[cfg(test)]
        let ac = all_conjunctions.iter().map(|ac| {
            ac.borrow().senders.iter().map(|s| {
                if s.upgrade().unwrap().borrow().next_signal.unwrap_or(Pulse::Low) == Pulse::High {
                    '1'
                } else {
                    '0'
                }
            })
            .collect::<String>()
        })
        .collect::<String>();

        //println!("{}", ac);
        //println!("{}", new_combination);

        let mut match_count = 0;
        new_combination.split_whitespace().enumerate().for_each(|(idx, nc)| {
            if nc.chars().all(|ch| ch == '0') {
                match_count += 1;
                #[cfg(test)]
                if idx == 0 {
                    loops.push(presses);
                    if loops.len() == 40 {
                        println!("{:#?}", loops);
                        end = true;
                    }
                }
                //#[cfg(test)]
                if match_count > 1 {
                    println!("{} conjunction #{} press#{}", new_combination, idx, presses);
                }
            }
        });
        //#[cfg(test)]
        if !combinations.contains(&new_combination) {
            #[cfg(test)]
            if ones > max_ones {
                println!("new max ones: {} at {} presses", ones, presses);
                max_ones = ones;
            }
            combinations.insert(new_combination);
            //println!("new combination at press #{}\n  combinations: {}", presses, combinations.len());
        } else {
            #[cfg(test)]
            println!("repeated combination at press #{} combinations: {}", presses, combinations.len());
        }
        #[cfg(test)]
        if presses - combinations.len() > last_nr_change {
            //println!("{} presses, {} unique combinations", presses, combinations.len());
            println!("{} repeats at {} presses", presses - combinations.len(), presses);
            last_nr_change = presses - combinations.len();
        }
    }
    presses
}

fn main() {
    let input = include_str!("input.txt");
    let answer = solve_1(input);
    println!("Part 1 answer: {}", answer);

    let answer = solve_2(input);
    println!("Part 2 answer: {}", answer);
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
