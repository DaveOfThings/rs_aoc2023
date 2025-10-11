use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

    fn to_left(&self) -> isize {
        self.start.0
    }

    fn to_right(&self) -> isize {
        self.width - self.start.0 - 1
    }

    fn to_top(&self) -> isize {
        self.start.1
    }

    fn to_bottom(&self) -> isize {
        self.height - self.start.1 - 1
    }
}


// Different classes of blocks, based on where the propagation started.
#[derive(EnumIter, Debug)]
enum FillFrom {
    // Start in the center
    Center,

    // Started in right, center
    // Each new class goes around the compass counter clockwise
    East,
    NorthEast,
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
}

#[derive(EnumIter, Debug)]
enum FillAmount {
    Least,                  // The tile on the leading edge of the propagation front
    NextLeast,              // A tile with part of the propagation front
    FullOdd,                // A full tile that starts on an odd time step.
    FullEven,               // A full tile that starts on an even time step.

    // Note: Since "Odd" and "Even" denote which time step a tile normally starts on,
    // To evaluate how many places are set after <steps> steps, for a tile, seed it with
    // the starting step then run the minimum even number of steps to fill the tile from 
    // that position.  Then run one more step if <steps> was odd XOR It was FullOdd instead
    // of FullEven.
    // TODO : Confirm that num_of() and test_num_of() adhere to this convention.
    // TODO : Implement and test count_for() to use this convention.
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

// TODO: Replace hard coded constants, 131, 132, 65, etc, with functions derived from input.

    fn num_of(start: &FillFrom, amount: &FillAmount, steps: usize, _input: &Input) -> usize {
        match start {
            FillFrom::Center => {
                match amount {
                    FillAmount::Least => if steps < 132 { 1 } else { 0 },
                    FillAmount::NextLeast => 0,
                    FillAmount::FullOdd => 0,
                    FillAmount::FullEven => if steps >= 132 { 1 } else { 0 },
                }
            }
            FillFrom::East | FillFrom::North | FillFrom::West | FillFrom::South => {
                // Count tiles on horizontal and vertical axes
                match amount {
                    FillAmount::Least => if steps < 66 { 0 } else { 1 },
                    FillAmount::NextLeast => if steps <= 131+65 { 0 } else { 1 },
                    FillAmount::FullEven => {
                        if steps <= 2*131+65 { 0 }
                        else {
                            // 65 across center, 131 steps per tile, subtract Least and NextLeast tiles.
                            // finally add one (to round up) then divide by 2 to get just even
                            ((steps - 65)/131 - 2 + 1)/2
                        }
                    }
                    FillAmount::FullOdd => {
                        if steps <= 3*131+65 { 0 }
                        else {
                            // 66 steps to get out of center, 131 steps per tile, subtract Least and NextLeast tiles.
                            // Add one to round up and finally divide by 2 to get just odd
                            ((steps - 65)/131 - 2)/2
                        }
                    }
                }
            }
            FillFrom::NorthEast | FillFrom::NorthWest | FillFrom::SouthEast | FillFrom::SouthWest => {
                // Count tiles on triangular regions
                match amount {
                    FillAmount::Least => if steps < 132 { 0 } else {
                        // These are the leading edge of the diagonals
                        (steps-1)/131
                    }
                    FillAmount::NextLeast => if steps < 132+131 { 0 } else {
                        // These are just behind the leading edge of the diagonals
                        (steps - (1+131))/131
                    }
                    FillAmount::FullEven => if steps < 132+2*131 { 0 } else {
                        // These are the bulk of the triangular area, with even step counts
                        let diags = (steps - (1+2*131))/131;
                        let even_diags = (diags+1)/2;
                        let even_tiles = even_diags * even_diags;
                        even_tiles
                    }
                    FillAmount::FullOdd => if steps < 132+2*131 { 0 } else {
                        // These are the bulk of the triangular area, with odd step counts
                        let diags = (steps - (1+2*131))/131;
                        let odd_diags = diags/2;
                        let odd_tiles = odd_diags * (odd_diags + 1);
                        odd_tiles
                    }

                }
            }
        }
    }

