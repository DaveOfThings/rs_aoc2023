use std::{collections::{HashMap, VecDeque}, fs::File, io::{BufRead, BufReader}};

use crate::day::{Day, Answer};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref LINE_RE: Regex = Regex::new("([%&]?)([a-z]+) -> (.*)").unwrap();
}

#[derive(Clone, Copy, PartialEq)]
enum ModType {
    Node,
    FlipFlop,
    Nand,
}

struct Input {
    lines: Vec<(ModType, String, Vec<String>)>,
}

impl Input {
    fn read(filename: &str) -> Input {
        let f = File::open(filename).unwrap();
        let reader = BufReader::new(f);
        let mut lines: Vec<(ModType, String, Vec<String>)> = Vec::new();

        for line in reader.lines() {
            if let Some(mod_desc) = Input::process_line(&line.unwrap()) {
                lines.push(mod_desc);
            }
        }

        Input { lines }
    }

    fn process_line(s: &str) -> Option<(ModType, String, Vec<String>)> {
        if let Some(caps) = LINE_RE.captures(s) {
            // Module type: caps[1]
            let mod_type = match &caps[1] {
                "%" => ModType::FlipFlop,
                "&" => ModType::Nand,
                _ => ModType::Node,
            };

            // Module name: caps[2]
            let mod_name = caps[2].to_string();

            // Module outputs: caps[3]
            let outputs: Vec<String> = caps[3].split(", ").map(|s| s.to_string()).collect();

            Some( (mod_type, mod_name, outputs) )
        }
        else {
            println!("Mismatch on line: '{s}'");
            None
        }
    }
}


#[derive(PartialEq)]
struct Module {
    // module type
    mod_type: ModType,

    // name
    name: String,

    // id
    id: usize,
    mask: u128,

    // inputs: name of input and last pulse polarity seen.
    inputs: HashMap<usize, bool>,

    // outputs: 
    outputs: Vec<usize>,

    // state
    state: bool,
}

impl Module {
    pub fn new(mod_type: ModType, name: &str, id: usize) -> Module {
        Module { 
            mod_type, 
            name: name.to_string(), 
            id, 
            mask: 1 << id, 
            inputs: HashMap::new(), 
            outputs: Vec::new(), 
            state: false 
        }
    }

    fn register_output(&mut self, other: usize) {
        self.outputs.push(other);
    }

    fn register_input(&mut self, other: usize) {
        // println!("Node {} gets input {}", self.name, other);
        self.inputs.insert(other, false);
    }
}

struct StateSet {
    watched: usize,
    active_state: bool,
    modules: Vec<usize>,
    observations: HashMap<u128, usize>,  // state -> step when first observed
    steps: usize,
    period: usize,
    last_active: usize,
    active_times: Vec<usize>,
}

impl StateSet {
    fn create(m: usize, active_state: bool, sim_modules: &Vec<Module>) -> StateSet {
        let mut visited: Vec<usize> = Vec::new();
        let mut to_visit: Vec<usize> = Vec::new();
        // TODO-DW : remove // let mut module_names: Vec<String> = Vec::new();
        let mut modules: Vec<usize> = Vec::new();

        to_visit.push(m);
        while to_visit.len() > 0 {
            // pop from to_visit
            let visiting_id = to_visit.pop().unwrap();
            let visiting = &sim_modules[visiting_id];

            // if not visited
            if !visited.contains(&visiting_id) {
                // println!("  Visiting {}", visiting.name);

                // push to visited
                visited.push(visiting_id);

                // if flipflop, add to modules
                if visiting.mod_type == ModType::FlipFlop {
                    modules.push(visiting_id);
                }

                // push all inputs to to_visit
                for input_id in visiting.inputs.keys() {
                    // println!("schedule visit to {}", input_name);
                    to_visit.push(*input_id);
                }
            }
            else {
                // println!("Already visited {}, skipping", visiting.name);
            }
        }

        let observations: HashMap<u128, usize> = HashMap::new();

        StateSet { watched: m, active_state, modules, observations, steps: 0, period: 0, last_active: 0, active_times: Vec::new() }
    }

    // the state of a state set.
    fn construct_state(&self, modules: &Vec<Module>) -> u128 {
        let mut state: u128 = 0;
        for module_id in &self.modules {
            let module = &modules[*module_id];
            if module.state {
                state |= module.mask;
            }
        }
        
        state
    }

