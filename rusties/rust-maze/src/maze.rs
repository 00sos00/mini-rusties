use crate::maze_block::{Block, BlockType};
use rand::{thread_rng, Rng};
use std::ops::DerefMut;

pub type Grid = Vec<Vec<Block>>;

pub struct Maze {
    pub width: i32,
    pub height: i32,
    pub grid: Grid,
}

impl Maze {
    pub fn new(width: i32, height: i32) -> Self {
        assert!(width > 0, "Expected positive width, got {width}");
        assert!(height > 0, "Expected positive height, got {height}");
        assert!(width % 2 != 0, "Expected even width, got {width}");
        assert!(height % 2 != 0, "Expected even height, got {height}");

        Self {
            width,
            height,
            grid: vec![],
        }
    }

    pub fn get_block(&self, x: i32, y: i32) -> Option<&Block> {
        self.grid.get(y as usize).and_then(|v| v.get(x as usize))
    }

    pub fn random_unvisited_empty_block(&self) -> &Block {
        let mut rng = thread_rng();

        let x = rng.gen_range(1..self.width / 2) * 2 + 1;
        let y = rng.gen_range(1..self.height / 2) * 2 + 1;

        let block = self.get_block(x, y).unwrap();

        if let BlockType::Empty { visited } = block.typ.borrow_mut().deref_mut() {
            *visited = true;
            block
        } else {
            unreachable!("Wall block found at ({x}, {y}), expected empty block");
        }
    }

    pub fn generate_grid(mut self) -> Self {
        for y in 0..self.height {
            let mut row = vec![];

            for x in 0..self.width {
                if x % 2 == 0 || y % 2 == 0 {
                    row.push(Block::new(x, y, BlockType::Wall));
                } else {
                    row.push(Block::new(x, y, BlockType::Empty { visited: false }));
                }
            }

            self.grid.push(row);
        }

        self
    }

    pub fn generate_maze(self) -> Self {
        let mut stack = vec![];
        let mut num_visited_blocks = 0;
        let mut current_block = self.random_unvisited_empty_block();

        num_visited_blocks += 1;

        let num_empty_blocks = (self.width - 1) * (self.height - 1) / 4;

        while num_visited_blocks < num_empty_blocks {
            if let Some(random_neighbour) = current_block.random_unvisited_neighbour(&self) {
                if let BlockType::Empty { visited } = random_neighbour.typ.borrow_mut().deref_mut()
                {
                    stack.push(current_block);

                    current_block.remove_wall_between(random_neighbour, &self);
                    current_block = random_neighbour;

                    *visited = true;
                    num_visited_blocks += 1;
                }
            } else if let Some(popped_block) = stack.pop() {
                current_block = popped_block;
            }
        }

        self
    }
}
