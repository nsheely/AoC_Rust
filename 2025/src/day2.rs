/// Day 2: Gift Shop
///
/// Find and sum all "invalid" product IDs within given ranges.
/// An invalid ID is any number formed by repeating a digit sequence.
/// Examples: 11 (1 repeated twice), 6464 (64 repeated twice), 123123 (123 repeated twice)
///
/// Rather than iterating through every number in the range checking if it's invalid,
/// we compute which patterns exist in the range, then sum them with an arithmetic series.
///
/// Pattern multipliers:
/// - 2-digit pattern repeated twice: XY * 101 (e.g., 12 * 101 = 1212)
/// - 3-digit pattern repeated twice: ABC * 1001 (e.g., 123 * 1001 = 123123)
/// - General: pattern * ((10^total_digits - 1) / (10^pattern_digits - 1))

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

/// Sum all invalid IDs in a range with a specific repetition structure.
///
/// total_digits: Total digits in the full number
/// pattern_digits: Digits in the repeating unit
///
/// Example: total_digits=6, pattern_digits=2 → "AB" repeated 3 times (ABABAB)
#[inline]
fn sum_for_config(start: u64, end: u64, total_digits: u32, pattern_digits: u32) -> u64 {
    if total_digits % pattern_digits != 0 {
        return 0;
    }

    let pattern_min = 10u64.pow(pattern_digits - 1);
    let pattern_max = 10u64.pow(pattern_digits) - 1;

    // Multiplier converts base pattern to full number
    // Example: for "AB" repeated 3 times, multiplier is 10101, so AB * 10101 = ABABAB
    let multiplier = (10u64.pow(total_digits) - 1) / (10u64.pow(pattern_digits) - 1);

    // Range bounds for all possible invalid IDs with this structure
    let invalid_min = pattern_min * multiplier;
    let invalid_max = pattern_max * multiplier;

    // Intersect with the query range [start, end]
    let lower = start.max(invalid_min);
    let upper = end.min(invalid_max);

    if lower > upper {
        return 0;
    }

    // Convert back to base patterns to count how many exist in range
    let first_pattern = (lower + multiplier - 1) / multiplier; // Round up
    let last_pattern = upper / multiplier; // Round down

    if first_pattern > last_pattern || first_pattern < pattern_min || last_pattern > pattern_max {
        return 0;
    }

    // Arithmetic series: sum = (first + last) * count / 2
    let count = last_pattern - first_pattern + 1;
    let sum_of_patterns = (first_pattern + last_pattern) * count / 2;

    sum_of_patterns * multiplier
}

#[aoc(day2, part1)]
pub fn part1(ranges: &[(u64, u64)]) -> u64 {
    ranges
        .iter()
        .map(|&(start, end)| {
            // Check all patterns repeated exactly 2 times
            // 1-digit × 2 = 2 digits, 2-digit × 2 = 4 digits, ..., 10-digit × 2 = 20 digits
            (1..=10)
                .map(|pattern_len| sum_for_config(start, end, pattern_len * 2, pattern_len))
                .sum::<u64>()
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(ranges: &[(u64, u64)]) -> u64 {
    // Part 2: Patterns repeated at least 2 times (vs exactly 2 times in Part 1)
    // Use inclusion-exclusion to avoid counting numbers multiple ways

    // Start with all patterns repeated exactly 2 times
    let part1_sum: u64 = ranges
        .iter()
        .map(|&(start, end)| {
            (1..=10)
                .map(|pattern_len| sum_for_config(start, end, pattern_len * 2, pattern_len))
                .sum::<u64>()
        })
        .sum();

    // Add patterns with 3+ repetitions
    let configs = [
        (3, 1),   // "X" repeated 3 times (XXX)
        (5, 1),   // "X" repeated 5 times (XXXXX)
        (6, 2),   // "XY" repeated 3 times (XYXYXY)
        (7, 1),   // "X" repeated 7 times (XXXXXXX)
        (9, 3),   // "XYZ" repeated 3 times (XYZXYZXYZ)
        (10, 2),  // "XY" repeated 5 times (XYXYXYXYXY)
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

    // Subtract numbers counted multiple ways
    // Example: 111111 is both "11" repeated 3× and "111" repeated 2×
    let overlap_sum: u64 = ranges
        .iter()
        .map(|&(start, end)| {
            sum_for_config(start, end, 6, 1) + sum_for_config(start, end, 10, 1)
        })
        .sum();

    part1_sum + additional_sum - overlap_sum
}
