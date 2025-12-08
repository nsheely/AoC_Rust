// Day 1: Trebuchet?!
//
// Extract calibration values from each line by finding the first and last digits,
// combining them into two-digit numbers, and summing the results.
//
// Part 2: Digits can be spelled out ("one", "two", etc.) and may overlap.

#[aoc(day1, part1)]
pub fn part1(document: &str) -> u32 {
    document
        .lines()
        .filter_map(|line| {
            let first_digit = line.chars().find(|c| c.is_ascii_digit());
            let last_digit = line.chars().rev().find(|c| c.is_ascii_digit());

            if let (Some(f), Some(l)) = (first_digit, last_digit) {
                f.to_digit(10)
                    .and_then(|f_val| l.to_digit(10).map(|l_val| f_val * 10 + l_val))
            } else {
                None
            }
        })
        .sum()
}

struct NumberFinder {
    spelled_numbers: [(&'static str, u32); 9],
}

impl NumberFinder {
    fn new() -> Self {
        let spelled_numbers = [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];
        NumberFinder { spelled_numbers }
    }

    fn find_first_and_last_number(&self, line: &str) -> (Option<u32>, Option<u32>) {
        let mut first_number = None;
        let mut last_number = None;

        for (index, c) in line.char_indices() {
            let mut updated = false;

            for &(word, value) in &self.spelled_numbers {
                if line[index..].starts_with(word) {
                    if first_number.is_none() {
                        first_number = Some(value);
                    }
                    last_number = Some(value);
                    updated = true;
                    break;
                }
            }

            if !updated && c.is_ascii_digit() {
                let digit = c.to_digit(10).unwrap();
                if first_number.is_none() {
                    first_number = Some(digit);
                }
                last_number = Some(digit);
            }
        }

        (first_number, last_number)
    }
}

#[aoc(day1, part2)]
pub fn part2(document: &str) -> u32 {
    let finder = NumberFinder::new();
    document
        .lines()
        .filter_map(|line| {
            let (first_digit, last_digit) = finder.find_first_and_last_number(line);

            if let (Some(f), Some(l)) = (first_digit, last_digit) {
                Some(f * 10 + l)
            } else {
                None
            }
        })
        .sum()
}
