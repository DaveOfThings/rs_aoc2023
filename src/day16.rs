use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet, HashMap}};

use crate::day::{Day, Answer};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum GridElt {
    Empty,
    ReflectBackslash,
    ReflectSlash,
    SplitUpDown,
    SplitLeftRight,
}

struct Input {
    grid: Vec<Vec<GridElt>>,
}

impl Input {
    pub fn read(filename: &str) -> Input {
        let mut grid: Vec<Vec<GridElt>> = Vec::new();

        let f = File::open(filename).unwrap();
        let reader = BufReader::new(f);
        for line in reader.lines() {
            let mut grid_line: Vec<GridElt> = Vec::new();
            for c in line.unwrap().chars() {
                let push_elt: Option<GridElt> = match c {
                    '.' => Some(GridElt::Empty),
                    '\\' => Some(GridElt::ReflectBackslash),
                    '/' => Some(GridElt::ReflectSlash),
                    '|' => Some(GridElt::SplitUpDown),
                    '-' => Some(GridElt::SplitLeftRight),
                    _ => None,
                };

                if let Some(elt) = push_elt {
                    grid_line.push(elt);
                }
            }

            if grid_line.len() > 0 {
                grid.push(grid_line);
            }
        }

        Input { grid }
    }

    pub fn energize(&self, dir: Direction, row: usize, col: usize) -> usize {
        // A ray with <Direction>, encounting <GridElt> will produce a vector of rays,
        // with new_direction <Direction>.
        let propagation: HashMap<(Direction, GridElt), Vec<Direction>> = HashMap::from([
            // Empty grid cell
            ( (Direction::Up, GridElt::Empty), vec![Direction::Up] ),
            ( (Direction::Down, GridElt::Empty), vec![Direction::Down] ),
            ( (Direction::Left, GridElt::Empty), vec![Direction::Left] ),
            ( (Direction::Right, GridElt::Empty), vec![Direction::Right] ),

            // ReflectSlash grid cell
            ( (Direction::Up, GridElt::ReflectSlash), vec![Direction::Right] ),
            ( (Direction::Down, GridElt::ReflectSlash), vec![Direction::Left] ),
            ( (Direction::Left, GridElt::ReflectSlash), vec![Direction::Down] ),
            ( (Direction::Right, GridElt::ReflectSlash), vec![Direction::Up] ),

            // ReflectBackslash grid cell
            ( (Direction::Up, GridElt::ReflectBackslash), vec![Direction::Left] ),
            ( (Direction::Down, GridElt::ReflectBackslash), vec![Direction::Right] ),
            ( (Direction::Left, GridElt::ReflectBackslash), vec![Direction::Up] ),
            ( (Direction::Right, GridElt::ReflectBackslash), vec![Direction::Down] ),

            // SplitUpDown grid cell
            ( (Direction::Up, GridElt::SplitUpDown), vec![Direction::Up] ),
            ( (Direction::Down, GridElt::SplitUpDown), vec![Direction::Down] ),
            ( (Direction::Left, GridElt::SplitUpDown), vec![Direction::Up, Direction::Down] ),
            ( (Direction::Right, GridElt::SplitUpDown), vec![Direction::Down, Direction::Up] ),

            // SplitLeftRight grid cell
            ( (Direction::Up, GridElt::SplitLeftRight), vec![Direction::Left, Direction::Right] ),
            ( (Direction::Down, GridElt::SplitLeftRight), vec![Direction::Left, Direction::Right] ),
            ( (Direction::Left, GridElt::SplitLeftRight), vec![Direction::Left] ),
            ( (Direction::Right, GridElt::SplitLeftRight), vec![Direction::Right] ),
        ]);

        let movement: HashMap<Direction, (isize, isize)> = HashMap::from([
            (Direction::Up, (-1, 0)),
            (Direction::Down, (1, 0)),
            (Direction::Left, (0, -1)),
            (Direction::Right, (0, 1)),
        ]);

        let rows = self.grid.len() as isize;
        let cols = self.grid[0].len() as isize;

        // These are beams to chase down: (row, col, direction)
        let mut to_do: Vec<(usize, usize, Direction)> = Vec::new();

        // beams that have already been processed
        let mut done: HashSet<(usize, usize, Direction)> = HashSet::new();

        // energized cells
        let mut energized: HashSet<(usize, usize)> = HashSet::new();

        to_do.push( (row, col, dir) );
        done.insert( (row, col, dir) );
        energized.insert( (row, col) );

        while to_do.len() > 0 {
            let (row, col, dir) = to_do.pop().unwrap();

            // propagate
            // println!("{} : Propagating {dir:?} into ({row},{col}), {:?}", to_do.len()+1, self.grid[row][col]);
            let propagated = propagation.get( &(dir, self.grid[row][col]) ).unwrap();
            for dir in propagated {
                let (d_row, d_col) = movement.get(dir).unwrap();
                let new_col = col as isize + d_col;
                let new_row = row as isize + d_row;

                // println!("  ({row}, {col}), go {dir:?}, to ({new_row}, {new_col})");

                if new_col < 0 || new_col >= cols ||
                   new_row < 0 || new_row >= rows {
                    // this propagation went off the edge
                    // println!("    off the map.");
                    continue;
                }

                let new_col = new_col as usize;
                let new_row = new_row as usize;

                if !done.contains(&(new_row, new_col, *dir)) {
                    // we need to explore this further
                    to_do.push((new_row, new_col, *dir));
                    done.insert((new_row, new_col, *dir));
                    energized.insert((new_row, new_col));
                }
                else {
                    // println!("    been there.");
                }
            }
        }

        // return the number of energized cells
        energized.len()
    }

