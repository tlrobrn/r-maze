extern crate rand;
extern crate disjoint_sets;

use self::rand::{thread_rng, Rng};
use self::disjoint_sets::DisjointSets;
use std::fmt::{self, Formatter, Display};

pub struct Maze { 
    width: usize,
    height: usize,
    walls: Vec<bool>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        let size = width * height;

        let mut maze = Maze {
            width: width,
            height: height,
            walls: vec![false; size],
        };


        let mut dsets = DisjointSets::new(size);
        let (east, south) = maze.initialize_walls();

        for i in 0..east.len() {
            maze.try_wall(&mut dsets, east[i], true);
            maze.try_wall(&mut dsets, south[i], false);
        }

        maze
    }

    fn initialize_walls(&mut self) -> (Vec<usize>, Vec<usize>) {
        let mut east = vec![];
        let mut south = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                let i = x + y * self.height;
                let is_wall = (x % 2) == 1 || (y % 2) == 1;

                self.walls[i] = is_wall;
                if !is_wall {
                    east.push(i);
                    south.push(i);
                }
            }
        }

        let mut rng = thread_rng();
        rng.shuffle(&mut east);
        rng.shuffle(&mut south);

        (east, south)
    }


    fn try_wall(&mut self, mut dsets: &mut DisjointSets, index: usize, is_east: bool) {
        let step = match self.coordinates(index) {
            (x, _) if is_east && (self.width > 1 && x < self.width - 2) => { 1 },
            (_, y) if (self.height > 1 && y < self.height - 2) => { self.width },
            _ => { return },
        };

        if dsets.find_root(index) != dsets.find_root(index + step * 2) {
            self.walls[index + step] = false;
            dsets.set_union(index, index + step);
            dsets.set_union(index, index + step * 2);
        }
    }

    fn coordinates(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for _ in 0..(self.width + 1) { let _ = write!(f, "X"); }
        for y in 0..self.height {
            let _ = write!(f, "X\nX");
            for x in 0..self.width {
                let _ = write!(f, "{}", if self.walls[x + y * self.width] { "X" } else { " " });
            }
        }
        let _ = write!(f, "X\nX");
        for _ in 0..(self.width + 1) { let _ = write!(f, "X"); }
        write!(f, "\n")
    }
}

