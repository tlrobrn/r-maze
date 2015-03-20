mod maze;
use maze::Maze;

fn main() {
    let maze = Maze::new(5,5);
    println!("{}", maze);
}
