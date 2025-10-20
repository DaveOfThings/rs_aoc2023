use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}};

use crate::day::{Day, Answer};
use regex::Regex;

#[derive(Debug, Eq, Hash, PartialEq)]
struct V3D {
    x: isize,
    y: isize,
    z: isize,
}

impl V3D {
    const UNIT: V3D = V3D { x:1, y:1, z:1 };

    fn new(x: isize, y: isize, z: isize) -> V3D {
        V3D { x, y, z }
    }

    fn minus(&self, other: &V3D) -> V3D {
        V3D {x: self.x-other.x, y: self.y-other.y, z: self.z-other.z }
    }
    
    fn plus(&self, other: &V3D) -> V3D {
        V3D {x: self.x+other.x, y: self.y+other.y, z: self.z+other.z }
    }
}

#[derive(Debug)]
struct Block {
    initial_pos: V3D,
    size: V3D,
}

impl Block {
    fn new_p1_p2(p1: V3D, p2: V3D) -> Block {
        let size = p2.minus(&p1).plus(&V3D::UNIT);
        Block { initial_pos: p1, size: size}
    }
}

struct Input {
    blocks: Vec<Block>
}

impl Input {
    fn read(filename: &str) -> Input {
        let mut blocks = Vec::new();

        let f = File::open(filename).unwrap();
        let reader = BufReader::new(f);

        let line_re = Regex::new("([0-9]+),([0-9]+),([0-9]+)~([0-9]+),([0-9]+),([0-9]+)").unwrap();

        for line in reader.lines() {
            if let Some(caps) = line_re.captures(&line.unwrap()) {
                let p1 = V3D::new(caps[1].parse::<isize>().unwrap(),
                                            caps[2].parse::<isize>().unwrap(),
                                            caps[3].parse::<isize>().unwrap());
                let p2 = V3D::new(caps[4].parse::<isize>().unwrap(),
                                            caps[5].parse::<isize>().unwrap(),
                                            caps[6].parse::<isize>().unwrap());

                blocks.push(Block::new_p1_p2(p1, p2) );
            }
        }

        Input { blocks }
    }
}

struct Stack<'a> {
    input: &'a Input,

    // Note: blocks are identified by their index into input.blocks: usize

    // Height of each existing column
    // top_block: HashMap<(isize, isize), (usize, usize)>,  // (x, y) -> (block_id, height)

    // Blocks occupying a given column
    // blocks_over: HashMap<(isize, isize), Vec<usize>>,  // Which blocks cover this coordinate.

    // support height of each block in the stack
    // A block's min Z will be this + 1.  Ground is Z=0.  The first block to fall will have base_height=0 so it's min Z is 1.
    base_height: Vec<isize>,              

    /*
    // Blocks immediately below a given one.
    below: HashMap<usize, Vec<usize>>,

    // Blocks immediately above a given one.
    above: HashMap<usize, Vec<usize>>,
    */

    // Which blocks occupy each coordinate V3D -> block id
    occupied: HashMap<V3D, usize>,
}