    fn watch(&mut self, modules: &Vec<Module>) {
        if modules[self.watched].state == self.active_state {
            if self.steps != self.last_active {
                // println!("t={}, Seen {} active.", self.steps, modules[self.watched].name);
                self.active_times.push(self.steps);
                self.last_active = self.steps;
            }
        }
    }

    fn update(&mut self, modules: &Vec<Module>) {
        self.steps += 1;

        let state = self.construct_state(modules);
        if let std::collections::hash_map::Entry::Vacant(e) = self.observations.entry(state) {
            e.insert(self.steps);
        } else if self.period == 0 {
            // set period
            self.period = self.steps - self.observations[&state];
        }

 
    }

    fn found_period(&self) -> bool {
        self.period != 0
    }

    fn get_period(&self) -> usize {
        self.period
    }

    // TODO
}

struct Sim {
    module_name_to_id: HashMap<String, usize>,
    modules: Vec<Module>,     // states of nodes
    events: VecDeque<(usize, usize, bool)>,
    state_sets: Vec<StateSet>,

    low_pulses: usize,
    high_pulses: usize,
    t: usize,
}

impl Sim {
    fn new(input: &Input) -> Sim {
        let mut id = 0;
        let mut module_name_to_id: HashMap<String, usize> = HashMap::new();
        let mut modules: Vec<Module> = Vec::new();

        // Create button module
        let button = Module::new(ModType::Node, "button", id);
        modules.push(button);
        module_name_to_id.insert("button".to_string(), id);
        // println!("registered button");
        id += 1;

        // Create Modules from input
        for line in &input.lines {
            let name = &line.1;
            let module = Module::new(line.0, &name, id);
            modules.push(module);
            module_name_to_id.insert(name.to_string(), id);
            // println!("registered name {}", name);
            id += 1;
        }

        // Create modules that only appear as outputs.
        for line in &input.lines {
            for out_name in &line.2 {
                match module_name_to_id.get_mut(out_name) {
                    Some(_) => {
                        // output already registered, don't re-create it.
                    },
                    None => {
                        // output neeeds a node created.
                        let module = Module::new(ModType::Node, out_name, id);

                        modules.push(module);
                        module_name_to_id.insert(out_name.to_string(), id);
                        // println!("registered output name: {}", out_name);
                        id += 1;
                    }
                }
            }
        }

        // Connect inputs / outputs
        for line in &input.lines {
            let name = &line.1;

            for out_name in &line.2 {
                let node_id = module_name_to_id.get(name).unwrap();
                let out_id = module_name_to_id.get(out_name).unwrap();

                modules[*node_id].register_output(*out_id);
                modules[*out_id].register_input(*node_id);
            }
        }

        // Event is (from_id, to_id, state)
        let events: VecDeque<(usize, usize, bool)> = VecDeque::new(); // event queue

        let state_sets = Vec::new();

        // 
        let mut sim = Sim {
            module_name_to_id, 
            modules, 
            events, 
            state_sets, 
            low_pulses: 0, 
            high_pulses: 0,
            t: 0,
        };

        sim.setup_state_sets();

        sim
    }

    fn setup_state_sets(&mut self) {
        match self.module_name_to_id.get("rx") {
            Some(rx_id) => {
                let rx = &self.modules[*rx_id];
                // create the state sets for each of 'rx' inputs
                for (input_id, _) in rx.inputs.iter() {
                    let input = &self.modules[*input_id];

                    // create state sets for each second-level input to rx.  (expecting 4)
                    for (input_id2, _) in input.inputs.iter() {
                        // let input2 = self.modules[*input_id2];
                        
                        // println!("Create state set for {}", input_name2);
                        let state_set2 = StateSet::create(*input_id2, true, &self.modules);  // borked
                        self.state_sets.push(state_set2);
                    }
                }
            }
            None => { } // do nothing, return state_sets will be empty vector. 
        }
    }

    fn product(&self) -> usize {
        // println!("Low pulses: {}, High pulses: {}.", self.low_pulses, self.high_pulses);

        self.low_pulses * self.high_pulses
    }

    // simulate a button press
    fn button(&mut self) {
        // increment sim time
        self.t += 1;

        // push one event, a low signal to 'broadcaster'
        let button_id = self.module_name_to_id.get("button").unwrap();
        let broadcaster_id = self.module_name_to_id.get("broadcaster").unwrap();
        self.events.push_back((*button_id, *broadcaster_id, false));
    }

