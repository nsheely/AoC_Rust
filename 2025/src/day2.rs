/// Day 2: Gift Shop
///
/// Find and sum all "invalid" product IDs within given ranges.
/// An invalid ID is any number formed by repeating a digit sequence exactly twice.
/// Examples: 11 (1 repeated), 6464 (64 repeated), 123123 (123 repeated)
///
/// Rather than checking each number, we generate invalid IDs directly.
/// For a given total digit count D (must be even), invalid IDs follow a pattern.
/// Example: 4-digit IDs with 2-digit sequences: 1010, 1111, 1212, ..., 9898, 9999
/// The sequence XY repeated twice equals: XY * 101 (for 2 digits)
/// The sequence ABC repeated twice equals: ABC * 1001 (for 3 digits)

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .flat_map(|line| {
            line.split(',')
                .filter_map(|range| {
                    let parts: Vec<&str> = range.trim().split('-').collect();
                    if parts.len() == 2 {
                        let start: u64 = parts[0].parse().ok()?;
                        let end: u64 = parts[1].parse().ok()?;
                        Some((start, end))
                    } else {
                        None
                    }
                })
        })
        .collect()
}

/// Sum invalid IDs in a range for a specific pattern configuration.
///
/// total_digits: Total number of digits in the invalid ID
/// pattern_digits: Length of the repeating pattern
///
/// Example: total_digits=6, pattern_digits=2 means "AB" repeated 3 times (ABABAB)
#[inline]
fn sum_for_config(start: u64, end: u64, total_digits: u32, pattern_digits: u32) -> u64 {
    if total_digits % pattern_digits != 0 {
        return 0;
    }

    let pattern_min = 10u64.pow(pattern_digits - 1);
    let pattern_max = 10u64.pow(pattern_digits) - 1;

    // Multiplier to convert pattern to full invalid ID
    // For pattern "AB" repeated 3 times: AB * 10101
    let multiplier = (10u64.pow(total_digits) - 1) / (10u64.pow(pattern_digits) - 1);

    // Range of invalid IDs
    let invalid_min = pattern_min * multiplier;
    let invalid_max = pattern_max * multiplier;

    // Find overlap with [start, end]
    let lower = start.max(invalid_min);
    let upper = end.min(invalid_max);

    if lower > upper {
        return 0;
    }

    // Find first and last patterns in range
    let first_pattern = (lower + multiplier - 1) / multiplier; // Round up
    let last_pattern = upper / multiplier; // Round down

    if first_pattern > last_pattern || first_pattern < pattern_min || last_pattern > pattern_max {
        return 0;
    }

    // Sum using arithmetic series formula
    let count = last_pattern - first_pattern + 1;
    let sum_of_patterns = (first_pattern + last_pattern) * count / 2;

    sum_of_patterns * multiplier
}

#[aoc(day2, part1)]
pub fn part1(ranges: &[(u64, u64)]) -> u64 {
    ranges
        .iter()
        .map(|&(start, end)| {
            // Patterns repeated exactly 2 times: [2,1], [4,2], [6,3], [8,4], [10,5]
            (1..=10)
                .map(|pattern_len| sum_for_config(start, end, pattern_len * 2, pattern_len))
                .sum::<u64>()
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(ranges: &[(u64, u64)]) -> u64 {
    // Part 2: Patterns repeated at least 2 times
    // Use inclusion-exclusion to avoid double-counting

    // All patterns repeated exactly 2 times
    let part1_sum: u64 = ranges
        .iter()
        .map(|&(start, end)| {
            (1..=10)
                .map(|pattern_len| sum_for_config(start, end, pattern_len * 2, pattern_len))
                .sum::<u64>()
        })
        .sum();

    // Additional patterns: odd repetitions (3, 5, 7, ...)
    // [3,1], [5,1], [6,2], [7,1], [9,3], [10,2]
    let configs = [
        (3, 1),   // "X" repeated 3 times
        (5, 1),   // "X" repeated 5 times
        (6, 2),   // "XY" repeated 3 times
        (7, 1),   // "X" repeated 7 times
        (9, 3),   // "XYZ" repeated 3 times
        (10, 2),  // "XY" repeated 5 times
    ];

    let additional_sum: u64 = ranges
        .iter()
        .map(|&(start, end)| {
            configs
                .iter()
                .map(|&(total, pattern)| sum_for_config(start, end, total, pattern))
                .sum::<u64>()
        })
        .sum();

    // Subtract overlaps: [6,1], [10,1]
    // These are numbers like 111111 that can be counted multiple ways
    let overlap_sum: u64 = ranges
        .iter()
        .map(|&(start, end)| {
            sum_for_config(start, end, 6, 1) + sum_for_config(start, end, 10, 1)
        })
        .sum();

    part1_sum + additional_sum - overlap_sum
}
