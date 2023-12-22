use std::{collections::HashMap, fs::File, io::BufReader, io::BufRead};

use regex::Regex;
use num::integer::lcm;

use crate::day::{Day, Answer};

struct NodeInfo {
    left: String,
    right: String,
    is_ghost_start: bool,
    is_ghost_end: bool,
}

struct Input {
    directions: Vec<char>,
    nodes: Vec<NodeInfo>,
    node_name_to_id: HashMap<String, usize>,
    node_map: Vec<(usize, usize)>,
}

pub struct Day8<'a> {
    input_filename: &'a str,
}

impl<'a> Day8<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { input_filename: filename }
    }

    fn read_input(&self, _part2: bool) -> Input {
        let directions_re = Regex::new("^([RL]+)$").unwrap();

        // "NFK = (LMH, RSS)" -> cap[1]:"NFK", cap[2]:"LMH", cap[3]:"RSS"
        let node_re = Regex::new("([A-Z0-9]{3}) = \\(([A-Z0-9]{3}), ([A-Z0-9]{3})\\)").unwrap();
        
        let infile = File::open(&self.input_filename).expect("Failed to open puzzle input.");
        let mut directions: Vec<char> = Vec::new();
        let mut nodes: Vec<NodeInfo> = Vec::new();
        let mut node_name_to_id: HashMap<String, usize> = HashMap::new();  

        let reader = BufReader::new(infile);
        for line in reader.lines() {
            match &line {
                Ok(line) => {
                    let dir_cap = directions_re.captures(&line);
                    match dir_cap {
                        Some(cap) => {
                            // get directions
                            directions = cap[1].chars().collect();
                        }
                        None => {}
                    }
        
                    let node_cap = node_re.captures(&line);
                    match node_cap {
                        Some(cap) => {
                            // Store a node
                            let name = cap[1].to_string();
                            let left = cap[2].to_string();
                            let right = cap[3].to_string();

                            let node_no = nodes.len();
                            let is_ghost_start = name.ends_with("A");
                            let is_ghost_end = name.ends_with("Z");
                            // println!("{node_no}: {name}, is_ghost_end {is_ghost_end}");

                            nodes.push(NodeInfo {left, right, is_ghost_start, is_ghost_end});
                            node_name_to_id.insert(name, node_no);
                        }
                        None => {}
                    }
                }
                Err(_) => {}
            }
        }

        let mut node_map: Vec<(usize, usize)> = Vec::new();
        for n in 0..nodes.len() {
            let left_no = *node_name_to_id.get(&nodes[n].left).unwrap();
            let right_no = *node_name_to_id.get(&nodes[n].right).unwrap();

            node_map.push((left_no, right_no));
        }

        Input {directions, nodes, node_name_to_id, node_map}
    }

    fn steps_to_zzz(&self, input: &Input) -> usize {
        let mut steps = 0;
        let mut dir_index = 0;
        let mut loc = input.node_name_to_id["AAA"];
        let zzz = input.node_name_to_id["ZZZ"];

        let mut loc_count: HashMap<(usize, usize), bool> = HashMap::new();

        while loc != zzz {
            if loc_count.contains_key(&(loc, dir_index)) {
                println!("Repeating!");
                panic!();
            }
            loc_count.insert((loc, dir_index), true);


            steps += 1;
            loc = match input.directions[dir_index] {
                'L' => {
                    input.node_map[loc].0
                }
                'R' => {
                    input.node_map[loc].1
                }
                _ => panic!(),
            };

            dir_index = (dir_index+1) % input.directions.len();
        }

        steps
    }

    // returns (period_start, period_len, endings_per_period, last_ending)
    fn periodicity(&self, input: &Input, start: usize) -> (usize, usize, usize, usize) {
        let mut steps = 0;
        let mut endings = 0;
        let mut last_ending = 0;
        let mut dir_index = 0;
        let mut loc = start;

        // (node_no, dir_index) -> (steps, endings)
        let mut history: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut period_len = 0;
        let mut period_start = 0;
        let mut prior_endings: usize;
        let mut endings_per_period = 0;


        let mut repeating = false;
        while !repeating {

            history.insert((loc, dir_index), (steps, endings));

            steps += 1;


            loc = match input.directions[dir_index] {
                'L' => {
                    input.node_map[loc].0
                }
                'R' => {
                    input.node_map[loc].1
                }
                _ => panic!(),
            };

            if input.nodes[loc].is_ghost_end {
                endings += 1;
                last_ending = loc;
            }

            dir_index = (dir_index+1) % input.directions.len();

            if history.contains_key(&(loc, dir_index)) {

                (period_start, prior_endings) = history[&(loc, dir_index)];
                period_len = steps - period_start;
                endings_per_period = endings - prior_endings;
                repeating = true;
            }
        }

        (period_start, period_len, endings_per_period, last_ending)
    }

    fn ghost_steps(&self, input: &Input) -> usize {
        // For each start state, find its periodicity parameters
        let mut to_satisfy: HashMap<usize, (usize, usize, usize)> = HashMap::new();
        for n in 0..input.nodes.len() {
            if input.nodes[n].is_ghost_start {
                let periodicity = self.periodicity(input, n);

                let initial = periodicity.0;
                let period = periodicity.1;
                let mod_info = (initial, period, 0);  // the repeat happens when modulus is zero.  That's just baked in to the input.

                // initial, divisor, modulus
                to_satisfy.insert(n, mod_info);
            }
        }

        // Initialize steps with minimum modulo value
        let mut steps = 0;
        let mut increment = 1;
        let mut to_remove: Vec<usize> = Vec::new();
        while &to_satisfy.len() > &0 {
            steps += increment;

            for (k, constraint) in &to_satisfy {
                // check if this is satisfied now.
                // println!("    {} % {} = {} ?= {}", steps, constraint.1, (steps % constraint.1), constraint.2);
                if (steps % constraint.1) == constraint.2 {
                    // It's satisfied!
                    // Update the step increment to keep it satisfied.

                    increment = lcm(increment, constraint.1);
                    
                    // Remove this constraint
                    to_remove.push(*k);
                }
            }

            // clean out satisfied constraints.
            for n in &to_remove {
                to_satisfy.remove(n);
            }
            to_remove.clear();
        }

        steps

        // Find the time when all the periods line up.
        // 1. step out by the max initial period.
    }
}