    fn sim(&mut self) {
         // run until the event queue is empty
        while !self.events.is_empty() {
            // pop an event
            let (from_id, to_id, in_pulse) = self.events.pop_front().unwrap();

            // count it
            if in_pulse {
                self.high_pulses += 1;
            }
            else {
                self.low_pulses += 1;
            }

            // Locate the node
            let node = &mut self.modules[to_id];
/*
            if node.name == "zf" && self.t == 3947 {
                if in_pulse {
                    println!("t={}, zf received high", self.t);
                }
                else {
                    println!("t={}, zf received low", self.t);
                }
            }
*/

            // Update the node and propagate the pulses
            let out_pulse = match node.mod_type {
                ModType::Node => {
                    // Propagate pulses to all outputs
                    node.state = in_pulse;
                    Some(node.state)
                }
                ModType::Nand => {

                    // Update this input
                    node.inputs.insert(from_id, in_pulse);

                    // Evaluate all inputs, set state false if all inputs true
                    let mut all_true = true;
                    for (_other_name, other_state) in &node.inputs {
                        if *other_state == false {
                            all_true = false;
                            break;
                        }
                    }

                    node.state = !all_true;
                    Some(node.state)
                }

                ModType::FlipFlop => {
                    if in_pulse {
                        // With a high pulse input, a flipflop does nothing
                        None
                    }
                    else {
                        // With a low pulse, switch state, send corresponding pulse
                        node.state = !node.state;
                        Some(node.state)
                    }
                }
            };

            // Propagate a pulse to each output
            if let Some(out_pulse) = out_pulse {
                for other_id in &node.outputs {
                    self.events.push_back((node.id, *other_id, out_pulse));
                }
            }

            // Let state sets observe their watched nodes
            for ss in self.state_sets.iter_mut() {
                ss.watch(&self.modules);
            }
        }
    }

    fn run(&mut self, buttons: usize) {
        for _ in 0..buttons {
            self.button();
            self.sim();
            self.t += 1;
        }
    }

    fn run_to_known_periods(&mut self) -> usize {
        let mut found_periods = false;
        while !found_periods {
            self.button();
            self.sim();

            found_periods = true;
          
            for ss in self.state_sets.iter_mut() {
                ss.update(&self.modules);
                found_periods = found_periods && ss.found_period();
            }
        }

        let mut overall_period: usize = 1;
        for ss in self.state_sets.iter_mut() {
            overall_period *= ss.get_period();
        }

        overall_period
    }

    /*
    // create a vector of state sets.  A StateSet represents a group of flipflop modules that
    // communicate with each other.
    fn get_state_sets(&self) -> Vec<StateSet> {

        let mut sets = Vec::new();

        // create the state sets for each of 'rx' inputs
        let rx_id = self.module_name_to_id.get("rx").unwrap();
        let rx = &self.modules[*rx_id];
        for (input_id, _) in rx.inputs.iter() {
            let input = &self.modules[*input_id];

            // create state sets for each second-level input to rx.
            for (input_id2, _) in input.inputs.iter() {
                
                let state_set2 = StateSet::create(*input_id2, true, &self.modules);
                sets.push(state_set2);
            }
        }

        sets
    }
    */

    /*
    fn run_to_rx(&mut self) -> usize {
        let state_sets = self.get_state_sets());

        // TODO
        0

    }
    */
}

pub struct Day20<'a> {
    input_filename: &'a str,
}

impl<'a> Day20<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { input_filename: filename }
    }
}

impl<'a> Day for Day20<'a> {
    fn part1(&self) -> Answer {
        let input = Input::read(self.input_filename);

        let mut sim = Sim::new(&input);

        sim.run(1000);

        Answer::Numeric(sim.product())
    }

    fn part2(&self) -> Answer {
        // (3733*3793*3947*4057 + 1) = 226732077152352, too large

        let input = Input::read(self.input_filename);

        let mut sim = Sim::new(&input);


        // run until all state sets have detected periodic behavior
        let long_period = sim.run_to_known_periods();

        // I've seen that each component of the period only activates ones per sub-cycle and
        // it activates on the last time step of the cycle.
        // So the product of all the sub-cycle periods is the point where the final signal
        // is activated.

        
        Answer::Numeric(long_period)   // TODO-DW
    }
}

#[cfg(test)]
mod test {
    use crate::{day::{Answer, Day}, day20::{Day20, Input, Sim, StateSet}};

    #[test]
    fn test_input1() {
        let input = Input::read("examples/day20_example1.txt");
        assert_eq!(input.lines.len(), 5);
    }