    fn count_for(start: &FillFrom, amount: &FillAmount, steps: usize, input: &Input) -> usize {
        let sim_start = match start {
            FillFrom::NorthWest => (0, 0),
            FillFrom::North => (65, 0),
            FillFrom::NorthEast => (130, 0),
            FillFrom::West => (0, 65),
            FillFrom::Center => (65, 65),
            FillFrom::East => (130, 65),
            FillFrom::SouthWest => (0, 130),
            FillFrom::South => (65, 130),
            FillFrom::SouthEast => (130, 130),
        };

        let sim_steps = match amount {
            FillAmount::Least => {
                match start {
                    FillFrom::Center => steps,
                    FillFrom::North | FillFrom::East | FillFrom::South | FillFrom::West => (steps - 66) % 131,
                    FillFrom::NorthEast | FillFrom::NorthWest | FillFrom::SouthEast | FillFrom::SouthWest => (steps - 132) % 131
                }
            }
            FillAmount::NextLeast => {
                match start {
                    FillFrom::Center => panic!("Center as NextLeast shouldn't happen."),  // Shouldn't happen
                    FillFrom::North | FillFrom::East | FillFrom::South | FillFrom::West => (steps - 66) % 131 + 131,
                    FillFrom::NorthEast | FillFrom::NorthWest | FillFrom::SouthEast | FillFrom::SouthWest => (steps - 132) % 131 + 131
                }
            }
            FillAmount::FullEven => {
                let for_even = match start {
                    FillFrom::Center => 132,
                    FillFrom::North | FillFrom::East | FillFrom::South | FillFrom::West => 131+65,
                    FillFrom::NorthEast | FillFrom::NorthWest | FillFrom::SouthEast | FillFrom::SouthWest => 2*131,
                };
                if steps & 1 == 0 { for_even } else { for_even + 1 }
            }
            FillAmount::FullOdd => {
                let for_odd = match start {
                    FillFrom::Center => panic!("Center as FullOdd shouldn't happen."),  // Shouldn't happen
                    FillFrom::North | FillFrom::East | FillFrom::South | FillFrom::West => 131+65,
                    FillFrom::NorthEast | FillFrom::NorthWest | FillFrom::SouthEast | FillFrom::SouthWest => 2*131
                };
                if steps & 1 == 1 { for_odd } else { for_odd + 1 }
            }
        };
        
        let count_for = Day21::num_by_steps(sim_steps, input, &sim_start, false);
        println!("  count_for(start:{start:?}, amount:{amount:?}, steps:{sim_steps:?}) -> {count_for}");

        count_for
    }

    fn num_by_tile_analysis(steps: usize, input: &Input) -> usize {
        let mut sum = 0;
        
        for start in FillFrom::iter() {
            for amount in FillAmount::iter() {
                let tiles = Self::num_of(&start, &amount, steps, input);       // how many tiles fit this start/amount
                if tiles != 0 {
                    let count = Self::count_for(&start, &amount, steps, input);    // how many places reached start/amount
                    println!("{start:16?} {amount:16?} : {tiles} tiles of {count} places -> {}", tiles*count);
                    sum += tiles * count;
                }
            }
        }

        sum
    }

