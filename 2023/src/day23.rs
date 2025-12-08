// Day 23: A Long Walk
//
// The longest path problem is NP-hard. Approach:
//
// 1. Compression: Convert maze into undirected weighted graph
// 2. Grid Conversion: Transform graph into 6x6 grid representation
// 3. Row Deduplication: DP approach that deduplicates states at each row
//
// Part 1: Simple DP (directed graph, only right/down allowed)
// Part 2: Complex DP with row-by-row state exploration

use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

const UP: Point = Point { x: 0, y: -1 };
const DOWN: Point = Point { x: 0, y: 1 };
const LEFT: Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };
const ORTHOGONAL: [Point; 4] = [UP, DOWN, LEFT, RIGHT];
const ORIGIN: Point = Point { x: 0, y: 0 };

/// Simple grid wrapper
struct Grid {
    bytes: Vec<u8>,
    width: i32,
    height: i32,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len() as i32;
        let width = lines[0].len() as i32;
        let bytes: Vec<u8> = lines.iter().flat_map(|line| line.bytes()).collect();
        Grid { bytes, width, height }
    }

    fn get(&self, p: Point) -> u8 {
        if p.x >= 0 && p.x < self.width && p.y >= 0 && p.y < self.height {
            self.bytes[(p.y * self.width + p.x) as usize]
        } else {
            b'#'
        }
    }

    fn set(&mut self, p: Point, val: u8) {
        if p.x >= 0 && p.x < self.width && p.y >= 0 && p.y < self.height {
            self.bytes[(p.y * self.width + p.x) as usize] = val;
        }
    }
}

/// We only use 6 elements but use 8 for alignment.
type Row = [u8; 8];

/// Undirected weighted graph representing the compressed maze.
struct Graph {
    start: Point,
    end: Point,
    edges: HashMap<Point, Vec<Point>>,
    weight: HashMap<(Point, Point), u32>,
}

/// Distilled two dimensional array of only weights.
pub struct Input {
    extra: u32,
    horizontal: [[u32; 6]; 6],
    vertical: [[u32; 6]; 6],
}

/// Helper for iterating set bits
struct BitIterator(u32);

impl Iterator for BitIterator {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.0 == 0 {
            None
        } else {
            let bit = self.0.trailing_zeros() as usize;
            self.0 &= self.0 - 1;
            Some(bit)
        }
    }
}

trait Biterator {
    fn biterator(self) -> BitIterator;
}

impl Biterator for u32 {
    fn biterator(self) -> BitIterator {
        BitIterator(self)
    }
}

/// Parse and compress input into 6x6 grid.
#[aoc_generator(day23)]
pub fn parse_input(input: &str) -> Input {
    let graph = compress(input);
    graph_to_grid(&graph)
}

/// Part 1: Directed graph (only right/down) with DP.
#[aoc(day23, part1)]
pub fn part1(input: &Input) -> u32 {
    let mut total = [[0; 6]; 6];

    for y in 0..6 {
        for x in 0..6 {
            let left = if x > 0 { total[y][x - 1] + input.horizontal[y][x - 1] } else { 0 };
            let above = if y > 0 { total[y - 1][x] + input.vertical[y - 1][x] } else { 0 };
            total[y][x] = left.max(above);
        }
    }

    input.extra + total[5][5]
}

