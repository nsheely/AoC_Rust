// Day 7: Laboratories
//
// Simulate tachyon beams splitting through a manifold.
// Beams move downward, split at '^' into left/right children.
//
// Part 1: Count unique splitter hits.
// Part 2: Count timelines (N timelines → 2N timelines at each split).

type Output = (u64, u64);

#[aoc_generator(day7)]
pub fn parse(input: &str) -> Output {
    let lines: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let width = lines[0].len();
    let center = width / 2;

    let mut splits = 0;
    let mut timelines = vec![0u64; width];
    timelines[center] = 1;

    // Skip first 2 rows (S and empty), then process every other row (splitters)
    // Beams spread triangularly from center
    for (y, row) in lines.iter().skip(2).step_by(2).enumerate() {
        let start = center.saturating_sub(y);
        let end = (center + y + 1).min(width);

        for x in (start..end).step_by(2) {
            let count = timelines[x];
            if count > 0 && row[x] == b'^' {
                splits += 1;
                timelines[x] = 0;
                if x > 0 {
                    timelines[x - 1] += count;
                }
                if x + 1 < width {
                    timelines[x + 1] += count;
                }
            }
        }
    }

    (splits, timelines.iter().sum())
}

#[aoc(day7, part1)]
pub fn part1(input: &Output) -> u64 {
    input.0
}

#[aoc(day7, part2)]
pub fn part2(input: &Output) -> u64 {
    input.1
}
