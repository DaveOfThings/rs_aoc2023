use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

use crate::day::{Day, Answer};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    dir: Direction,
    dist: usize,
    dir2: Direction,
    dist2: usize,
}

lazy_static! {
    // caps[1]: "R", "L", "U", "D"
    // caps[2]: number, distance
    // caps[3]: two hex digits for red
    // caps[4]: two hex digits for green
    // caps[5]: two hex digits for blue
    static ref INSTR_RE: Regex = Regex::new("([RLUD]) ([0-9]+) \\(#([0-9a-f]{5})([0-9a-f]{1})\\)").unwrap();
}

impl Instruction {
    pub fn parse(line: &str) -> Option<Instruction> {
        if let Some(caps) = INSTR_RE.captures(line) {
            let dir = match &caps[1] {
                "U" => { Direction::Up }
                "D" => { Direction::Down }
                "R" => { Direction::Right }
                "L" => { Direction::Left }
                _ => panic!("Wrong letter for direction.")
            };
            let dist = caps[2].parse::<usize>().unwrap();

            let dist2 = usize::from_str_radix(&caps[3], 16).unwrap();
            let dir2 = match &caps[4] {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => panic!("Bad direction")
            };

            Some( Instruction { dir, dist, dir2, dist2 } )
        }
        else {
            None
        }
    }

    fn dir2_dist2(&self) -> (Direction, usize) {
        (self.dir2, self.dist2)
    }
}

struct Input {
    instructions: Vec<Instruction>,
}

#[derive(Debug, PartialEq, Eq)]
enum InOutState {
    Outside,
    NorthBorder,
    SouthBorder,
    Inside
}

struct InOutFsm {
    state: InOutState,
}

impl InOutFsm {
    fn new() -> InOutFsm {
        InOutFsm { state: InOutState::Outside}
    }

    // Returns true if in the interior
    fn step(&mut self, cell: bool, above: bool, below: bool) -> bool {
        self.state = match (&self.state, cell, above, below) {
            // If cell is false, state can't change
            (&InOutState::Outside, false, _, _) => { InOutState::Outside }
            (&InOutState::Inside,  false, _, _) => { InOutState::Inside  }
            (&InOutState::NorthBorder, false, _, _) => { panic!("Weird transition") }
            (&InOutState::SouthBorder,  false, _, _) => { panic!("Weird transition")  }

            // Other combos from Outside
            (&InOutState::Outside, true, false, false) => { panic!("Weird transition") }
            (&InOutState::Outside, true, false, true) => { InOutState::NorthBorder }
            (&InOutState::Outside, true, true, false) => { InOutState::SouthBorder }
            (&InOutState::Outside, true, true, true) => { InOutState::Inside }

            // Other combos from Inside
            (&InOutState::Inside, true, false, false) => { panic!("Weird transition") }
            (&InOutState::Inside, true, false, true) => { InOutState::SouthBorder }
            (&InOutState::Inside, true, true, false) => { InOutState::NorthBorder }
            (&InOutState::Inside, true, true, true) => { InOutState::Outside }

            // Other combos from North Border
            (&InOutState::NorthBorder, true, false, false) => { InOutState::NorthBorder }
            (&InOutState::NorthBorder, true, false, true) => { InOutState::Outside }
            (&InOutState::NorthBorder, true, true, false) => { InOutState::Inside }
            (&InOutState::NorthBorder, true, true, true) => { panic!("Weird transition") }

            // Other combos from South Border
            (&InOutState::SouthBorder, true, false, false) => { InOutState::SouthBorder }
            (&InOutState::SouthBorder, true, false, true) => { InOutState::Inside }
            (&InOutState::SouthBorder, true, true, false) => { InOutState::Outside }
            (&InOutState::SouthBorder, true, true, true) => { panic!("weird transition") }
        };

        // Return true if in interior (Any place on the border counts as inside.)
        cell || (self.state != InOutState::Outside)

    }

    fn reset(&mut self) {
        self.state = InOutState::Outside;
    }
}

impl Input {
    pub fn read(filename: &str) -> Input {
        let f = File::open(filename).unwrap();
        let reader = BufReader::new(f);
        let mut instructions: Vec<Instruction> = Vec::new();

        for line in reader.lines() {
            if let Some(instruction) = Instruction::parse(&line.unwrap()) {
                instructions.push(instruction);
            }
        }

        Input { instructions }
    }

