// Day 4: Printing Department
//
// Count paper rolls (@) that can be accessed by forklifts.
// Part 1: Count rolls with fewer than 4 neighboring rolls
// Part 2: Count rolls removed by cascade (removing a roll reduces neighbor counts)

#[inline(always)]
const fn neighbor_offsets(stride: usize) -> [isize; 8] {
    let s = stride as isize;
    [-s - 1, -s, -s + 1, -1, 1, s - 1, s, s + 1]
}

// Shared setup: creates padded grid and returns (padded, width, height, offsets)
#[inline]
fn setup(input: &str) -> (Vec<u8>, usize, usize, [isize; 8]) {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let height = grid.len();
    let width = grid[0].len();
    let padded_width = width + 2;

    let mut padded = vec![b'.'; padded_width * (height + 2)];
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            padded[(y + 1) * padded_width + (x + 1)] = cell;
        }
    }

    (padded, padded_width, height, neighbor_offsets(padded_width))
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let (padded, padded_width, height, offsets) = setup(input);
    let width = padded_width - 2;
    let mut accessible = 0;

    for y in 0..height {
        for x in 0..width {
            let idx = (y + 1) * padded_width + (x + 1);
            if padded[idx] != b'@' {
                continue;
            }

            let count = offsets
                .iter()
                .filter(|&&off| padded[(idx as isize + off) as usize] == b'@')
                .count();

            if count < 4 {
                accessible += 1;
            }
        }
    }

    accessible
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let (padded, padded_width, height, offsets) = setup(input);
    let width = padded_width - 2;

    let mut counts = vec![255u8; padded_width * (height + 2)];
    let mut queue = Vec::with_capacity(2048);

    // Count neighbors and queue initially accessible cells
    for y in 0..height {
        for x in 0..width {
            let idx = (y + 1) * padded_width + (x + 1);
            if padded[idx] != b'@' {
                continue;
            }

            let count = offsets
                .iter()
                .filter(|&&off| padded[(idx as isize + off) as usize] == b'@')
                .count() as u8;

            counts[idx] = count;
            if count < 4 {
                queue.push(idx);
            }
        }
    }

    // Process cascade removals
    let mut removed = 0;
    let mut i = 0;

    while i < queue.len() {
        let pos = queue[i];
        i += 1;
        removed += 1;

        // Decrement neighbor counts and queue newly accessible cells
        for &offset in &offsets {
            let neighbor = (pos as isize + offset) as usize;
            if counts[neighbor] == 4 {
                queue.push(neighbor);
            }
            counts[neighbor] -= 1;
        }
    }

    removed
}
