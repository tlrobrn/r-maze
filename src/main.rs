mod maze;
use maze::Maze;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (width, height) = if args.len() == 3 {
        (args[1].parse(), args[2].parse())
    } else {
        (Ok(5), Ok(5))
    };

    match (width, height) {
        (Ok(width), Ok(height)) => println!("{}", Maze::new(width, height)),
        _ => println!("Usage $ r-maze WIDTH HEIGHT"),
    }
}
