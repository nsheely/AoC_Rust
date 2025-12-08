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

const fn pow10(n: u32) -> u64 {
    match n {
        0 => 1,
        1 => 10,
        2 => 100,
        3 => 1000,
        4 => 10000,
        5 => 100000,
        6 => 1000000,
        7 => 10000000,
        8 => 100000000,
        9 => 1000000000,
        10 => 10000000000,
        11 => 100000000000,
        12 => 1000000000000,
        13 => 10000000000000,
        14 => 100000000000000,
        15 => 1000000000000000,
        16 => 10000000000000000,
        17 => 100000000000000000,
        18 => 1000000000000000000,
        19 => 10000000000000000000,
        _ => panic!("pow10 only supports up to 10^19"),
    }
}

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
#[derive(Copy, Clone)]
struct Config {
    pattern_min: u64,
    pattern_max: u64,
    multiplier: u64,
    invalid_min: u64,
    invalid_max: u64,
}

// Precomputed configs for Part 1: patterns repeated exactly 2 times
const PART1_CONFIGS: [Config; 10] = [
    Config::new(2, 1),
    Config::new(4, 2),
    Config::new(6, 3),
    Config::new(8, 4),
    Config::new(10, 5),
    Config::new(12, 6),
    Config::new(14, 7),
    Config::new(16, 8),
    Config::new(18, 9),
    Config::new(20, 10), // largest input is ~10 digits, but we check up to 20 for completeness
];

// Precomputed configs for Part 2: additional patterns with 3+ repetitions
const PART2_ADDITIONAL: [Config; 6] = [
    Config::new(3, 1),
    Config::new(5, 1),
    Config::new(6, 2),
    Config::new(7, 1),
    Config::new(9, 3),
    Config::new(10, 2),
];

// Precomputed configs for Part 2: overlaps to subtract
const PART2_OVERLAPS: [Config; 2] = [Config::new(6, 1), Config::new(10, 1)];

impl Config {
    const fn new(total_digits: u32, pattern_digits: u32) -> Self {
        let pattern_min = pow10(pattern_digits - 1);
        let pattern_max = pow10(pattern_digits) - 1;

        // For patterns repeated n times: multiplier = (10^(pattern_digits * n) - 1) / (10^pattern_digits - 1)
        // For n=2: multiplier = 10^pattern_digits + 1
        // For n=3: multiplier = 10^(2*pattern_digits) + 10^pattern_digits + 1
        let multiplier = if total_digits == pattern_digits * 2 {
            pow10(pattern_digits) + 1
        } else if total_digits == pattern_digits * 3 {
            pow10(pattern_digits * 2) + pow10(pattern_digits) + 1
        } else if total_digits == pattern_digits * 5 {
            pow10(pattern_digits * 4) + pow10(pattern_digits * 3) + pow10(pattern_digits * 2) + pow10(pattern_digits) + 1
        } else if total_digits == pattern_digits * 7 {
            // For single digit patterns, we can compute this
            pow10(6) + pow10(5) + pow10(4) + pow10(3) + pow10(2) + pow10(1) + 1
        } else {
            // Fallback to division formula (works when it doesn't overflow)
            (pow10(total_digits) - 1) / (pow10(pattern_digits) - 1)
        };

        // Check for overflow when computing invalid_max
        let (invalid_max, overflow) = pattern_max.overflowing_mul(multiplier);
        let invalid_min = if overflow { 0 } else { pattern_min * multiplier };
        let invalid_max = if overflow { 0 } else { invalid_max };

        Config {
            pattern_min,
            pattern_max,
            multiplier,
            invalid_min,
            invalid_max,
        }
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
    PART1_CONFIGS
        .iter()
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
    let part1_sum: u64 = PART1_CONFIGS
        .iter()
        .map(|config| {
            ranges
                .iter()
                .map(|&(start, end)| config.sum_in_range(start, end))
                .sum::<u64>()
        })
        .sum();

    // Add patterns with 3+ repetitions
    let additional_sum: u64 = PART2_ADDITIONAL
        .iter()
        .map(|config| {
            ranges
                .iter()
                .map(|&(start, end)| config.sum_in_range(start, end))
                .sum::<u64>()
        })
        .sum();

    // Subtract numbers counted multiple ways
    // Example: 111111 is both "11" repeated 3× and "111" repeated 2×
    let overlap_sum: u64 = PART2_OVERLAPS
        .iter()
        .map(|config| {
            ranges
                .iter()
                .map(|&(start, end)| config.sum_in_range(start, end))
                .sum::<u64>()
        })
        .sum();

    part1_sum + additional_sum - overlap_sum
}