/// Part 2: Undirected graph, complex DP with row deduplication.
#[aoc(day23, part2)]
pub fn part2(input: &Input) -> u32 {
    let start = [b'S', 0, 0, 0, 0, 0, 0, 0];
    let end = [0, 0, 0, 0, 0, b'S', 0, 0];

    // Compute all possible 76 rows and their successors.
    let mut todo = VecDeque::new();
    let mut seen = HashSet::default();
    let mut graph = HashMap::default();

    todo.push_back(start);
    seen.insert(start);

    while let Some(row) = todo.pop_front() {
        let mut neighbors = Vec::new();
        dfs(&mut neighbors, row, [0; 8], 0, false, 0, 0);

        for &(next, ..) in &neighbors {
            if seen.insert(next) {
                todo.push_back(next);
            }
        }

        graph.insert(row, neighbors);
    }

    // Step through each row of the grid, keeping track of the maximum value for each row type.
    let mut current = HashMap::default();
    let mut next = HashMap::default();

    current.insert((start, false), 0);

    for y in 0..6 {
        for ((row, gap), steps) in current.drain() {
            for &(next_row, next_gap, horizontal, vertical) in &graph[&row] {
                // Only 1 gap total is allowed
                if gap && next_gap {
                    continue;
                }

                // Add edge weights
                let extra = horizontal.biterator().map(|x| input.horizontal[y][x]).sum::<u32>()
                    + vertical.biterator().map(|x| input.vertical[y][x]).sum::<u32>();

                // Deduplicate states
                let e = next.entry((next_row, gap || next_gap)).or_insert(0);
                *e = (*e).max(steps + extra);
            }
        }

        (current, next) = (next, current);
    }

    // The maximum path must have skipped 1 node.
    input.extra + current[&(end, true)]
}

/// Convert maze to undirected graph.
fn compress(input: &str) -> Graph {
    let mut grid = Grid::parse(input);
    let width = grid.width;
    let height = grid.height;

    // Move start and end away from edge.
    let start = Point::new(1, 1);
    let end = Point::new(width - 2, height - 2);

    // Modify edge of grid to remove the need for boundary checks.
    grid.set(start + UP, b'#');
    grid.set(end + DOWN, b'#');

    // BFS to find distances between POIs (points of interest: start, end, junctions).
    let mut poi = VecDeque::new();
    let mut seen = HashSet::default();
    let mut edges: HashMap<Point, Vec<Point>> = HashMap::default();
    let mut weight: HashMap<(Point, Point), u32> = HashMap::default();

    poi.push_back(start);
    grid.set(end, b'P');

    while let Some(from) = poi.pop_front() {
        // Mark this POI as done.
        grid.set(from, b'#');

        for direction in ORTHOGONAL {
            if grid.get(from + direction) != b'#' {
                let mut to = from + direction;
                let mut cost = 1;

                while grid.get(to) != b'P' {
                    let neighbors: Vec<Point> =
                        ORTHOGONAL.iter().map(|&o| to + o).filter(|&n| grid.get(n) != b'#').collect();
                    let next = neighbors[0];

                    // More than 1 neighbor means we've reached a junction.
                    if neighbors.len() > 1 {
                        grid.set(to, b'P');
                        break;
                    }

                    // Follow maze path toward next POI.
                    grid.set(to, b'#');
                    to = next;
                    cost += 1;
                }

                // Graph is undirected so add both edges.
                edges.entry(from).or_insert_with(Vec::new).push(to);
                edges.entry(to).or_insert_with(Vec::new).push(from);
                weight.insert((from, to), cost);
                weight.insert((to, from), cost);

                // Queue POI for processing if we haven't seen it before.
                if seen.insert(to) {
                    poi.push_back(to);
                }
            }
        }
    }

    Graph { start, end, edges, weight }
}