    pub fn volume(&self) -> usize {

        // Direction -> (delta_row, delta_cp;)
        let delta: HashMap<Direction, (isize, isize)> = HashMap::from([
            (Direction::Up, (-1, 0)),
            (Direction::Down, (1, 0)),
            (Direction::Left, (0, -1)),
            (Direction::Right, (0, 1)),
        ]);
        let mut border: HashMap<(isize, isize), isize> = HashMap::new();        
        let mut interior: HashMap<(isize, isize), isize> = HashMap::new();

        let mut min_row = 0;
        let mut min_col = 0;
        let mut max_row = 0;
        let mut max_col = 0;

        // Interpret instructions from input, drawing outline
        let (mut row, mut col) = (0, 0);
        border.insert((row, col), 1);

        for i in &self.instructions {
            let (delta_row, delta_col) = delta[&i.dir];

            for _n in 0..i.dist {
                row += delta_row;
                col += delta_col;
                border.insert((row, col), 1);
            }

            if row < min_row { min_row = row; }
            if row > max_row { max_row = row; }
            if col < min_col { min_col = col; }
            if col > max_col { max_col = col; }
        }
        
        let mut fsm = InOutFsm::new();
        for row in min_row..=max_row {
            fsm.reset();
            for col in min_col..=max_col {
                let cell = border.contains_key( &(row, col) );
                let above = border.contains_key( &(row-1, col) );
                let below = border.contains_key( &(row+1, col) );
                if fsm.step(cell, above, below) {
                    // Interior cell, make sure it's in map
                    interior.insert((row, col), 1);
                }
            }
        }

        interior.len()
    }

    pub fn volume2(&self) -> usize {
        // Direction -> (delta_row, delta_cp;)
        let delta: HashMap<Direction, (isize, isize)> = HashMap::from([
            (Direction::Up, (-1, 0)),
            (Direction::Down, (1, 0)),
            (Direction::Left, (0, -1)),
            (Direction::Right, (0, 1)),
        ]);
        let mut coords: Vec<(isize, isize)> = Vec::new();
        let mut moves = 0;

        // Construct a list of coordinates making up the perimeter.
        let (mut row, mut col) = (0, 0);
        coords.push((row, col));

        for i in &self.instructions {
            let (dir2, dist2) = i.dir2_dist2();
            moves += dist2;
            let (delta_row, delta_col) = delta[&dir2];
            row = row + delta_row * dist2 as isize;
            col = col + delta_col * dist2 as isize;

            // println!("Pushing coord ({row}, {col})");
            coords.push((row, col));
        }

        // Sum up the contribution from all the point pairs
        let mut sum = 0;
        let (mut x1, mut y1) = coords[0];
        for n in 0..coords.len()-1 {
            let (x0, y0) = (x1, y1);
            x1 = coords[n+1].0;
            y1 = coords[n+1].1;
            sum += x0*y1 - y0*x1;
            // println!("  {x0}*{y1} - {y0}*{x1} = {}", x0*y1 - y0*x1);
            // println!("  sum: {sum}");
        }

        // Don't forget the wrap around
        let (x0, y0) = (x1, y1);
        x1 = coords[0].0;
        y1 = coords[0].1;
        sum += x0*y1 - y0*x1;
        // println!("  {x0}*{y1} - {y0}*{x1} = {}", x0*y1 - y0*x1);
        // println!("  sum: {sum}");

        let sum = sum.abs() as usize;

        let area = sum/2 + moves/2 + 1;

        area
    }
}



/*
struct Dig {
    // maps (row, col) to depth.
    border: HashMap<(isize, isize), isize>,
    interior: HashMap<(isize, isize), isize>,

    min_row: isize,
    min_col: isize,
    max_row: isize,
    max_col: isize,
}

impl Dig {
    pub fn new(input: &Input) -> Dig {


        Dig { border, interior, min_row, min_col, max_row, max_col }
    }


}

*/



pub struct Day18<'a> {
    input_filename: &'a str,
}

impl<'a> Day18<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { input_filename: filename }
    }
}

impl<'a> Day for Day18<'a> {
    fn part1(&self) -> Answer {
        let input = Input::read(self.input_filename);

        Answer::Numeric(input.volume())
    }

    fn part2(&self) -> Answer {
        let input = Input::read(self.input_filename);

        Answer::Numeric(input.volume2())
    }
}

#[cfg(test)]
mod tests {
    use crate::{day18::{Input, Direction, Day18}, day::{Answer, Day}};

    #[test]
    fn test_input() {
        let input = Input::read("examples/day18_example1.txt");

        assert_eq!(input.instructions.len(), 14);
        assert_eq!(input.instructions[0].dir, Direction::Right);
        assert_eq!(input.instructions[0].dist, 6);
        assert_eq!(input.instructions[0].dir2, Direction::Right);
        assert_eq!(input.instructions[0].dist2, 0x70c71);
    }

    #[test]
    fn test_volume() {
        let input = Input::read("examples/day18_example1.txt");

        assert_eq!(input.volume(), 62);
    }

    #[test]
    fn test_volume2() {
        let input = Input::read("examples/day18_example1.txt");

        assert_eq!(input.volume2(), 952408144115);
    }

    #[test]
    fn test_part1() {
        let d = Day18::new("examples/day18_example1.txt");
        
        assert_eq!(d.part1(), Answer::Numeric(62));
    }

    #[test]
    fn test_part2() {
        let d = Day18::new("examples/day18_example1.txt");
        
        assert_eq!(d.part2(), Answer::Numeric(952408144115));
    }
}
