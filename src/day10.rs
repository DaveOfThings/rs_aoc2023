use std::{fs::File, io::{BufReader, BufRead}};

use crate::day::{Day, Answer};

#[derive(Debug)]
enum Direction {
    N, E, S, W,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Pipe {
    Ground,
    NE,
    SE,
    NS,
    NW,
    SW,
    EW,
}

#[derive(Debug)]
enum InOutState {
    Outside,
    NorthBorder,
    SouthBorder,
    Inside
}

struct InOutFsm {
    state: InOutState,
    inside: usize,
}

impl InOutFsm {
    fn new() -> InOutFsm {
        InOutFsm { state: InOutState::Outside, inside: 0}
    }

    fn process(&mut self, pipes: &Vec<Pipe>) {
        self.state = InOutState::Outside;
        let mut index = 0;

        for pipe in pipes {
            self.state = match (&self.state, pipe) {
                (InOutState::Outside, Pipe::Ground) => {
                    // We found an outside bit of ground
                    InOutState::Outside    // Stay in outside state
                }
                (InOutState::Inside, Pipe::Ground) => {
                    self.inside += 1;         // We found an inside bit of ground
                    InOutState::Inside     // Stay in inside state
                }
                (InOutState::Outside, Pipe::SE) => { InOutState::NorthBorder }
                (InOutState::Outside, Pipe::NE) => { InOutState::SouthBorder }
                (InOutState::Outside, Pipe::NS) => { InOutState::Inside }
                (InOutState::NorthBorder, Pipe::SW) => { InOutState::Outside }
                (InOutState::NorthBorder, Pipe::EW) => { InOutState::NorthBorder }
                (InOutState::NorthBorder, Pipe::NW) => { InOutState::Inside }
                (InOutState::SouthBorder, Pipe::NW) => { InOutState::Outside }
                (InOutState::SouthBorder, Pipe::EW) => { InOutState::SouthBorder }
                (InOutState::SouthBorder, Pipe::SW) => { InOutState::Inside }
                (InOutState::Inside, Pipe::NE) => { InOutState::NorthBorder }
                (InOutState::Inside, Pipe::SE) => { InOutState::SouthBorder }
                (InOutState::Inside, Pipe::NS) => { InOutState::Outside }
                _ => { 
                    println!("Processing {:?}", pipes);
                    println!("In state {:?} at index {}", &self.state, index);
                    panic!("Invalid pipe found.") 
                }
            };

            index += 1;
        }
    }

    fn get_inside(&self) -> usize {
        self.inside
    }
}

struct Input {
    pipes: Vec<Vec<Pipe>>,
    start: (usize, usize),
}

impl Input {
    fn loop_length(&self) -> usize {
        let mut position = self.start;
        let mut pipe = &self.pipes[position.0][position.1];

        // choose an initial direction of travel
        let mut direction = match pipe {
            Pipe::Ground => { panic!("We shouldn't be here.") }
            Pipe::NE => { Direction::N }
            Pipe::SE => { Direction::E }
            Pipe::NS => { Direction::N }
            Pipe::NW => { Direction::N }
            Pipe::SW => { Direction::S }
            Pipe::EW => { Direction::E }
        };

        // println!("Starting at ({}, {}), moving {:?}", position.0, position.1, direction);

        let mut steps = 0;
        loop {
            // move in the chosen direction
            position = match direction {
                Direction::N => { (position.0-1, position.1) }
                Direction::E => { (position.0,   position.1+1) }
                Direction::S => { (position.0+1, position.1) }
                Direction::W => { (position.0,   position.1-1) }
            };
            
            // update the pipe we are at
            pipe = &self.pipes[position.0][position.1];

            // update the direction we are moving
            direction = match (&pipe, &direction) {
                (Pipe::NE, Direction::S) => {Direction::E}
                (Pipe::NE, Direction::W) => {Direction::N}
                (Pipe::SE, Direction::N) => {Direction::E}
                (Pipe::SE, Direction::W) => {Direction::S}
                (Pipe::NS, Direction::S) => {Direction::S}
                (Pipe::NS, Direction::N) => {Direction::N}
                (Pipe::NW, Direction::S) => {Direction::W}
                (Pipe::NW, Direction::E) => {Direction::N}
                (Pipe::SW, Direction::N) => {Direction::W}
                (Pipe::SW, Direction::E) => {Direction::S}
                (Pipe::EW, Direction::W) => {Direction::W}
                (Pipe::EW, Direction::E) => {Direction::E}
                _ => {
                    println!("At ({}, {}), Pipe {:?}, Direction {:?}",
                        position.0, position.1, &pipe, &direction);

                    panic!();
                }
            };
            // println!("    at ({}, {}), moving {:?}", position.0, position.1, direction);

            // count that step
            steps += 1;

            // break out if we are back at the start
            if position == self.start {
                break;
            }
        }

        steps
    }

