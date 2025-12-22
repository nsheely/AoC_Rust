// Day 6: Trash Compactor
//
// Parse vertically-arranged math problems from columns.
// Part 1: Read left-to-right across rows.
// Part 2: Read top-to-bottom down columns (cephalopod math).

#[aoc(day6, part1)]
pub fn part1(input: &str) -> i64 {
    let lines: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    if lines.is_empty() {
        return 0;
    }

    let width = lines[0].len();
    let bottom = lines.len() - 1;

    let mut total = 0;
    let mut right = width;

    // Scan operator row right-to-left for problem boundaries
    for left in (0..width).rev() {
        let op = lines[bottom][left];
        if op == b' ' {
            continue;
        }

        // Parse numbers from each row (left-to-right within row)
        let numbers: Vec<i64> = lines[..bottom]
            .iter()
            .filter_map(|row| {
                let num = (left..right).fold(0i64, |acc, col| {
                    let b = row[col];
                    if b.is_ascii_digit() {
                        acc * 10 + (b - b'0') as i64
                    } else {
                        acc
                    }
                });
                (num > 0).then_some(num)
            })
            .collect();

        total += if op == b'+' {
            numbers.iter().sum::<i64>()
        } else {
            numbers.iter().product::<i64>()
        };

        right = left;
    }

    total
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> i64 {
    let lines: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    if lines.is_empty() {
        return 0;
    }

    let width = lines[0].len();
    let bottom = lines.len() - 1;

    let mut total = 0;
    let mut right = width;

    // Scan operator row right-to-left for problem boundaries
    for left in (0..width).rev() {
        let op = lines[bottom][left];
        if op == b' ' {
            continue;
        }

        // Parse numbers from each column (top-to-bottom, right-to-left)
        let numbers: Vec<i64> = (left..right)
            .rev()
            .filter_map(|col| {
                let num = (0..bottom).fold(0i64, |acc, row| {
                    let b = lines[row][col];
                    if b.is_ascii_digit() {
                        acc * 10 + (b - b'0') as i64
                    } else {
                        acc
                    }
                });
                (num > 0).then_some(num)
            })
            .collect();

        total += if op == b'+' {
            numbers.iter().sum::<i64>()
        } else {
            numbers.iter().product::<i64>()
        };

        right = left;
    }

    total
}
