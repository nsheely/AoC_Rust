// Day 16: The Floor Will Be Lava
//
// Part 1: Trace light beam through mirrors and count energized tiles
// Part 2: Find optimal entry point for maximum energized tiles

use rayon::prelude::*;

const NORTH: u8 = 1;
const EAST: u8 = 2;
const SOUTH: u8 = 4;
const WEST: u8 = 8;

#[aoc(day16, part1)]
pub fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    energize_count(&grid, 0, 0, EAST)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .filter(|v: &Vec<char>| !v.is_empty())
        .collect()
}

fn energize_count(grid: &Vec<Vec<char>>, start_x: i64, start_y: i64, start_dir: u8) -> usize {
    let (width, height) = (grid[0].len(), grid.len());
    let mut total = 0;
    let mut light = vec![vec![0u8; width]; height];
    let mut queue = vec![(start_y, start_x, start_dir)];

    while let Some((y, x, dir_bit)) = queue.pop() {
        if x < 0 || y < 0 || x >= width as i64 || y >= height as i64 {
            continue;
        }
        let (yi, xi) = (y as usize, x as usize);

        if light[yi][xi] & dir_bit != 0 {
            continue;
        }

        light[yi][xi] |= dir_bit;
        if light[yi][xi] == dir_bit {
            total += 1;
        }

        let cell = grid[yi][xi];
        match dir_bit {
            NORTH => match cell {
                '.' | '|' => queue.push((y - 1, x, NORTH)),
                '/' => queue.push((y, x + 1, EAST)),
                '\\' => queue.push((y, x - 1, WEST)),
                '-' => {
                    queue.push((y, x + 1, EAST));
                    queue.push((y, x - 1, WEST));
                }
                _ => (),
            },
            SOUTH => match cell {
                '.' | '|' => queue.push((y + 1, x, SOUTH)),
                '/' => queue.push((y, x - 1, WEST)),
                '\\' => queue.push((y, x + 1, EAST)),
                '-' => {
                    queue.push((y, x + 1, EAST));
                    queue.push((y, x - 1, WEST));
                }
                _ => (),
            },
            EAST => match cell {
                '.' | '-' => queue.push((y, x + 1, EAST)),
                '/' => queue.push((y - 1, x, NORTH)),
                '\\' => queue.push((y + 1, x, SOUTH)),
                '|' => {
                    queue.push((y - 1, x, NORTH));
                    queue.push((y + 1, x, SOUTH));
                }
                _ => (),
            },
            WEST => match cell {
                '.' | '-' => queue.push((y, x - 1, WEST)),
                '/' => queue.push((y + 1, x, SOUTH)),
                '\\' => queue.push((y - 1, x, NORTH)),
                '|' => {
                    queue.push((y - 1, x, NORTH));
                    queue.push((y + 1, x, SOUTH));
                }
                _ => (),
            },
            _ => (),
        }
    }

    total
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    let (width, height) = (grid[0].len() as i64, grid.len() as i64);

    let mut edge_positions = Vec::new();
    for x in 0..width {
        edge_positions.push((x, 0, SOUTH));
        edge_positions.push((x, height - 1, NORTH));
    }
    for y in 0..height {
        edge_positions.push((0, y, EAST));
        edge_positions.push((width - 1, y, WEST));
    }

    edge_positions
        .par_iter()
        .map(|&(x, y, dir)| energize_count(&grid, x, y, dir))
        .max()
        .unwrap_or(0)
}