    // Produce a cleaned up version of the input with the single loop isolated.
    fn isolated(&self) -> Input {
        // Create a blank map, with ground everywhere.
        let mut cleaned: Vec<Vec<Pipe>> = Vec::new();
        for row in &self.pipes {
            let mut clean_row = Vec::new();
            for _pipe in row {
                clean_row.push(Pipe::Ground);
            }
            cleaned.push(clean_row);
        }

        // Copy the path elements into the blank map
        let mut position = self.start;
        let mut pipe = &self.pipes[position.0][position.1];

        // choose an initial direction of travel
        let mut direction = match pipe {
            Pipe::Ground => { panic!("We shouldn't be here.") }
            Pipe::NE => { Direction::N }
            Pipe::SE => { Direction::E }
            Pipe::NS => { Direction::N }
            Pipe::NW => { Direction::N }
            Pipe::SW => { Direction::S }
            Pipe::EW => { Direction::E }
        };

        // println!("Starting at ({}, {}), moving {:?}", position.0, position.1, direction);

        loop {
            // move in the chosen direction
            position = match direction {
                Direction::N => { (position.0-1, position.1) }
                Direction::E => { (position.0,   position.1+1) }
                Direction::S => { (position.0+1, position.1) }
                Direction::W => { (position.0,   position.1-1) }
            };
            
            // update the pipe we are at
            pipe = &self.pipes[position.0][position.1];

            // update the direction we are moving
            direction = match (&pipe, &direction) {
                (Pipe::NE, Direction::S) => {Direction::E}
                (Pipe::NE, Direction::W) => {Direction::N}
                (Pipe::SE, Direction::N) => {Direction::E}
                (Pipe::SE, Direction::W) => {Direction::S}
                (Pipe::NS, Direction::S) => {Direction::S}
                (Pipe::NS, Direction::N) => {Direction::N}
                (Pipe::NW, Direction::S) => {Direction::W}
                (Pipe::NW, Direction::E) => {Direction::N}
                (Pipe::SW, Direction::N) => {Direction::W}
                (Pipe::SW, Direction::E) => {Direction::S}
                (Pipe::EW, Direction::W) => {Direction::W}
                (Pipe::EW, Direction::E) => {Direction::E}
                _ => {
                    println!("At ({}, {}), Pipe {:?}, Direction {:?}",
                        position.0, position.1, &pipe, &direction);

                    panic!();
                }
            };
            // println!("    at ({}, {}), moving {:?}", position.0, position.1, direction);

            // copy this pipe
            cleaned[position.0][position.1] = self.pipes[position.0][position.1];

            // break out if we are back at the start
            if position == self.start {
                break;
            }
        }


        Input { pipes: cleaned, start: self.start }
    }

    fn enclosed(&self) -> usize {
        // Identify the loop.
        // Make a clean copy with only the loop and open ground
        let clean_input = self.isolated();

        let mut fsm = InOutFsm::new();
        for row in &clean_input.pipes {
            fsm.process(row);
        }

        fsm.get_inside()
    }
}

pub struct Day10<'a> {
    input_filename: &'a str,
}

