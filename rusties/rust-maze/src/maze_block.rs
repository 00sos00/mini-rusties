use crate::maze::Maze;
use rand::seq::SliceRandom;
use std::cell::RefCell;

pub enum BlockType {
    Wall,
    Empty { visited: bool },
}

pub struct Block {
    x: i32,
    y: i32,
    pub typ: RefCell<BlockType>,
}

impl Block {
    pub fn new(x: i32, y: i32, typ: BlockType) -> Self {
        Self {
            x,
            y,
            typ: RefCell::new(typ),
        }
    }

    pub fn random_unvisited_neighbour<'a>(&self, maze: &'a Maze) -> Option<&'a Block> {
        let mut neighbours = vec![];

        if let Some(top_neighbour) = maze.get_block(self.x, self.y - 2) {
            if let BlockType::Empty { visited } = *top_neighbour.typ.borrow() {
                if !visited {
                    neighbours.push(top_neighbour);
                }
            }
        }

        if let Some(right_neighbour) = maze.get_block(self.x + 2, self.y) {
            if let BlockType::Empty { visited } = *right_neighbour.typ.borrow() {
                if !visited {
                    neighbours.push(right_neighbour);
                }
            }
        }

        if let Some(bottom_neighbour) = maze.get_block(self.x, self.y + 2) {
            if let BlockType::Empty { visited } = *bottom_neighbour.typ.borrow() {
                if !visited {
                    neighbours.push(bottom_neighbour);
                }
            }
        }

        if let Some(left_neighbour) = maze.get_block(self.x - 2, self.y) {
            if let BlockType::Empty { visited } = *left_neighbour.typ.borrow() {
                if !visited {
                    neighbours.push(left_neighbour);
                }
            }
        }

        neighbours.choose(&mut rand::thread_rng()).copied()
    }

    pub fn remove_wall_between(&self, other_cell: &Block, maze: &Maze) {
        let x_diff = self.x - other_cell.x;
        let y_diff = self.y - other_cell.y;

        if y_diff == 2 {
            let top_wall = maze.get_block(self.x, self.y - 1).unwrap();

            top_wall.typ.replace(BlockType::Empty { visited: false });
        } else if x_diff == -2 {
            let right_wall = maze.get_block(self.x + 1, self.y).unwrap();

            right_wall.typ.replace(BlockType::Empty { visited: false });
        } else if y_diff == -2 {
            let bottom_wall = maze.get_block(self.x, self.y + 1).unwrap();

            bottom_wall.typ.replace(BlockType::Empty { visited: false });
        } else if x_diff == 2 {
            let left_wall = maze.get_block(self.x - 1, self.y).unwrap();

            left_wall.typ.replace(BlockType::Empty { visited: false });
        }
    }
}
