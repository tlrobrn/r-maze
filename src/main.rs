#![feature(core)]
mod maze;
use maze::Maze;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let (width, height) = match args.as_slice() {
        [_, ref width, ref height] => (width.parse(), height.parse()),
        _ => (Ok(5), Ok(5)),
    };

    match (width, height) {
        (Ok(width), Ok(height)) => println!("{}", Maze::new(width, height)),
        _ => println!("Usage $ r-maze WIDTH HEIGHT"),
    }
}