impl<'a> Day10<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { input_filename: filename }
    }

    fn read_input(filename: &str) -> Input {
        let mut pipes: Vec<Vec<Pipe>> = Vec::new();
        let (mut start_col, mut start_row) = (0, 0);

        let mut row_no = 0;
        let mut col_no = 0;

        let infile = File::open(filename).expect("Failed to open puzzle input.");

        let reader = BufReader::new(infile);
        for line in reader.lines() {
            let mut row: Vec<Pipe> = Vec::new();

            for c in line.unwrap().chars() {
                let (pipe, at_start) = match c {
                    '.' => { (Pipe::Ground, false) }
                    'L' => { (Pipe::NE, false) }
                    'F' => { (Pipe::SE, false) }
                    '|' => { (Pipe::NS, false) }
                    '7' => { (Pipe::SW, false) }
                    'J' => { (Pipe::NW, false) }
                    '-' => { (Pipe::EW, false) }
                    'S' => { (Pipe::Ground, true) }  // the start!
                    _ => {
                        // there should be nothing else
                        panic!("Unknown character in input file");
                    }
                };

                row.push(pipe);
                if at_start {
                    (start_row, start_col) = (row_no, col_no);
                }
                col_no += 1;
            }

            pipes.push(row);
            col_no = 0;
            row_no += 1;
        }

        // Last thing: go fix the pipe type at the start position.
        let north = 
            (start_row > 0) && 
            ((pipes[start_row-1][start_col] == Pipe::NS) ||
             (pipes[start_row-1][start_col] == Pipe::SE) ||
             (pipes[start_row-1][start_col] == Pipe::SW));
        let south = 
             (start_row < pipes.len()-1) && 
             ((pipes[start_row+1][start_col] == Pipe::NS) ||
              (pipes[start_row+1][start_col] == Pipe::NE) ||
              (pipes[start_row+1][start_col] == Pipe::NW));
        let east = 
              (start_col < pipes[0].len()-1) && 
              ((pipes[start_row][start_col+1] == Pipe::EW) ||
               (pipes[start_row][start_col+1] == Pipe::NW) ||
               (pipes[start_row][start_col+1] == Pipe::SW));
        let west = 
               (start_col > 0) && 
               ((pipes[start_row][start_col-1] == Pipe::EW) ||
                (pipes[start_row][start_col-1] == Pipe::NE) ||
                (pipes[start_row][start_col-1] == Pipe::SE));

        pipes[start_row][start_col] = match (north, south, east, west) {
            (true, true, false, false) => { Pipe::NS },
            (true, false, true, false) => { Pipe::NE },
            (false, true, true, false) => { Pipe::SE },
            (true, false, false, true) => { Pipe::NW },
            (false, true, false, true) => { Pipe::SW },
            (false, false, true, true) => { Pipe::EW },
            (false, false, false, false) => { Pipe::Ground },
            _ => { panic!("Bad pipe connections to node.") },
        };

        Input { pipes, start: (start_row, start_col) }
    }
}

impl<'a> Day for Day10<'a> {
    fn part1(&self) -> Answer {
        let input = Day10::read_input(self.input_filename);
        Answer::Numeric(input.loop_length()/2 as usize)
    }

    fn part2(&self) -> Answer {
        let input = Day10::read_input(self.input_filename);
        Answer::Numeric(input.enclosed())
    }
}

#[cfg(test)]
mod test {
    use crate::{day10::Day10, day::{Day, Answer}};

    #[test]
    fn test_input1() {
        let input = Day10::read_input("examples/day10_example1.txt");

        assert_eq!(input.pipes.len(), 5);
        assert_eq!(input.pipes[0].len(), 5);
        assert_eq!(input.start, (1, 1));
    }

    #[test]
    fn test_input2() {
        let input = Day10::read_input("examples/day10_example2.txt");

        assert_eq!(input.pipes.len(), 5);
        assert_eq!(input.pipes[0].len(), 5);
        assert_eq!(input.start, (2, 0));
    }

    #[test]
    fn test_length() {
        let input1 = Day10::read_input("examples/day10_example1.txt");
        assert_eq!(input1.loop_length(), 8);

        let input2 = Day10::read_input("examples/day10_example2.txt");
        assert_eq!(input2.loop_length(), 16);
    }

    #[test]
    fn test_part1_ex1() {
        let d = Day10::new("examples/day10_example1.txt");
        assert_eq!(d.part1(), Answer::Numeric(4));
    }

    #[test]
    fn test_part1_ex2() {
        let d = Day10::new("examples/day10_example2.txt");
        assert_eq!(d.part1(), Answer::Numeric(8));
    }

    #[test]
    fn test_enclosed_ex3() {
        let input = Day10::read_input("examples/day10_example3.txt");
        assert_eq!(input.enclosed(), 4);
    }

    #[test]
    fn test_enclosed_ex4() {
        let input = Day10::read_input("examples/day10_example4.txt");
        assert_eq!(input.enclosed(), 8);
    }
    
    #[test]
    fn test_enclosed_ex5() {
        let input = Day10::read_input("examples/day10_example5.txt");
        assert_eq!(input.enclosed(), 10);
    }
    
    #[test]
    fn test_part1_ex5() {
        let d = Day10::new("examples/day10_example5.txt");
        assert_eq!(d.part2(), Answer::Numeric(10));
    }
}