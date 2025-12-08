#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    pub fn rotate(self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }

    pub fn step(self, pos: usize, line_len: usize, grid_len: usize) -> Option<usize> {
        match self {
            Dir::N => (pos >= line_len).then(|| pos - line_len),
            Dir::E => ((pos + 1) % line_len != 0).then(|| pos + 1),
            Dir::S => (pos + line_len < grid_len).then(|| pos + line_len),
            Dir::W => (pos % line_len != 0).then(|| pos - 1),
        }
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> u32 {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let grid_size = rows * cols;
    let mut visited = vec![0u64; (grid_size + 63) / 64]; // Flat bitmask
    let mut visited_count = 0;

    let start_pos = input.bytes().position(|b| b == b'^').unwrap();
    let mut pos = start_pos;
    let mut dir = Dir::N;

    loop {
        if mark_visited(&mut visited, pos) {
            visited_count += 1;
        }

        if let Some(next_pos) = dir.step(pos, cols + 1, rows * (cols + 1)) {
            if input.as_bytes()[next_pos] == b'#' {
                dir = dir.rotate();
            } else {
                pos = next_pos;
            }
        } else {
            break;
        }
    }

    visited_count
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u32 {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let start_pos = input.bytes().position(|b| b == b'^').unwrap();

    // First, find all positions visited in the original path
    let visited_positions = get_path_positions(input.as_bytes(), cols + 1, rows * (cols + 1), start_pos);

    let mut obstacle_count = 0;

    // Only check obstacles on the original path
    for pos in visited_positions {
        if pos == start_pos {
            continue;
        }

        if causes_loop(input.as_bytes(), cols + 1, rows * (cols + 1), start_pos, pos) {
            obstacle_count += 1;
        }
    }

    obstacle_count
}

fn mark_visited(visited: &mut [u64], pos: usize) -> bool {
    let idx = pos / 64;
    let bit = 1 << (pos % 64);
    if visited[idx] & bit == 0 {
        visited[idx] |= bit;
        true
    } else {
        false
    }
}

fn get_path_positions(bytes: &[u8], line_len: usize, grid_len: usize, start_pos: usize) -> Vec<usize> {
    let mut visited = vec![0u64; (grid_len + 63) / 64];
    let mut positions = Vec::new();
    let mut pos = start_pos;
    let mut dir = Dir::N;

    loop {
        if mark_visited(&mut visited, pos) {
            positions.push(pos);
        }

        if let Some(next_pos) = dir.step(pos, line_len, grid_len) {
            if bytes[next_pos] == b'#' {
                dir = dir.rotate();
            } else {
                pos = next_pos;
            }
        } else {
            break;
        }
    }

    positions
}

fn causes_loop(
    bytes: &[u8],
    line_len: usize,
    grid_len: usize,
    start_pos: usize,
    obstacle_pos: usize,
) -> bool {
    // Use a bit array to track visited states: 4 bits per position (one per direction)
    let mut visited_states = vec![0u8; grid_len];
    let mut pos = start_pos;
    let mut dir = Dir::N;

    loop {
        let dir_bit = match dir {
            Dir::N => 1,
            Dir::E => 2,
            Dir::S => 4,
            Dir::W => 8,
        };

        if visited_states[pos] & dir_bit != 0 {
            return true; // Loop detected
        }
        visited_states[pos] |= dir_bit;

        if let Some(next_pos) = dir.step(pos, line_len, grid_len) {
            if next_pos == obstacle_pos || bytes[next_pos] == b'#' {
                dir = dir.rotate();
            } else {
                pos = next_pos;
            }
        } else {
            return false; // Exit the map
        }
    }
}
