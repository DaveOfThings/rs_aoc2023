use std::{cmp, collections::HashSet, fs::File, io::{BufRead, BufReader}};

use crate::day::{Day, Answer};

struct Input {
    width: isize,
    height: isize,
    start: (isize, isize),
    garden: HashSet<(isize, isize)>,
}

impl Input {
    fn read(filename: &str) -> Input {
        let mut width: isize = 0;
        let mut height: isize = 0;
        let mut start: (isize, isize) = (0, 0);
        let mut garden: HashSet<(isize, isize)> = HashSet::new();

        let f = File::open(filename).unwrap();
        let reader = BufReader::new(f);

        for (y, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            width = line.len() as isize;
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => {
                        // garden patch
                        // println!("Inserting at ({x}, {y})");
                        garden.insert((x as isize, y as isize));
                    }
                    '#' => {
                        // rock
                        // (nothing to do)
                    }
                    'S' => {
                        // start position (garden patch)
                        start = (x as isize, y as isize);
                        garden.insert((x as isize, y as isize));
                    }
                    _ => {
                        // Something illegal.
                        panic!();
                    }
                }
            }
            height += 1;
        }

        Input { width, height, start, garden }

    }

}

pub struct Day21<'a> {
    input_filename: &'a str,
}

impl<'a> Day21<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { input_filename: filename }
    }

/*
    fn tile_occupied(input: &Input, start: (isize, isize), steps: usize) -> usize
    {
        // compute max number of steps we'll need to take.  If asked for more than this
        // we'll truncate to this while keeping parity.
        let max_x = cmp::max(start.0-1, input.width-start.0-1);
        let max_y = cmp::max(start.1-1, input.height-start.1-1);
        let max_steps = max_x + max_y;

        0  // TODO-DW
    }
*/

    fn num_by_steps(steps: usize, input: &Input) -> usize {
        // We're going to do let 'current' and 'next' swap between these maps.
        let mut set1: HashSet<(isize, isize)> = HashSet::new();
        let mut set2: HashSet<(isize, isize)> = HashSet::new();

        let current = &mut set1;
        let next = &mut set2;

        // start position is the only occupied space, initially.
        current.insert(input.start);  

        for _ in 0..steps {
            // clear out occupancy of next before we start
            next.clear();

            for (x, y) in current.iter() {
                // Check all neighbors.  If off, turn them on for next iteration.
                for (nx, ny) in [((*x+1), *y), (*x, (*y+1)), ((*x-1), *y), (*x, (*y-1))] {
                    let check_x = nx.rem_euclid(input.width);
                    let check_y = ny.rem_euclid(input.height);

                    // if (nx, ny) is in garden set put it in the next cycle
                    if input.garden.contains(&(check_x, check_y)) {
                        next.insert((nx, ny));
                    }
                }                
            }

            // set occupied to next
            std::mem::swap(current, next);
        }

        let num_occupied = current.len();

        /*
        for y in 0..input.height {
            for x in 0..input.width {
                if current.contains(&(x, y)) {
                    print!("O");
                }
                else if input.garden.contains(&(x, y)) {
                    print!(".");
                }
                else {
                    print!("#");
                }
            }
            println!();
        }
        println!();
        */

        num_occupied
    
    }
}

impl<'a> Day for Day21<'a> {
    fn part1(&self) -> Answer {
        let input = Input::read(self.input_filename);

        let occupied = Day21::num_by_steps(64, &input);

        Answer::Numeric(occupied)
    }

    fn part2(&self) -> Answer {
        Answer::None
    }
}

#[cfg(test)]
mod test {
    use crate::day21::{Day21, Input};

    #[test]
    fn test_input() {
        let input = Input::read("examples/day21_example1.txt");
        assert_eq!(input.width, 11);
        assert_eq!(input.height, 11);
        assert_eq!(input.start, (5, 5));
        assert!(!input.garden.contains(&(1, 2)));
        assert!(input.garden.contains(&(0, 2)));
    }

    #[test]
    fn test_num_by_steps() {
        let input = Input::read("examples/day21_example1.txt");

        let cases: [(usize, usize); 4] = [
            (6, 16), 
            (10, 50), 
            (50, 1594),
            (100, 6536), 
            // (500, 167004), 
            //(1000, 668697), 
            // (5000, 16733044)
            ];
        for (steps, expected) in &cases {
            let reach = Day21::num_by_steps(*steps, &input);
            assert_eq!(reach, *expected);
        }
    }
}