    fn num_by_steps(steps: usize, input: &Input, start: &(isize, isize), infinite: bool) -> usize {
        // We're going to do let 'current' and 'next' swap between these maps.
        let mut set1: HashSet<(isize, isize)> = HashSet::new();
        let mut set2: HashSet<(isize, isize)> = HashSet::new();

        let current = &mut set1;
        let next = &mut set2;

        // start position is the only occupied space, initially.
        current.insert(*start);  

        // println!("Doing num_by_steps: steps: {steps}, start:{start:?}, infinite:{infinite:?}");

        for _ in 0..steps {
            // clear out occupancy of next before we start
            next.clear();

            for (x, y) in current.iter() {
                // Check all neighbors.  If off, turn them on for next iteration.
                for (nx, ny) in [((*x+1), *y), (*x, (*y+1)), ((*x-1), *y), (*x, (*y-1))] {
                    if !infinite {
                        if (nx < 0) || (nx >= input.width) || (ny < 0) || (ny >= input.height) {
                            // This is outside the main block
                            continue;
                        }
                    }

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

    fn steps_from(input: &Input, start: (isize, isize), end: (isize, isize)) -> usize {
        // We're going to do let 'current' and 'next' swap between these maps.
        let mut set1: HashSet<(isize, isize)> = HashSet::new();
        let mut set2: HashSet<(isize, isize)> = HashSet::new();

        let current = &mut set1;
        let next = &mut set2;

        // start position is the only occupied space, initially.
        current.insert(start);  

        let mut done = false;
        let mut steps = 0;

        while !done {
            // clear out occupancy of next before we start
            next.clear();

            // Check to see if we're done.
            if current.contains(&end) {
                break;
            }

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

            steps += 1;
        }

        steps
    }
}

impl<'a> Day for Day21<'a> {
    fn part1(&self) -> Answer {
        let input = Input::read(self.input_filename);

        let occupied = Day21::num_by_steps(64, &input, &input.start, false);

        Answer::Numeric(occupied)
    }

    fn part2(&self) -> Answer {
        let input = Input::read(self.input_filename);
        let answer = Day21::num_by_tile_analysis(26501365, &input);

        Answer::Numeric(answer)
    }
}

#[cfg(test)]
mod test {
    use crate::day21::{Day21, Input, FillFrom, FillAmount};

    #[test]
    fn test_input() {
        let input = Input::read("examples/day21_example1.txt");
        assert_eq!(input.width, 11);
        assert_eq!(input.height, 11);
        assert_eq!(input.start, (5, 5));
        assert!(!input.garden.contains(&(1, 2)));
        assert!(input.garden.contains(&(0, 2)));

        assert_eq!(input.to_left(), 5);
        assert_eq!(input.to_right(), 5);
        assert_eq!(input.to_top(), 5);
        assert_eq!(input.to_bottom(), 5);
    }

    
    #[test]
    fn check_real_input() {
        let input = Input::read("data_aoc2023/day21.txt");
        assert_eq!(input.width, 131);
        assert_eq!(input.height, 131);
        assert_eq!(input.start, (65, 65));

        assert_eq!(input.to_left(), 65);
        assert_eq!(input.to_right(), 65);
        assert_eq!(input.to_top(), 65);
        assert_eq!(input.to_bottom(), 65);
    }

    #[test]
    fn test_num_by_steps() {
        let input = Input::read("examples/day21_example1.txt");

        let cases= [
            (6, 16), 
            (10, 50), 
            (50, 1594),
            (100, 6536), 
            (500, 167004), 
            //(1000, 668697), 
            // (5000, 16733044)
            ];
        for (steps, expected) in &cases {
            let reach = Day21::num_by_steps(*steps, &input, &input.start, true);
            assert_eq!(reach, *expected);
        }
    }

    #[test]
    fn test_steps_from() {
        let input = Input::read("examples/day21_example1.txt");

        assert_eq!(Day21::steps_from(&input, (5, 5), (5, 4)), 1);
        assert_eq!(Day21::steps_from(&input, (5, 5), (5, 3)), 2);
        assert_eq!(Day21::steps_from(&input, (5, 5), (7, 4)), 5);
        assert_eq!(Day21::steps_from(&input, (5, 5), (0, 5)), 7);
        assert_eq!(Day21::steps_from(&input, (5, 5), (-1, 5)), 8);
    }


    #[test]
    fn test_steps_from_real() {
        let input = Input::read("data_aoc2023/day21.txt");

        // From start to N, S, E, W on adjacent tile.
        assert_eq!(Day21::steps_from(&input, (65, 65), (-1, 65)), 66);
        assert_eq!(Day21::steps_from(&input, (65, 65), (131, 65)), 66);
        assert_eq!(Day21::steps_from(&input, (65, 65), (65, -1)), 66);
        assert_eq!(Day21::steps_from(&input, (65, 65), (65, 131)), 66);

        // from start to NE, SE, NW, SW on adjacent tile.
        assert_eq!(Day21::steps_from(&input, (65, 65), (-1, -1)), 132);
        assert_eq!(Day21::steps_from(&input, (65, 65), (131, 131)), 132);
        assert_eq!(Day21::steps_from(&input, (65, 65), (131, -1)), 132);
        assert_eq!(Day21::steps_from(&input, (65, 65), (-1, 131)), 132);

        // Horizontal/Vert from edge to next tile's edge.
        assert_eq!(Day21::steps_from(&input, (0, 0), (0, 131)), 131);
        assert_eq!(Day21::steps_from(&input, (0, 0), (131, 0)), 131);

        // Diagonal from corner to opposite corner, adjacent tile.
        assert_eq!(Day21::steps_from(&input, (0, 0), (131, 131)), 262);
        assert_eq!(Day21::steps_from(&input, (0, 130), (131, -1)), 262);
    }    

    #[test]
    fn test_num_of_center() {
        let input = Input::read("data_aoc2023/day21.txt");
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullEven, 1, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullOdd, 1, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::Least, 1, &input), 1);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::NextLeast, 1, &input), 0);

        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullEven, 65, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullOdd, 65, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::Least, 65, &input), 1);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::NextLeast, 65, &input), 0);

        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullEven, 66, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullOdd, 66, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::Least, 66, &input), 1);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::NextLeast, 66, &input), 0);

        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullEven, 131, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullOdd, 131, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::Least, 131, &input), 1);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::NextLeast, 131, &input), 0);

        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullEven, 132, &input), 1);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullOdd, 132, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::Least, 132, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::NextLeast, 132, &input), 0);

        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullEven, 133, &input), 1);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullOdd, 133, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::Least, 133, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::NextLeast, 133, &input), 0);

        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullEven, 26501364, &input), 1);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullOdd, 26501364, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::Least, 26501364, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::NextLeast, 26501364, &input), 0);

        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullEven, 26501365, &input), 1);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::FullOdd, 26501365, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::Least, 26501365, &input), 0);
        assert_eq!(Day21::num_of(&FillFrom::Center, &FillAmount::NextLeast, 26501365, &input), 0);
    }


    #[test]
    fn test_num_of_NESW() {
        let input = Input::read("data_aoc2023/day21.txt");

        for seed in &[FillFrom::North, FillFrom::South, FillFrom::East, FillFrom::West] {
            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 1, &input), 0);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 65, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 65, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 65, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 65, &input), 0);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 65+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 65+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 65+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 65+1, &input), 0);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 65+131, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 65+131, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 65+131, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 65+131, &input), 0);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 65+131+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 65+131+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 65+131+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 65+131+1, &input), 1);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 65+2*131, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 65+2*131, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 65+2*131, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 65+2*131, &input), 1);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 65+2*131+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 65+2*131+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 65+2*131+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 65+2*131+1, &input), 1);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 65+3*131, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 65+3*131, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 65+3*131, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 65+3*131, &input), 1);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 65+3*131+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 65+3*131+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 65+3*131+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 65+3*131+1, &input), 1);

            // assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 65+4*131, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 65+4*131, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 65+4*131, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 65+4*131, &input), 1);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 65+4*131+1, &input), 2);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 65+4*131+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 65+4*131+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 65+4*131+1, &input), 1);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 26501364, &input), 101149);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 26501364, &input), 101149);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 26501364, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 26501364, &input), 1);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 26501365, &input), 101150);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 26501365, &input), 101149);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 26501365, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 26501365, &input), 1);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 26501365, &input), 101150);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 26501365, &input), 101149);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 26501365, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 26501365, &input), 1);
        }
    }

    #[test]
    fn test_num_of_Diags() {
        let input = Input::read("data_aoc2023/day21.txt");

        for seed in &[FillFrom::NorthEast, FillFrom::SouthEast, FillFrom::NorthWest, FillFrom::SouthWest] {
            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 1, &input), 0);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 131, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 131, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 131, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 131, &input), 0);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 131+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 131+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 131+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 131+1, &input), 0);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 2*131, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 2*131, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 2*131, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 2*131, &input), 0);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 2*131+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 2*131+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 2*131+1, &input), 2);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 2*131+1, &input), 1);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 3*131, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 3*131, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 3*131, &input), 2);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 3*131, &input), 1);

            // assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 3*131+1, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 3*131+1, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 3*131+1, &input), 3);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 3*131+1, &input), 2);
            
            // assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 4*131, &input), 1);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 4*131, &input), 0);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 4*131, &input), 3);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 4*131, &input), 2);

            assert_eq!(Day21::num_of(seed, &FillAmount::FullEven, 26501365, &input), 10231120201);
            assert_eq!(Day21::num_of(seed, &FillAmount::FullOdd, 26501365, &input), 10231221350);
            assert_eq!(Day21::num_of(seed, &FillAmount::Least, 26501365, &input), 202300);
            assert_eq!(Day21::num_of(seed, &FillAmount::NextLeast, 26501365, &input), 202299);
        }
    }

    #[test]
    fn test_count_for_center() {
        let input = Input::read("data_aoc2023/day21.txt");
        assert_eq!(Day21::count_for(&FillFrom::Center, &FillAmount::Least, 1, &input), 4);
        assert_eq!(Day21::count_for(&FillFrom::Center, &FillAmount::Least, 65, &input), 3821);
        assert_eq!(Day21::count_for(&FillFrom::Center, &FillAmount::Least, 66, &input), 3984);
        assert_eq!(Day21::count_for(&FillFrom::Center, &FillAmount::Least, 131, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::Center, &FillAmount::FullEven, 132, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::Center, &FillAmount::FullEven, 133, &input), 7556);
    }

    #[test]
    fn test_count_for_NSEW() {
        let input = Input::read("data_aoc2023/day21.txt");

        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::Least, 66, &input), 1);
        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::Least, 67, &input), 3);
        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::Least, 66+129, &input), 5656);
        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::Least, 66+130, &input), 5692);

        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::NextLeast, 66+131, &input), 5786);
        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::NextLeast, 66+2*131-1, &input), 7602);

        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::FullEven, 3*131+65, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::FullEven, 3*131+65+1, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::FullEven, 23*131+65, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::FullEven, 23*131+65+1, &input), 7602);

        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::FullOdd, 3*131+65, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::FullOdd, 3*131+65+1, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::FullOdd, 23*131+65, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::North, &FillAmount::FullOdd, 23*131+65+1, &input), 7556);

        // ------------------------------------------------------------------------------------------------

        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::Least, 66, &input), 1);
        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::Least, 67, &input), 1);
        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::Least, 66+129, &input), 5573);
        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::Least, 66+130, &input), 5667);

        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::NextLeast, 66+131, &input), 5705);
        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::NextLeast, 66+2*131-1, &input), 7556);

        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::FullEven, 3*131+65, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::FullEven, 3*131+65+1, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::FullEven, 23*131+65, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::FullEven, 23*131+65+1, &input), 7556);

        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::FullOdd, 3*131+65, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::FullOdd, 3*131+65+1, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::FullOdd, 23*131+65, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::East, &FillAmount::FullOdd, 23*131+65+1, &input), 7602);

        // -----------------------------------------------------------------------------------------------

        assert_eq!(Day21::count_for(&FillFrom::South, &FillAmount::FullEven, 3*131+65, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::South, &FillAmount::FullEven, 3*131+65+1, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::South, &FillAmount::FullEven, 23*131+65, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::South, &FillAmount::FullEven, 23*131+65+1, &input), 7556);

        assert_eq!(Day21::count_for(&FillFrom::South, &FillAmount::FullOdd, 3*131+65, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::South, &FillAmount::FullOdd, 3*131+65+1, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::South, &FillAmount::FullOdd, 23*131+65, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::South, &FillAmount::FullOdd, 23*131+65+1, &input), 7602);

        // -----------------------------------------------------------------------------------------------

        assert_eq!(Day21::count_for(&FillFrom::West, &FillAmount::FullEven, 3*131+65, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::West, &FillAmount::FullEven, 3*131+65+1, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::West, &FillAmount::FullEven, 23*131+65, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::West, &FillAmount::FullEven, 23*131+65+1, &input), 7602);

        assert_eq!(Day21::count_for(&FillFrom::West, &FillAmount::FullOdd, 3*131+65, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::West, &FillAmount::FullOdd, 3*131+65+1, &input), 7556);
        assert_eq!(Day21::count_for(&FillFrom::West, &FillAmount::FullOdd, 23*131+65, &input), 7602);
        assert_eq!(Day21::count_for(&FillFrom::West, &FillAmount::FullOdd, 23*131+65+1, &input), 7556);

    }

    #[test]
    fn test_regression() {
        // let input = Input::read("data_aoc2023/day21.txt");
        let input = Input::read("examples/day21_example1.txt");

        let a = Day21::num_by_steps(input.start.0 as usize, &input, &input.start, true);
        let b = Day21::num_by_steps((input.start.0+6*input.width) as usize, &input, &input.start, true);
        let c = Day21::num_by_steps((input.start.0+8*input.width) as usize, &input, &input.start, true);

        println!("a: {a}, b: {b}, c:{c}");
    }

    #[test]
    fn test_num_by_tile_analysis() {
        let input = Input::read("data_aoc2023/day21.txt");

        // 465262978619852 is too low.
        // 620348631940729 is too high.
        let answer = Day21::num_by_tile_analysis(26501365, &input);
        assert!(answer > 465262978619852);
        assert!(answer < 620348631940729);
        assert_eq!(answer, 620348631910321);
    }

    #[test]
    fn test_by_analysis() {
        let input = Input::read("examples/day21_example1.txt");

        let n = Day21::num_by_tile_analysis(5000, &input);
    }

    #[test]
    fn test_by_analysis2() {
        let input = Input::read("data_aoc2023/day21.txt");

        let n = Day21::num_by_tile_analysis(26501365, &input);
    }
}
