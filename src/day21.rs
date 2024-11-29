use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}};

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

        let mut y = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            width = line.len() as isize;
            let mut x = 0;
            for c in line.chars() {
                match c {
                    '.' => {
                        // garden patch
                        // println!("Inserting at ({x}, {y})");
                        garden.insert((x, y));
                    }
                    '#' => {
                        // rock
                        // (nothing to do)
                    }
                    'S' => {
                        // start position (garden patch)
                        start = (x, y);
                        garden.insert((x, y));
                    }
                    _ => {
                        // Something illegal.
                        panic!();
                    }
                }
                x += 1;
            }
            y += 1;
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

    fn num_by_steps(steps: usize, input: &Input) -> usize {
        let mut aged_out: usize = 0;

        // We're going to do let 'current' and 'next' swap between these maps.
        let mut map1: HashMap<(isize, isize), usize> = HashMap::new();
        let mut map2: HashMap<(isize, isize), usize> = HashMap::new();

        let current = &mut map1;
        let next = &mut map2;

        // start position is the only occupied space, initially.
        current.insert(input.start, 1);  

        for _ in 0..steps {
            // clear out occupancy of next before we start
            next.clear();

            for ((x, y), t) in current.iter() {
                // If this position has been on for N time ticks ...
                match t {
                    1 => {
                        // Check all neighbors.  If off, turn them on for next iteration.
                        for (nx, ny) in [((*x+1), *y), (*x, (*y+1)), ((*x-1), *y), (*x, (*y-1))] {
                            let check_x = nx.rem_euclid(input.width);
                            let check_y = ny.rem_euclid(input.height);
        
                            // if (nx, ny) is in garden set and it's not active, activate for next cycle
                            if input.garden.contains(&(check_x, check_y)) {
                                if !current.contains_key(&(nx, ny)) {
                                    // turn  (nx, ny) on for the first time.
                                    next.insert((nx, ny), 1);
                                }
                                else {
                                    // a neighbor is occupied, that will keep the lights on for this square for a second tick.
                                    next.insert((*x, *y), 2);
                                    println!("({}, {}) is on for two ticks.", *x, *y);
                                    assert!(false);
                                }
                            }
                        }
                    }
                    2 => {
                        // All neighbors should already be on for the last 1 tick.  This position will
                        // age to 3 ticks.
                        next.insert((*x, *y), 3);
                    }
                    _ => {
                        // let permanently on squares that are 3 or more ticks old be dropped.  They
                        // shouldn't affect any future cells as those cells will now be 2 or more ticks
                        // old and stuck on.  We do need to keep count of these that have dropped, though.
                        aged_out += 1;
                    }
                }
                
            }

            // set occupied to next
            std::mem::swap(current, next);
        }

        let num_occupied = &current.len() + aged_out;

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

        let cases: [(usize, usize); 5] = [
            (6, 16), 
            (10, 50), 
            (50, 1594),
            (100, 6536), 
            (500, 167004), 
            //(1000, 668697), 
            // (5000, 16733044)
            ];
        for (steps, expected) in &cases {
            let reach = Day21::num_by_steps(*steps, &input);
            assert_eq!(reach, *expected);
        }
    }

    #[test]
    fn test_mod() {
        for n in -5..5isize {
            println!("{n} rem_euclid 3 is {}", n.rem_euclid(3));
        }
        assert!(true);
    }
}
