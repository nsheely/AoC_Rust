// Day 21: Step Counter
//
// Part 1: BFS to find all plots reachable in exactly 64 steps.
// Part 2: Geometric calculation for infinite grid after 26501365 steps.
//
// The real input has special properties:
// - Clear horizontal and vertical roads from center
// - Completely clear edges
// - Grid is 131x131 with start at (65, 65)
// - 26501365 = 65 + 131 * 202300
//
// This means we can cross a tile in exactly 131 steps.
// The reachable area forms a diamond 202300 tiles wide.
// Due to parity (odd grid size), plots flip odd/even across tile boundaries.

const CENTER: i32 = 65;
const SIZE: i32 = 131;
const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub struct Parsed {
    pub part1: u64,
    pub part2: u64,
}

#[aoc_generator(day21)]
pub fn parse_input(input: &str) -> Parsed {
    // Parse grid as flat byte vec
    let grid: Vec<u8> = input.bytes().filter(|&b| b != b'\n').collect();

    // BFS from center to categorize plots by parity and distance
    let (even_inner, even_outer, odd_inner, odd_outer) =
        bfs_categorize(&grid, &[(CENTER, CENTER)], 130);

    // Part 1: even plots within 64 steps
    let part1 = even_inner;

    let even_full = even_inner + even_outer;
    let odd_full = odd_inner + odd_outer;
    let remove_corners = odd_outer;

    // BFS from corners to find reachable edge plots
    let corners = [(0, 0), (130, 0), (0, 130), (130, 130)];
    let (even_corners, ..) = bfs_categorize(&grid, &corners, 64);
    let add_corners = even_corners;

    // Calculate diamond area: n = 202300
    let n = 202300u64;
    let part2 = n * n * even_full
        + (n + 1) * (n + 1) * odd_full
        + n * add_corners
        - (n + 1) * remove_corners;

    Parsed { part1, part2 }
}

#[aoc(day21, part1)]
pub fn part1(parsed: &Parsed) -> u64 {
    parsed.part1
}

#[aoc(day21, part2)]
pub fn part2(parsed: &Parsed) -> u64 {
    parsed.part2
}

/// BFS that categorizes plots by parity and distance from center
#[inline(always)]
fn bfs_categorize(
    grid_input: &[u8],
    starts: &[(i32, i32)],
    limit: u32,
) -> (u64, u64, u64, u64) {
    // Clone grid and mark visited by setting to '#'
    let mut grid = grid_input.to_vec();
    let mut queue = [(0i32, 0i32, 0u32); 20000];
    let mut head = 0;
    let mut tail = 0;

    for &(x, y) in starts {
        let idx = (y * SIZE + x) as usize;
        unsafe { *grid.get_unchecked_mut(idx) = b'#' };
        queue[tail] = (x, y, 0);
        tail += 1;
    }

    let mut even_inner = 0;
    let mut even_outer = 0;
    let mut odd_inner = 0;
    let mut odd_outer = 0;

    while head < tail {
        let (x, y, dist) = unsafe { *queue.get_unchecked(head) };
        head += 1;

        // Manhattan distance from center
        let manhattan = (x - CENTER).abs() + (y - CENTER).abs();

        // Categorize by parity and distance
        if dist & 1 == 1 {
            if manhattan <= 65 {
                odd_inner += 1;
            } else {
                odd_outer += 1;
            }
        } else if dist <= 64 {
            even_inner += 1;
        } else {
            even_outer += 1;
        }

        if dist < limit {
            for &(dx, dy) in &DIRS {
                let nx = x + dx;
                let ny = y + dy;

                if nx >= 0 && nx < SIZE && ny >= 0 && ny < SIZE {
                    let idx = (ny * SIZE + nx) as usize;
                    unsafe {
                        if *grid.get_unchecked(idx) != b'#' {
                            *grid.get_unchecked_mut(idx) = b'#';
                            *queue.get_unchecked_mut(tail) = (nx, ny, dist + 1);
                            tail += 1;
                        }
                    }
                }
            }
        }
    }

    (even_inner, even_outer, odd_inner, odd_outer)
}
