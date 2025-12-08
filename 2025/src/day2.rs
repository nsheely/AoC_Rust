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

/// Precomputed constants for a repetition configuration.
struct Config {
    pattern_min: u64,
    pattern_max: u64,
    multiplier: u64,
    invalid_min: u64,
    invalid_max: u64,
}

impl Config {
    fn new(total_digits: u32, pattern_digits: u32) -> Option<Self> {
        if total_digits % pattern_digits != 0 {
            return None;
        }

        let pattern_min = 10u64.pow(pattern_digits - 1);
        let pattern_max = 10u64.pow(pattern_digits) - 1;
        let multiplier = (10u64.pow(total_digits) - 1) / (10u64.pow(pattern_digits) - 1);
        let invalid_min = pattern_min * multiplier;
        let invalid_max = pattern_max * multiplier;

        Some(Config {
            pattern_min,
            pattern_max,
            multiplier,
            invalid_min,
            invalid_max,
        })
    }

    /// Sum all invalid IDs in a range with this configuration.
    #[inline]
    fn sum_in_range(&self, start: u64, end: u64) -> u64 {
        // Intersect with the query range [start, end]
        let lower = start.max(self.invalid_min);
        let upper = end.min(self.invalid_max);

        if lower > upper {
            return 0;
        }

        // Convert back to base patterns to count how many exist in range
        let first_pattern = (lower + self.multiplier - 1) / self.multiplier; // Round up
        let last_pattern = upper / self.multiplier; // Round down

        if first_pattern > last_pattern
            || first_pattern < self.pattern_min
            || last_pattern > self.pattern_max
        {
            return 0;
        }

        // Arithmetic series: sum = (first + last) * count / 2
        let count = last_pattern - first_pattern + 1;
        let sum_of_patterns = (first_pattern + last_pattern) * count / 2;

        sum_of_patterns * self.multiplier
    }
}

#[aoc(day2, part1)]
pub fn part1(ranges: &[(u64, u64)]) -> u64 {
    // Check all patterns repeated exactly 2 times
    // 1-digit × 2 = 2 digits, 2-digit × 2 = 4 digits, ..., 10-digit × 2 = 20 digits
    (1..=10)
        .filter_map(|pattern_len| Config::new(pattern_len * 2, pattern_len))
        .map(|config| {
            ranges
                .iter()
                .map(|&(start, end)| config.sum_in_range(start, end))
                .sum::<u64>()
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(ranges: &[(u64, u64)]) -> u64 {
    // Part 2: Patterns repeated at least 2 times (vs exactly 2 times in Part 1)
    // Use inclusion-exclusion to avoid counting numbers multiple ways

    // Start with all patterns repeated exactly 2 times
    let part1_sum: u64 = (1..=10)
        .filter_map(|pattern_len| Config::new(pattern_len * 2, pattern_len))
        .map(|config| {
            ranges
                .iter()
                .map(|&(start, end)| config.sum_in_range(start, end))
                .sum::<u64>()
        })
        .sum();

    // Add patterns with 3+ repetitions
    let additional_configs = [
        (3, 1),   // "X" repeated 3 times (XXX)
        (5, 1),   // "X" repeated 5 times (XXXXX)
        (6, 2),   // "XY" repeated 3 times (XYXYXY)
        (7, 1),   // "X" repeated 7 times (XXXXXXX)
        (9, 3),   // "XYZ" repeated 3 times (XYZXYZXYZ)
        (10, 2),  // "XY" repeated 5 times (XYXYXYXYXY)
    ];

    let additional_sum: u64 = additional_configs
        .iter()
        .filter_map(|&(total, pattern)| Config::new(total, pattern))
        .map(|config| {
            ranges
                .iter()
                .map(|&(start, end)| config.sum_in_range(start, end))
                .sum::<u64>()
        })
        .sum();

    // Subtract numbers counted multiple ways
    // Example: 111111 is both "11" repeated 3× and "111" repeated 2×
    let overlap_configs = [(6, 1), (10, 1)];

    let overlap_sum: u64 = overlap_configs
        .iter()
        .filter_map(|&(total, pattern)| Config::new(total, pattern))
        .map(|config| {
            ranges
                .iter()
                .map(|&(start, end)| config.sum_in_range(start, end))
                .sum::<u64>()
        })
        .sum();

    part1_sum + additional_sum - overlap_sum
}