impl<'a> Day for Day8<'a> {
    fn part1(&self) -> Answer {
        let input = self.read_input(false);

        Answer::Numeric(self.steps_to_zzz(&input))
    }

    fn part2(&self) -> Answer {
        let input = self.read_input(false);

        Answer::Numeric(self.ghost_steps(&input))
    }
}

#[cfg(test)]
mod tests {

    use crate::{Day, Answer, Day8};

    #[test]
    fn test_input_ex1_p1() {        
        let d = Day8::new("examples/day8_example1.txt");
        let input = d.read_input(false);

        assert_eq!(input.directions.len(), 2);
        assert_eq!(input.nodes.len(), 7);
    }

    #[test]
    fn test_input_ex2_p1() {        
        let d = Day8::new("examples/day8_example2.txt");
        let input = d.read_input(false);

        assert_eq!(input.directions.len(), 3);
        assert_eq!(input.nodes.len(), 3);
    }

    #[test]
    fn test_input_ex1_steps() {        
        let d = Day8::new("examples/day8_example1.txt");
        let input = d.read_input(false);

        assert_eq!(d.steps_to_zzz(&input), 2);
        // assert_eq!(d.part1(), Answer::Numeric(2));
    }

    #[test]
    fn test_input_ex2_steps() {        
        let d = Day8::new("examples/day8_example2.txt");
        let input = d.read_input(false);

        assert_eq!(d.steps_to_zzz(&input), 6);
    }
    
    #[test]
    fn test_ex3_ghost_steps() {        
        let d = Day8::new("examples/day8_example3.txt");
        let input = d.read_input(false);

        assert_eq!(d.ghost_steps(&input), 6);
    }
        
    #[test]
    fn test_input_ghost_steps() {        
        let d = Day8::new("data_aoc2023/day8.txt");
        let input = d.read_input(false);

        assert_eq!(d.ghost_steps(&input), 9177460370549);
    }

    #[test]
    fn test_periodicity() {        
        let d = Day8::new("examples/day8_example3.txt");
        let input = d.read_input(false);

        for n in 0..input.nodes.len() {
            if input.nodes[n].is_ghost_start {
                // println!("Testing start: {n} {}", input.nodes[n].name);
                let results = d.periodicity(&input, n);
                match n {
                    0 => { 
                        // 11A, 
                        assert_eq!(results, (1, 2, 1, 2));
                    }
                    3 => { 
                        // 22A,
                        assert_eq!(results, (1, 6, 2, 6));
                    }
                    _ => {}
                }
            }
        }
    }

    #[test]
    fn test_periodicity_day8() {        
        let d = Day8::new("data_aoc2023/day8.txt");
        let input = d.read_input(false);

        for n in 0..input.nodes.len() {
            if input.nodes[n].is_ghost_start {
                let results = d.periodicity(&input, n);
                match n {
                    39 => { 
                        // SLA, 
                        assert_eq!(results, (4, 11653, 1, 161));
                    }
                    127 => { 
                        // AAA,
                        assert_eq!(results, (2, 19783, 1, 364));
                    }
                    251 => { 
                        // LVA, 
                        assert_eq!(results, (4, 19241, 1, 651));
                    }
                    309 => { 
                        // NPA, 
                        assert_eq!(results, (4, 16531, 1, 676));
                    }
                    321 => { 
                        // GDA, 
                        assert_eq!(results, (6, 12737, 1, 501));
                    }
                    531 => { 
                        // RCA, 
                        assert_eq!(results, (2, 14363, 1, 643));
                    }
                    _ => {}
                }
            }
        }
    }
}
