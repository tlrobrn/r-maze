mod maze;
use maze::Maze;

fn main() {
    let maze = Maze::new(99,49);
    println!("{}", maze);
}
