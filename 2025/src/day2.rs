// Day 2: Gift Shop
//
// Find and sum all "invalid" product IDs within given ranges.
// An invalid ID is any number formed by repeating a digit sequence.
// Examples: 11 (1 repeated twice), 6464 (64 repeated twice), 123123 (123 repeated twice)
//
// Rather than iterating through every number in the range checking if it's invalid,
// we compute which patterns exist in the range, then sum them with an arithmetic series.
//
// Pattern multipliers:
// - 2-digit pattern repeated twice: XY * 101 (e.g., 12 * 101 = 1212)
// - 3-digit pattern repeated twice: ABC * 1001 (e.g., 123 * 1001 = 123123)
// - General: pattern * ((10^total_digits - 1) / (10^pattern_digits - 1))

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .flat_map(|line| {
            line.split(',').filter_map(|range| {
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
const PART1_CONFIGS: [Config; 9] = [
    Config {
        pattern_min: 1,
        pattern_max: 9,
        multiplier: 11,
        invalid_min: 11,
        invalid_max: 99,
    },
    Config {
        pattern_min: 10,
        pattern_max: 99,
        multiplier: 101,
        invalid_min: 1010,
        invalid_max: 9999,
    },
    Config {
        pattern_min: 100,
        pattern_max: 999,
        multiplier: 1001,
        invalid_min: 100100,
        invalid_max: 999999,
    },
    Config {
        pattern_min: 1000,
        pattern_max: 9999,
        multiplier: 10001,
        invalid_min: 10001000,
        invalid_max: 99999999,
    },
    Config {
        pattern_min: 10000,
        pattern_max: 99999,
        multiplier: 100001,
        invalid_min: 1000010000,
        invalid_max: 9999999999,
    },
    Config {
        pattern_min: 100000,
        pattern_max: 999999,
        multiplier: 1000001,
        invalid_min: 100000100000,
        invalid_max: 999999999999,
    },
    Config {
        pattern_min: 1000000,
        pattern_max: 9999999,
        multiplier: 10000001,
        invalid_min: 10000001000000,
        invalid_max: 99999999999999,
    },
    Config {
        pattern_min: 10000000,
        pattern_max: 99999999,
        multiplier: 100000001,
        invalid_min: 1000000010000000,
        invalid_max: 9999999999999999,
    },
    Config {
        pattern_min: 100000000,
        pattern_max: 999999999,
        multiplier: 1000000001,
        invalid_min: 100000000100000000,
        invalid_max: 999999999999999999,
    },
];

// Precomputed configs for Part 2: additional patterns with 3+ repetitions
const PART2_ADDITIONAL: [Config; 6] = [
    Config {
        pattern_min: 1,
        pattern_max: 9,
        multiplier: 111,
        invalid_min: 111,
        invalid_max: 999,
    },
    Config {
        pattern_min: 1,
        pattern_max: 9,
        multiplier: 11111,
        invalid_min: 11111,
        invalid_max: 99999,
    },
    Config {
        pattern_min: 10,
        pattern_max: 99,
        multiplier: 10101,
        invalid_min: 101010,
        invalid_max: 999999,
    },
    Config {
        pattern_min: 1,
        pattern_max: 9,
        multiplier: 1111111,
        invalid_min: 1111111,
        invalid_max: 9999999,
    },
    Config {
        pattern_min: 100,
        pattern_max: 999,
        multiplier: 1001001,
        invalid_min: 100100100,
        invalid_max: 999999999,
    },
    Config {
        pattern_min: 10,
        pattern_max: 99,
        multiplier: 101010101,
        invalid_min: 1010101010,
        invalid_max: 9999999999,
    },
];

// Precomputed configs for Part 2: overlaps to subtract
const PART2_OVERLAPS: [Config; 2] = [
    Config {
        pattern_min: 1,
        pattern_max: 9,
        multiplier: 111111,
        invalid_min: 111111,
        invalid_max: 999999,
    },
    Config {
        pattern_min: 1,
        pattern_max: 9,
        multiplier: 1111111111,
        invalid_min: 1111111111,
        invalid_max: 9999999999,
    },
];

impl Config {
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
        let first_pattern = lower.div_ceil(self.multiplier);
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
