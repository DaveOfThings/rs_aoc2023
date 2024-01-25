use std::{collections::{HashMap, VecDeque}, fs::File, io::{BufRead, BufReader}, rc::Rc};

use crate::day::{Day, Answer};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref LINE_RE: Regex = Regex::new("([%&]?)([a-x]+) -> (.*)").unwrap();
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
    inputs: Vec<Rc<Module>>,

    // outputs
    outputs: Vec<Rc<Module>>,

    // state
    state: bool,
}

impl Module {
    pub fn new(mod_type: &ModType, name: &str) -> Module {
        // TODO

        Module { mod_type: mod_type, name: name.to_string(), outputs: Vec::new(), state: false }
    }

    pub fn connect_to(&self, other: &Module) {
        // TODO
    }
}

struct Sim<'a> {
    modules: HashMap<String, Module>,     // states of nodes
    events: VecDeque<(bool, String)>,

    low_pulses: usize,
    high_pulses: usize,
}

impl<'a> Sim<'a> {
    fn new(input: &'a Input) -> Sim {
        let modules: HashMap<String, Module> = HashMap::new(); // states of nodes

        // TODO : Create Modules from input
        for line in input.lines {
            let name = line.1;
            let module = Module::new(line.0, name);
            modules.insert(name, mod);
        }
        input.lines.iter()
            .map(|line| Module::new(line.0, line.1)).collect()

        // TODO : Connect modules to each other

        let events: VecDeque<(bool, String)> = VecDeque::new(); // event queue
        Sim {modules, events, low_pulses: 0, high_pulses: 0 }
    }

    fn product(&self) -> usize {
        self.low_pulses * self.high_pulses
    }

    // simulate a button press
    fn button(&mut self) {
        // push one event, a low signal to 'broadcaster'
        self.events.push_back( (false, "broadcaster".to_string()) );
    }

    fn sim(&mut self) {
         // run until the event queue is empty
        while !self.events.is_empty() {
            // pop an event
            let (high, name) = self.events.pop_front().unwrap();

            // count it
            if high {
                self.high_pulses += 1;
            }
            else {
                self.low_pulses += 1;
            }

            // Locate the node
            if let Some(node) = self.modules.get(name) {
                // Update the node and propagate the pulses
                match node.mod_type {
                    ModType::Node => {}
                    ModType::Nand(inputs) => {
                        // remember this pulse type for this input.
                        // TODO : Associate this pulse with its source
                        // TODO : Have AND type gates know their inputs.
                        input.set()
                    }
                    ModType::FlipFlop => {

                    }
                }
            }
        }
    }

    fn run(&mut self, buttons: usize) {
        for _ in 0..buttons {
            self.button();
            self.sim();
        }
    }
}

pub struct Day20<'a> {
    _input_filename: &'a str,
}

impl<'a> Day20<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { _input_filename: filename }
    }
}

impl<'a> Day for Day20<'a> {
    fn part1(&self) -> Answer {
        Answer::None
    }

    fn part2(&self) -> Answer {
        Answer::None
    }
}

#[cfg(test)]
mod test {
    use crate::day20::{Input, Sim};

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
    fn test_sim1() {
        let input = Input::read("examples/day20_example2.txt");
        let sim = Sim::new();

        sim.run(&input);
        assert_eq!(sim.low_pulses, 8000);
        assert_eq!(sim.high_pulses, 4000);
        assert_eq!(sim.product(), 32000000);
    }

    #[test]
    fn test_sim2() {
        let input = Input::read("examples/day20_example2.txt");
        let sim = Sim::new();

        sim.run(&input);
        assert_eq!(sim.low_pulses, 4250);
        assert_eq!(sim.high_pulses, 2750);
        assert_eq!(sim.product(), 4250*2750);
    }
}
