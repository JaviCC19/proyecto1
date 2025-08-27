// maze.rs

use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Maze = Vec<Vec<char>>;

pub fn load_maze(filename: &str) -> Maze {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

pub fn is_wall(x: f32, y: f32, maze: &Maze, block_size: usize) -> bool {
    let i = (x as usize) / block_size;
    let j = (y as usize) / block_size;

    if j >= maze.len() || i >= maze[0].len() {
        return true; // fuera del mapa se considera pared
    }

    maze[j][i] != ' ' // cualquier cosa que no sea espacio es pared
}