    #[test]
    fn test_input2() {
        let input = Input::read("examples/day20_example2.txt");
        assert_eq!(input.lines.len(), 5);
    }

    #[test]
    fn test_button1() {
        let input = Input::read("examples/day20_example1.txt");
        let mut sim = Sim::new(&input);

        sim.run(1);
        assert_eq!(sim.low_pulses, 8);
        assert_eq!(sim.high_pulses, 4);
        assert_eq!(sim.product(), 32);
    }

    #[test]
    fn test_sim1() {
        let input = Input::read("examples/day20_example1.txt");
        let mut sim = Sim::new(&input);

        sim.run(1000);
        assert_eq!(sim.low_pulses, 8000);
        assert_eq!(sim.high_pulses, 4000);
        assert_eq!(sim.product(), 32000000);
    }

    #[test]
    fn test_part1_ex1() {
        let d = Day20::new("examples/day20_example1.txt");
        assert_eq!(d.part1(), Answer::Numeric(32000000));
    }

    #[test]
    fn test_sim2() {
        let input = Input::read("examples/day20_example2.txt");
        let mut sim = Sim::new(&input);

        sim.run(1000);
        assert_eq!(sim.high_pulses, 2750);
        assert_eq!(sim.low_pulses, 4250);

        assert_eq!(sim.product(), 4250*2750);
    }

    #[test]
    fn test_part1_ex2() {
        let d = Day20::new("examples/day20_example2.txt");
        assert_eq!(d.part1(), Answer::Numeric(4250*2750));
    }

    #[test]
    fn test_state_set() {
        // TODO : read input, create the state set for 'rx'
        let input = Input::read("data_aoc2023/day20.txt");
        let sim = Sim::new(&input);
        // println!("modules: {:?}", sim.modules.keys());
        let rx = sim.module_name_to_id.get("rx").unwrap();

        let state_set = StateSet::create(*rx, false, &sim.modules);

        assert_eq!(state_set.modules.len(), 48); // TODO-DW : change 4 to real length.

        // TODO : create the state sets for each of 'rx' inputs
    }

    #[test]
    fn test_state_sets2() {
        // read input, get rx module
        let input = Input::read("data_aoc2023/day20.txt");
        let sim = Sim::new(&input);
        // println!("modules: {:?}", sim.modules.keys());
        let rx_id = sim.module_name_to_id.get("rx").unwrap();
        let rx = &sim.modules[*rx_id];

        // create the state sets for each of 'rx' inputs
        assert_eq!(rx.inputs.len(), 1);
        for (input_id, _) in rx.inputs.iter() {
            let input = &sim.modules[*input_id];

            // println!("Create state set for {}", input_name);
            let state_set = StateSet::create(*input_id, false, &sim.modules);
            assert_eq!(state_set.modules.len(), 48);

            // create state sets for each second-level input to rx.  (expecting 4)
            assert_eq!(input.inputs.len(), 4);
            for (input_id2, _) in input.inputs.iter() {
                
                // println!("Create state set for {}", input_name2);
                let state_set2 = StateSet::create(*input_id2, true, &sim.modules);
                assert_eq!(state_set2.modules.len(), 12);
            }
        }

    }

    /*
    #[test]
    fn test_get_state_sets() {
        let input = Input::read("data_aoc2023/day20.txt");
        let sim = Sim::new(&input);

        let state_sets = sim.get_state_sets();
        assert_eq!(state_sets.len(), 4);       
    }
    */

    #[test]
    fn test_run_to_known_periods() {
        let input = Input::read("data_aoc2023/day20.txt");
        let mut sim = Sim::new(&input);

        let overall_period = sim.run_to_known_periods();

        assert!(sim.t < 4096);    
        for ss in sim.state_sets.iter() {
            println!("Found period {}.", ss.period);
            println!("  active {} times", ss.active_times.len());
        }
        assert_eq!(overall_period, 226732077152351);
    }
    
    /*
    #[test]
    fn test_rx() {
        let input = Input::read("data_aoc2023/day20.txt");
        let mut sim = Sim::new(&input);

        let count = sim.run_to_rx();

        assert_eq!(count, 0);
    }
    */

    #[test]
    fn test_part2() {
        let d = Day20::new("data_aoc2023/day20.txt");

        let count = d.part2();

        assert_eq!(count, Answer::Numeric(226_732_077_152_351));
    }
}
