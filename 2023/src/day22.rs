// Day 22: Sand Slabs
//
// Falling bricks simulation using graph theory.
// Key insights:
// - x and y coordinates are 0-9, so we use a 10x10 grid
// - Bricks sorted by z coordinate form a topological sort
// - This is a graph where edges represent "supports" relationships
//
// Part 1: Count bricks that can be safely removed
// Part 2: Sum of depths in dominator tree (total bricks that fall)

pub struct Parsed {
    pub part1: usize,
    pub part2: usize,
}

#[aoc_generator(day22)]
pub fn parse_input(input: &str) -> Parsed {
    // Parse bricks as [x1, y1, z1, x2, y2, z2]
    let mut bricks = Vec::with_capacity(1500);
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        let mut nums = [0usize; 6];
        let mut idx = 0;
        let mut num = 0;
        let mut has_digit = false;

        while i < bytes.len() && idx < 6 {
            let b = bytes[i];
            i += 1;

            let digit = b.wrapping_sub(b'0');
            if digit < 10 {
                num = num * 10 + digit as usize;
                has_digit = true;
            } else if has_digit {
                nums[idx] = num;
                idx += 1;
                num = 0;
                has_digit = false;
            }
        }
        if has_digit {
            nums[idx] = num;
            idx += 1;
        }

        if idx == 6 {
            bricks.push(nums);
        }
    }

    // x and y limited to 0-9, so 10x10 grid = 100 elements
    let mut heights = [0u16; 100];
    let mut indices = [u16::MAX; 100];

    // Track safe/unsafe bricks and dominator tree
    let mut safe = vec![true; bricks.len()];
    let mut dominator: Vec<(u16, u16)> = Vec::with_capacity(bricks.len());

    // Sort ascending by lowest z coordinate
    bricks.sort_unstable_by_key(|b| b[2]);

    for (i, brick) in bricks.iter().enumerate() {
        let x1 = brick[0];
        let y1 = brick[1];
        let z1 = brick[2];
        let x2 = brick[3];
        let y2 = brick[4];
        let z2 = brick[5];

        // Treat 1D array as 2D grid
        let start = 10 * y1 + x1;
        let end = 10 * y2 + x2;
        let step = if y2 > y1 { 10 } else { 1 };
        let height = z2 - z1 + 1;

        // Track what's underneath the brick
        let mut top = 0u16;
        let mut previous = u16::MAX;
        let mut underneath = 0u16;
        let mut parent = 0u16;
        let mut depth = 0u16;

        // Find highest z coordinate underneath brick
        let mut j = start;
        while j <= end {
            top = top.max(heights[j]);
            j += step;
        }

        // Check which bricks support this one
        let mut j = start;
        while j <= end {
            if heights[j] == top {
                let index = indices[j];
                if index != previous {
                    previous = index;
                    underneath += 1;

                    if underneath == 1 {
                        (parent, depth) = dominator[previous as usize];
                    } else {
                        // Find common ancestor
                        let (mut a, mut b) = (parent, depth);
                        let (mut x, mut y) = dominator[previous as usize];

                        // Align depths
                        while b > y {
                            (a, b) = dominator[a as usize];
                        }
                        while y > b {
                            (x, y) = dominator[x as usize];
                        }

                        // Find common ancestor
                        while a != x {
                            (a, b) = dominator[a as usize];
                            (x, _) = dominator[x as usize];
                        }

                        (parent, depth) = (a, b);
                    }
                }
            }

            // Update grid with new height and index
            heights[j] = top + height as u16;
            indices[j] = i as u16;
            j += step;
        }

        // If only 1 supporter, mark it unsafe and increase depth
        if underneath == 1 {
            safe[previous as usize] = false;
            parent = previous;
            depth = dominator[previous as usize].1 + 1;
        }

        dominator.push((parent, depth));
    }

    let part1 = safe.iter().filter(|&&b| b).count();
    let part2 = dominator.iter().map(|(_, d)| *d as usize).sum();

    Parsed { part1, part2 }
}

#[aoc(day22, part1)]
pub fn part1(parsed: &Parsed) -> usize {
    parsed.part1
}

#[aoc(day22, part2)]
pub fn part2(parsed: &Parsed) -> usize {
    parsed.part2
}