    fn max_energize(&self) -> usize {
        // create a vector of all possible initial beams
        let mut options: Vec<(Direction, usize, usize)> = Vec::new();

        // Right, Left beams
        let last_row = self.grid.len()-1;
        let last_col = self.grid[0].len()-1;
        for row in 0..=last_row {
            options.push( (Direction::Right, row, 0) );
            options.push( (Direction::Left, row, last_col) );
        }
        for col in 0..=last_col {
            options.push( (Direction::Down, 0, col) );
            options.push( (Direction::Up, last_row, col) );
        }

        options.iter()
            .map(|(dir, row, col)| self.energize(*dir, *row, *col))
            .max().unwrap()
    }
}

pub struct Day16<'a> {
    _input_filename: &'a str,
}

impl<'a> Day16<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { _input_filename: filename }
    }
}

impl<'a> Day for Day16<'a> {
    fn part1(&self) -> Answer {
        let input = Input::read(self._input_filename);

        Answer::Numeric(input.energize(Direction::Right, 0, 0))
    }

    fn part2(&self) -> Answer {
        let input = Input::read(self._input_filename);

        Answer::Numeric(input.max_energize())
    }
}

#[cfg(test)]
mod tests {
    use crate::{day16::{GridElt, Input, Day16, Direction}, day::{Answer, Day}};

    #[test]
    fn test_input() {
        let input = Input::read("examples/day16_example1.txt");

        assert_eq!(input.grid.len(), 10);
        assert_eq!(input.grid[0].len(), 10);
        assert_eq!(input.grid[0][0], GridElt::Empty);
        assert_eq!(input.grid[0][5], GridElt::ReflectBackslash);
        assert_eq!(input.grid[9][2], GridElt::ReflectSlash);
        assert_eq!(input.grid[0][1], GridElt::SplitUpDown);
        assert_eq!(input.grid[1][2], GridElt::SplitLeftRight);
    }

    #[test]
    fn test_energized() {
        let input = Input::read("examples/day16_example1.txt");

        assert_eq!(input.energize(Direction::Right, 0, 0), 46);
    }

    #[test]
    fn test_part1() {
        let d = Day16::new("examples/day16_example1.txt");

        assert_eq!(d.part1(), Answer::Numeric(46));
    }

    #[test]
    fn test_max_energized() {
        let input = Input::read("examples/day16_example1.txt");

        assert_eq!(input.max_energize(), 51);
    }

    #[test]
    fn test_part2() {
        let d = Day16::new("examples/day16_example1.txt");

        assert_eq!(d.part2(), Answer::Numeric(51));
    }
}
