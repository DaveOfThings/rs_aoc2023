use std::{collections::{HashMap, HashSet, VecDeque}, fs::File, io::{BufRead, BufReader}, rc::Rc};

use crate::day::{Day, Answer};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref LINE_RE: Regex = Regex::new("([%&]?)([a-z]+) -> (.*)").unwrap();
}

#[derive(Clone, Copy)]
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

struct Module {
    // module type
    mod_type: ModType,

    // name
    name: String,

    // inputs:
    inputs: HashMap<String, bool>,

    // outputs
    outputs: Vec<String>,

    // state
    state: bool,
}

impl Module {
    pub fn new(mod_type: ModType, name: &str) -> Module {
        Module { mod_type, name: name.to_string(), inputs: HashMap::new(), outputs: Vec::new(), state: false }
    }

    fn register_output(&mut self, other: &str) {
        self.outputs.push(other.to_string());
    }

    fn register_input(&mut self, other: &str) {
        self.inputs.insert(other.to_string(), false);
    }
}

struct StateSet {
    // TODO
}

impl StateSet {
    // TODO
}

struct StateSeq {
    // TODO
}

impl StateSeq {
    // TODO
}

struct Sim {
    modules: HashMap<String, Module>,     // states of nodes
    events: VecDeque<(String, String, bool)>,

    low_pulses: usize,
    high_pulses: usize,

    rx_activated: bool,
    rk_activated: bool,
    cd_activated: bool,
    zf_activated: bool,
    qx_activated: bool,
}

impl Sim {
    fn new(input: &Input) -> Sim {
        let mut modules: HashMap<String, Module> = HashMap::new(); // states of nodes

        // Create Modules from input
        for line in &input.lines {
            let name = &line.1;
            let module = Module::new(line.0, &name);
            modules.insert(name.to_string(), module);
        }

        // Connect inputs / outputs
        for line in &input.lines {
            let name = &line.1;

            for out_name in &line.2 {
                let node1 = modules.get_mut(name).unwrap();
                node1.register_output(out_name);
            }
        }

        // Connect inputs / outputs
        for line in &input.lines {
            let name = &line.1;

            for out_name in &line.2 {
                if let Some(node2) = modules.get_mut(out_name) {
                    node2.register_input(name);
                }
            }
        }

        // Event is (from_name, to_name, state)
        let events: VecDeque<(String, String, bool)> = VecDeque::new(); // event queue
        Sim {modules, events, low_pulses: 0, high_pulses: 0, 
            rx_activated: false, 
            rk_activated: false, cd_activated: false, zf_activated: false, qx_activated: false }
    }

    fn product(&self) -> usize {
        println!("Low pulses: {}, High pulses: {}.", self.low_pulses, self.high_pulses);

        self.low_pulses * self.high_pulses
    }

    // simulate a button press
    fn button(&mut self) {
        // push one event, a low signal to 'broadcaster'
        self.events.push_back( ("button".to_string(), "broadcaster".to_string(), false) );
    }