/// Convert graph to 6x6 grid representation.
fn graph_to_grid(graph: &Graph) -> Input {
    let Graph { start, end, edges, weight } = graph;

    // Extra steps for start and end (always taken).
    let extra = 2 + weight[&(*start, edges[start][0])] + weight[&(*end, edges[end][0])];

    // Helper to find next perimeter node.
    let mut seen = HashSet::default();
    let mut next_perimeter = |point: &Point| {
        *edges[point].iter().find(|&&next| edges[&next].len() == 3 && seen.insert(next)).unwrap()
    };

    let mut grid = [[ORIGIN; 6]; 6];
    let mut horizontal = [[0; 6]; 6];
    let mut vertical = [[0; 6]; 6];

    // Place start in top left.
    grid[0][0] = next_perimeter(start);

    // Fill out top edge and left edge.
    for i in 1..5 {
        let left = grid[0][i - 1];
        let above = grid[i - 1][0];

        let next_left = next_perimeter(&left);
        let next_above = next_perimeter(&above);

        grid[0][i] = next_left;
        grid[i][0] = next_above;
        horizontal[0][i - 1] = weight[&(left, next_left)];
        vertical[i - 1][0] = weight[&(above, next_above)];
    }

    // Add two extra corners by duplicating the last node.
    grid[0][5] = grid[0][4];
    grid[5][0] = grid[4][0];

    // Add remaining interior nodes.
    for y in 1..6 {
        for x in 1..6 {
            let left = grid[y][x - 1];
            let above = grid[y - 1][x];

            let (&next, _) = edges
                .iter()
                .find(|&(&k, v)| v.contains(&above) && v.contains(&left) && seen.insert(k))
                .unwrap();

            grid[y][x] = next;
            horizontal[y][x - 1] = weight[&(left, next)];
            vertical[y - 1][x] = weight[&(above, next)];
        }
    }

    Input { extra, horizontal, vertical }
}

/// Modified DFS that only allows rows that skip one node.
#[allow(clippy::too_many_arguments)]
fn dfs(
    result: &mut Vec<(Row, bool, u32, u32)>,
    previous: Row,
    current: Row,
    start: usize,
    gap: bool,
    horizontal: u32,
    vertical: u32,
) {
    // We're done, push the result.
    if start == 6 {
        result.push((current, gap, horizontal, vertical));
        return;
    }

    // Previous row has no vertical descending path.
    if previous[start] == 0 {
        // Skip at most 1 column per row.
        if !gap {
            dfs(result, previous, current, start + 1, true, horizontal, vertical);
        }

        let mut horizontal = horizontal;

        for end in (start + 1)..6 {
            horizontal |= 1 << (end - 1);

            if previous[end] == 0 {
                // Start a new path pair.
                let mut next = current;
                next[start] = b'S';
                next[end] = b'E';

                let vertical = vertical | (1 << start) | (1 << end);

                dfs(result, previous, next, end + 1, gap, horizontal, vertical);
            } else {
                // Move an existing path.
                let mut next = current;
                next[start] = previous[end];

                let vertical = vertical | (1 << start);

                dfs(result, previous, next, end + 1, gap, horizontal, vertical);
                break;
            }
        }
    } else {
        // Continue vertical path straight down.
        let mut next = current;
        next[start] = previous[start];
        dfs(result, previous, next, start + 1, gap, horizontal, vertical | (1 << start));

        let mut horizontal = horizontal;

        for end in (start + 1)..6 {
            horizontal |= 1 << (end - 1);

            if previous[end] == 0 {
                // Move existing path.
                let mut next = current;
                next[end] = previous[start];

                let vertical = vertical | (1 << end);

                dfs(result, previous, next, end + 1, gap, horizontal, vertical);
            } else {
                // Merge two path segments.
                match (previous[start], previous[end]) {
                    // No other changes needed.
                    (b'E', b'S') => {
                        dfs(result, previous, current, end + 1, gap, horizontal, vertical);
                    }
                    // Convert previous S to E.
                    (b'E', b'E') => {
                        let mut next = current;

                        for i in (0..start).rev() {
                            if current[i] == b'S' {
                                next[i] = b'E';
                                break;
                            }
                        }

                        dfs(result, previous, next, end + 1, gap, horizontal, vertical);
                    }
                    // Convert next E to S.
                    (b'S', b'S') => {
                        let mut modified = previous;
                        let mut level = 0;

                        for i in (end + 1)..6 {
                            if previous[i] == b'S' {
                                level += 1;
                            }
                            if previous[i] == b'E' {
                                if level == 0 {
                                    modified[i] = b'S';
                                    break;
                                }
                                level -= 1;
                            }
                        }

                        dfs(result, modified, current, end + 1, gap, horizontal, vertical);
                    }
                    _ => (), // (S, E) not allowed
                }
                break;
            }
        }
    }
}
