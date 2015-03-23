# Maze

Randomly generates acyclic 2D mazes.

## Build

```
$ cargo build --release
```

## Usage

```
$ target/release/r-maze WIDTH HEIGHT
```

Default `WIDTH` and `HEIGHT` are 5.

_*NOTE*: `WIDTH` and `HEIGHT` do not include the spaces necessary for walls.  Therefore, the output for a 2 x 2 maze actually takes up 5 x 5 spaces since it needs to display the border edges and the walls in between the spaces._
