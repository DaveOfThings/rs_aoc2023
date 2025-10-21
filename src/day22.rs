use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}};

use crate::day::{Day, Answer};
use regex::Regex;

// A basic 3D vector with isize components.
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

// A block's representation from the input file.
#[derive(Debug)]
struct Block {
    initial_pos: V3D,
    size: V3D,
}

impl Block {
    fn new(p1: V3D, p2: V3D) -> Block {
        let size = p2.minus(&p1).plus(&V3D::UNIT);
        Block { initial_pos: p1, size: size}
    }
}

// The puzzle input 
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

                blocks.push(Block::new(p1, p2) );
            }
        }

        Input { blocks }
    }
}

// A stack of blocks.  This represents the state of the puzzle after all blocks have come to rest.
struct Stack<'a> {
    input: &'a Input,

    // Note: blocks are identified by their index into input.blocks: usize

    // support height of each block in the stack
    // A block's min Z will be this + 1.  Ground is Z=0.  The first block to fall will have base_height=0 so it's min Z is 1.
    base_height: Vec<isize>,              

    // Which blocks occupy each coordinate V3D -> block id
    occupied: HashMap<V3D, usize>,

    // Which blocks support a given one.
    // supports[block_id] -> a vector of block ids that support this one.
    supports: Vec<Vec<usize>>,
}