    fn sim(&mut self) {
         // run until the event queue is empty
        while !self.events.is_empty() {
            // pop an event
            let (from_name, to_name, in_pulse) = self.events.pop_front().unwrap();

            if (to_name == "rx") && (in_pulse == false) {
                self.rx_activated = true;
            }

            if (to_name == "rk") && (in_pulse == false) {
                self.rk_activated = true;
            }
            if (to_name == "cd") && (in_pulse == false) {
                self.cd_activated = true;
            }
            if (to_name == "zf") && (in_pulse == false) {
                self.zf_activated = true;
            }
            if (to_name == "qx") && (in_pulse == false) {
                self.qx_activated = true;
            }


            // println!("{from_name} -{in_pulse}-> {to_name}");

            // count it
            if in_pulse {
                self.high_pulses += 1;
            }
            else {
                self.low_pulses += 1;
            }

            let mut final_state = false;

            // Locate the node
            if let Some(node) = self.modules.get_mut(&to_name) {
                final_state = node.state;

                // Update the node and propagate the pulses
                let out_pulse = match node.mod_type {
                    ModType::Node => {
                        // Propagate pulses to all outputs
                        final_state = in_pulse;
                        // println!("  Propagating {final_state}");
                        Some(final_state)
                    }
                    ModType::Nand => {

                        // TODO: Fix again.  Initially all inputs should be remembered as low.

                        // Update this input
                        node.inputs.insert(from_name, in_pulse);

                        // Evaluate all inputs, set state false if all inputs true
                        let mut all_true = true;
                        for (_other_name, other_state) in &node.inputs {
                            if *other_state == false {
                                all_true = false;
                                break;
                            }
                        }

                        final_state = !all_true;
                        // println!("  NAND evaluated to {final_state}");
                        Some(final_state)
                    }

                    ModType::FlipFlop => {
                        if in_pulse {
                            // With a high pulse input, a flipflop does nothing
                            // println!("  Flip Flop ignoring high pulse.");
                            None
                        }
                        else {
                            // With a low pulse, switch state, send corresponding pulse
                            final_state = !node.state;
                            // println!("  Flip Flop changed to {final_state}");
                            Some(final_state)
                        }
                    }
                };

                // Propagate a pulse to each output
                if let Some(out_pulse) = out_pulse {
                    for other_name in &node.outputs {
                        // TODO: instead of using name string, use reference to module.
                        self.events.push_back((node.name.to_string(), other_name.to_string(), out_pulse));
                    }
                }
                
            }

            if let Some(node) = self.modules.get_mut(&to_name) {
                node.state = final_state;
            }
        }
    }

    fn run(&mut self, buttons: usize) {
        for _ in 0..buttons {
            self.button();
            self.sim();
        }
    }

    fn get_state_sets(&self, node_name: &str) -> Vec<StateSet> {
        let sets = Vec::new();

        // Uh, we have a problem if there are loops in the graph, which there are.

        // Find node by name
        let module = self.modules.get(node_name).unwrap();

        if module.mod_type == ModType::FlipFlop {
            // 
        }
        else {

        }
        // If it's not a flip flop
        //   For each input
        //     Get the state sets for this input
        //     Concatenate those into sets
        // If it is a flip flop
        //   For each input
        //     Get the state sets for this input
        //     Union all the states into one.
        // 

        // TODO

        sets
    }

    fn compute_count(&self, state_seq: Vec<StateSeq>) -> usize {
        // TODO
    }

    fn run_to_rx(&mut self) -> usize {
        let state_sets = self.get_state_sets("rx");
        let state_seq: Vec<StateSeq> = state_sets.iter()
            .map(|set| StateSeq::new(set))
            .collect();

        let mut count = 0;
        let mut state_seq_done = state_seq.iter()
            .fold(true, |acc, seq| acc && seq.complete());
        self.rx_activated = false;

        // simulate one step at a time until rx is activated or the state sequencer has all
        // the info it needs
        while !self.rx_activated && !state_seq_done {
            count += 1;

            self.button();
            self.sim();

            // TODO : Update state sequencers
        }

        if self.rx_activated {
            // We simulated right up to rx_activated.
            count
        }
        else {
            // Figure it out from state sequencers
            self.compute_count(&state_seq)
        }

    }
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
        
        Answer::Numeric(sim.run_to_rx())
    }
}

#[cfg(test)]
mod test {
    use crate::{day::{Answer, Day}, day20::{Day20, Input, Sim}};

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
    fn test_rx() {
        let input = Input::read("data_aoc2023/day20.txt");
        let mut sim = Sim::new(&input);

        let count = sim.run_to_rx();

        assert_eq!(count, 0);
    }

    #[test]
    fn test_part2() {
        let d = Day20::new("data_aoc2023/day20.txt");

        let count = d.part2();

        assert_eq!(count, Answer::Numeric(0));
    }
}
