// Day 5: Cafeteria
//
// Determine which ingredient IDs are fresh based on ranges.
// Part 1: Count available IDs that fall into any fresh range.
// Part 2: Count total IDs covered by fresh ranges.
//
// Strategy: Sort and merge overlapping ranges, then use two-pointer merge.

pub struct Input {
    ranges: Vec<(u64, u64)>,
    ids: Vec<u64>,
}

#[aoc_generator(day5)]
pub fn parse(input: &str) -> Input {
    let mut parts = input.split("\n\n");

    let mut ranges: Vec<(u64, u64)> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut nums = line.split('-');
            (
                nums.next().unwrap().parse().unwrap(),
                nums.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    // Sort and merge overlapping ranges
    ranges.sort_unstable();

    let mut merged = Vec::with_capacity(ranges.len());
    if let Some(&first) = ranges.first() {
        let mut current = first;
        for &(start, end) in &ranges[1..] {
            if start <= current.1 + 1 {
                current.1 = current.1.max(end);
            } else {
                merged.push(current);
                current = (start, end);
            }
        }
        merged.push(current);
    }
    let ranges = merged;

    let mut ids: Vec<u64> = parts
        .next()
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    ids.sort_unstable();

    Input { ranges, ids }
}

#[aoc(day5, part1)]
pub fn part1(input: &Input) -> usize {
    let mut count = 0;
    let mut idx = 0;

    for &(start, end) in &input.ranges {
        // Skip IDs before range start
        while idx < input.ids.len() && input.ids[idx] < start {
            idx += 1;
        }

        // Count IDs within range
        let range_start = idx;
        while idx < input.ids.len() && input.ids[idx] <= end {
            idx += 1;
        }
        count += idx - range_start;
    }

    count
}

#[aoc(day5, part2)]
pub fn part2(input: &Input) -> u64 {
    input
        .ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}
