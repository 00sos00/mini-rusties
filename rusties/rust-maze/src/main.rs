mod maze;
mod maze_block;

use maze::Maze;
use maze_block::BlockType;

use colored::*;

fn print_maze_grid_row(data: &[u8]) {
    match data {
        [data @ ..] => {
            for num in data {
                if *num == 1 {
                    print!(" {}", "■".yellow());
                } else {
                    print!("  ");
                }
            }

            println!();
        }
    };
}

fn main() {
    rand::thread_rng();

    let maze_width = 9;
    let maze_height = 9;

    let maze = Maze::new(maze_width, maze_height)
        .generate_grid()
        .generate_maze();

    for row in &maze.grid {
        let mut trow = vec![];

        for block in row {
            match *block.typ.borrow() {
                BlockType::Wall => trow.push(1),
                BlockType::Empty { .. } => trow.push(0),
            }
        }

        print_maze_grid_row(&trow);
    }
}