impl <'a> Stack<'a> {
    fn new(input: &'a Input) -> Stack<'a> {
        // Create the stack with empty base_height, occupied and supports components.
        let mut stack = Stack { input, base_height: Vec::new(), occupied: HashMap::new(), supports: Vec::new() };

        // Run the "falling" process to allow all blocks to settle into supported positions.
        // This sets the base_height and occupied components of Stack.
        stack.run();

        // Analyze which blocks are supporting each block.
        // This sets the .supports component of Stack
        stack.gen_supports();

        // Voila, an initialized stack.
        stack
    }

    // Move one block into its settled position.
    // In order to settle the whole stack properly, blocks need to be dropped in order
    // from lowest to highest.
    fn drop(&mut self, block_id: usize) {
        let block = &self.input.blocks[block_id];

        // Start support_z at the initial height and reduce it until it reaches 0 or we find occupied blocks
        let mut support_z: isize = block.initial_pos.z;
        let mut supporters: usize = 0;

        while support_z > 0 && (supporters == 0) {
            // drop support_z by one
            support_z -= 1;

            // check occupancy for supporters
            for x in block.initial_pos.x..block.initial_pos.x+block.size.x {
                for y in block.initial_pos.y..block.initial_pos.y+block.size.y {
                    if self.occupied.contains_key(&V3D::new(x, y, support_z)) {
                        // Found a supporter
                        supporters += 1;
                    }
                }
            }
        }

        // Record the support Z for each block.  (The block's Z is this +1.)
        // println!("Found support at z={support_z} by {:?}", supporters);
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

    fn run(&mut self) {
        self.occupied.clear();

        // Initialize base_height vector with initial Z positions - 1
        self.base_height.clear();
        for n in 0..self.input.blocks.len() {
            self.base_height.push(self.input.blocks.get(n).unwrap().initial_pos.z - 1);
        }

        // Sort blocks by initial z, lowest to highest, creating a vector of (z, block_id)
        let mut block_ids = self.input.blocks.iter()
            .enumerate()
            .map(|(block_id, block)| {
                (block.initial_pos.z, block_id)
            })
            .collect::<Vec<(isize, usize)>>();
        block_ids.sort();

        // Go through all blocks, in Z order from 0 to max, dropping each one.
        for (_height, block_id) in &block_ids {
            self.drop(*block_id);
        }
    }

    // Find and record all the blocks supporting each block.
    // self.supports[supported block id: usize] -> Vec<supporting block id: usize>
    fn gen_supports(&mut self) {
        // For each block, generate a vector of blocks it rests on.
        for block_id in 0..self.input.blocks.len() {
            let block = &self.input.blocks[block_id];

            let mut blocks_below: HashSet<usize> = HashSet::new();
            let x0 = block.initial_pos.x;
            let y0 = block.initial_pos.y;
            let z0 = self.base_height.get(block_id).unwrap()+1;

            for dx in 0..block.size.x {
                for dy in 0..block.size.y {
                    if let Some(block_below) = self.occupied.get(&V3D::new(x0+dx, y0+dy, z0-1)) {
                        blocks_below.insert(*block_below);
                    }
                }
            }

            // Convert set to Vec and store it.
            self.supports.push(blocks_below.iter()
                .map(|n| { *n })
                .collect());

            // println!("Block {block_id} supported by {:?}", self.supports[block_id]);
        }
    }

    fn num_disintegrateable(&self) -> usize {
        // Start a set of indestructible blocks.
        let mut id_set: HashSet<usize> = HashSet::new();

        // Look at all support sets.  Wherever there's a set of just one, that one is indestructible.
        for support_set in &self.supports {
            if support_set.len() == 1 {
                id_set.insert(support_set[0]);
            }
        }
    
        // number of blocks minus number of indestructible blocks = number of disintegrateable blocks.
        self.input.blocks.len() - id_set.len()
    }

    fn fall_set_size(&self, block_id: usize) -> usize {
        let mut would_fall: HashSet<usize> = HashSet::new();
        would_fall.insert(block_id);
        // println!("What happens if {block_id} removed?");

        // Iterate until no blocks fall any more.
        let mut falls = true;
        while falls {
            // Will reset to true and keep the loop going if any new falls found on this iteration.
            falls = false;  

            for test_block in 0..self.input.blocks.len() {
                // We already decided this block would fall, skip it.
                if would_fall.contains(&test_block) { continue; }

                // Test block is on the ground, it can't fall.
                if self.base_height[test_block] == 0 { continue; }

                // A block is supported if one of its supports is not in the would_fall set.
                let supported = self.supports[test_block].iter()
                    .map(|supporter| { 
                        // println!("Is supporter {supporter} still ok? {}", !would_fall.contains(supporter) );
                        !would_fall.contains(supporter)
                    })
                    .fold(false, |accum, supported| { accum || supported });

                // println!("  {test_block} supported? {supported}");

                // If the test block is unsupported, record it as fallen
                if !supported {
                    would_fall.insert(test_block);
                    falls = true;
                }
            }
        }

        // println!("Removing {block_id}, these fall: {would_fall:?}");

        // The falling has settled, return the size of the would_fall hash set minus 1.
        // (Minus 1 is because we seeded would_fall with block_id but we don't want to count that one)
        would_fall.len() - 1
        
    }

    // Add up all the fall set sizes over all the blocks.
    fn total_would_fall(&self) -> usize {
        // Start from the top of the stack, create sets of blocks that would fall if
        // a given block would fall.  For a given block, determine all directly supported
        // block ids, then get the union of their sets.
        let mut total = 0;

        for block_id in 0..self.input.blocks.len() {
            total += self.fall_set_size(block_id);
        }

        total
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
        let stack = Stack::new(&input);

        Answer::Numeric(stack.num_disintegrateable())
    }

    fn part2(&self) -> Answer {
        let input = Input::read(self.input_filename);
        let stack = Stack::new(&input);

        Answer::Numeric(stack.total_would_fall())
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
        let stack = Stack::new(&input);

        assert_eq!(stack.base_height[0], 0);  // A at 1
        assert_eq!(stack.base_height[1], 1);  // B at 2
        assert_eq!(stack.base_height[2], 1);  // C at 2
        assert_eq!(stack.base_height[3], 2);  // D at 3
        assert_eq!(stack.base_height[4], 2);  // E at 3
        assert_eq!(stack.base_height[5], 3);  // F at 4
        assert_eq!(stack.base_height[6], 4);  // G at 5
    }

    #[test]
    fn test_disintegrateable_a() {
        let input = Input::read("examples/day22_example1.txt");
        let stack = Stack::new(&input);

        assert_eq!(stack.num_disintegrateable(), 5);
    }
        
    #[test]
    fn test_disintegrateable_b() {
        let input = Input::read("examples/day22_example2.txt");
        let stack = Stack::new(&input);

        assert_eq!(stack.num_disintegrateable(), 4);
    }

    #[test]
    fn test_fall_total() {
        let input = Input::read("examples/day22_example1.txt");
        let stack = Stack::new(&input);

        assert_eq!(stack.total_would_fall(), 7);
    }

    
    #[test]
    fn test_fall_total2() {
        let input = Input::read("data_aoc2023/day22.txt");
        let stack = Stack::new(&input);

        assert_eq!(stack.total_would_fall(), 70609);
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