impl <'a> Stack<'a> {
    fn new(input: &'a Input) -> Stack<'a> {
        Stack { input, base_height: Vec::new(), occupied: HashMap::new() }
    }

    fn drop(&mut self, block_id: usize) {
        let block = &self.input.blocks[block_id];

        // Check all the (x,y) locations this block will fall on and note the max
        // Z coordinate of all of them.  Also note the supporting blocks at that Z.

        let mut supporters: HashSet<usize> = HashSet::new();


        println!("Dropping {block_id} from {:?}", &block.initial_pos);

        // Reduce support_z until it reaches 0 or we find occupied blocks
        let mut support_z: isize = block.initial_pos.z;
        let mut supporters: HashSet<usize> = HashSet::new();

        while support_z > 0 && supporters.is_empty() {
            // drop support_z by one
            support_z -= 1;

            // check occupancy for supporters
            for x in block.initial_pos.x..block.initial_pos.x+block.size.x {
                for y in block.initial_pos.y..block.initial_pos.y+block.size.y {
                    // println!("  Considering location {x}, {y}");
                    if let Some(supporter) = self.occupied.get(&V3D::new(x, y, support_z)) {
                        // Found a supporter
                        supporters.insert(*supporter);
                    }
                }
            }
        }

        // Record the support Z for each block.  (The block's Z is this +1.)
        println!("Found support at z={support_z} by {:?}", supporters);
        self.base_height[block_id] = support_z;

        // Record this block's occupancy
        for x in block.initial_pos.x..block.initial_pos.x+block.size.x {
            for y in block.initial_pos.y..block.initial_pos.y+block.size.y {
                for z in support_z+1..support_z+block.size.z+1 {
                    // println!("Block {block_id} occupying ({x}, {y}, {z})");
                    assert!(!self.occupied.contains_key(&V3D::new(x, y, z)));
                    self.occupied.insert(V3D::new(x, y, z), block_id);
                }
            }
        }        
    }

    // The blocks aren't sorted by Z in the input so we can't run the simulation in the input order.
    // I can think of two approaches:
    //   1) Initialize all blocks in the position from the snapshot.
    //      Then go through all of them, moving them down until they hit another block.
    //      Repeat that until no block moves.
    //   2) Drop block sequentially in order of their Z coordinate in the input.
    //      I think this will guarantee that each block only needs to drop once.
    //
    // Trying 2 as it seems less disruptive.

    fn run(&mut self) {
        self.occupied.clear();

        // Initialize base_height vector with initial Z positions - 1
        self.base_height.clear();
        for n in 0..self.input.blocks.len() {
            self.base_height.push(self.input.blocks.get(n).unwrap().initial_pos.z - 1);
        }

        // Go through all blocks, in Z order from 0 to max, dropping each one.
        let mut block_ids = self.input.blocks.iter()
            .enumerate()
            .map(|(block_id, block)| {
                (block.initial_pos.z, block_id)
            })
            .collect::<Vec<(isize, usize)>>();
        block_ids.sort();
        for (_height, block_id) in &block_ids {
            self.drop(*block_id);
        }
    }

    fn is_disintegrateable(&self, block_id: usize) -> bool {
        let block = &self.input.blocks[block_id];
        // Make a set of all blocks above this one.
        let mut blocks_above: HashSet<usize> = HashSet::new();
        let x0 = block.initial_pos.x;
        let y0 = block.initial_pos.y;
        let z0 = self.base_height.get(block_id).unwrap()+1;

        for dx in 0..block.size.x {
            for dy in 0..block.size.y {
                let dz = block.size.z;
                if let Some(block_above) = self.occupied.get(&V3D::new(x0+dx, y0+dy, z0+dz)) {
                    assert_ne!(*block_above, block_id);
                    blocks_above.insert(*block_above);
                }
            }
        }
        println!("Blocks above {block_id}: {blocks_above:?}");

        // For each block above this one, count its supporters.  If it's just one, this 
        // block is not disintegrateable.
        for supported_block_id in &blocks_above {
            let block = &self.input.blocks[*supported_block_id];
            let mut blocks_below: HashSet<usize> = HashSet::new();
            let x0 = block.initial_pos.x;
            let y0 = block.initial_pos.y;
            let z0 = self.base_height.get(*supported_block_id).unwrap()+1;

            for dx in 0..block.size.x {
                for dy in 0..block.size.y {
                    
                    if let Some(block_below) = self.occupied.get(&V3D::new(x0+dx, y0+dy, z0-1)) {
                        blocks_below.insert(*block_below);
                    }
                }
            }
            println!("  Blocks below {supported_block_id}: {blocks_below:?}");
            assert!(blocks_below.contains(&block_id));

            if blocks_below.len() == 1 {
                // We can't disintegrate this block.  There's a supported block with only this block for support.
                println!("Can't disintegrate {block_id}.  The block {supported_block_id} depends on it.");
                return false;
            }
        }

        // We didn't find any reason this can't be disintegrated.
        true
    }

    fn num_disintegrateable(&self) -> usize {
        // TODO : Use an iter.
        let mut count = 0;

        for block_id in 0..self.input.blocks.len() {
            if self.is_disintegrateable(block_id) {
                count += 1;
            }
        }

        count
    }
}

pub struct Day22<'a> {
    input_filename: &'a str,
}

impl<'a> Day22<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { input_filename: filename }
    }
}

impl<'a> Day for Day22<'a> {
    fn part1(&self) -> Answer {
        let input = Input::read(self.input_filename);
        let mut stack = Stack::new(&input);
        stack.run();

        Answer::Numeric(stack.num_disintegrateable())
    }

    fn part2(&self) -> Answer {
        Answer::None
    }
}

#[cfg(test)]
mod test {
    use crate::day22::{Day22, Input, Block, Stack};
    use crate::{Answer, Day};

    #[test]
    fn test_read() {
        let input = Input::read("examples/day22_example1.txt");

        assert_eq!(input.blocks.len(), 7);
    }

    #[test]
    fn test_dimensions() {
        // Confirm that a block can only have one dimension that is >1.
        let input = Input::read("examples/day22_example1.txt");
        let large_ones: Vec<&Block> = input.blocks.iter()
            .filter(|b| { 
                let mut large_dims = 0; 
                if (b.size.x > 1) || (b.size.y > 1) || (b.size.z > 1) {
                    large_dims += 1;
                }
                large_dims > 1
            })
            .collect();
        
        large_ones.iter()
            .for_each(|b| {
                println!("Big Block: {b:?} ")
            });

        assert_eq!(large_ones.len(), 0);
    }

    #[test]
    fn test_stacking() {
        let input = Input::read("examples/day22_example1.txt");
        let mut stack = Stack::new(&input);
        stack.run();

        assert_eq!(stack.base_height[0], 0);  // A at 1
        assert_eq!(stack.base_height[1], 1);  // B at 2
        assert_eq!(stack.base_height[2], 1);  // C at 2
        assert_eq!(stack.base_height[3], 2);  // D at 3
        assert_eq!(stack.base_height[4], 2);  // E at 3
        assert_eq!(stack.base_height[5], 3);  // F at 4
        assert_eq!(stack.base_height[6], 4);  // G at 5
    }

    
    #[test]
    fn test_disintegrateable() {
        let input = Input::read("examples/day22_example1.txt");
        let mut stack = Stack::new(&input);
        stack.run();

        assert_eq!(stack.num_disintegrateable(), 5);
    }

        
    #[test]
    fn test_disintegrateable2() {
        let input = Input::read("examples/day22_example2.txt");
        let mut stack = Stack::new(&input);
        stack.run();

        assert_eq!(stack.num_disintegrateable(), 4);
    }

    #[test]
    fn test_part1() {
        let d = Day22::new("examples/day22_example1.txt");

        assert_eq!(d.part1(), Answer::Numeric(5));
    }
    
    #[test]
    fn test_part1_2() {
        let d = Day22::new("data_aoc2023/day22.txt");

        assert_eq!(d.part1(), Answer::Numeric(389));
    }
}
