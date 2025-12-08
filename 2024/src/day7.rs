use rayon::prelude::*;

const MAX_OPERANDS: usize = 10;
const POWERS_OF_10: [u64; 20] = [
    1, 10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000, 1_000_000_000,
    10_000_000_000, 100_000_000_000, 1_000_000_000_000, 10_000_000_000_000, 100_000_000_000_000,
    1_000_000_000_000_000, 10_000_000_000_000_000, 100_000_000_000_000_000,
    1_000_000_000_000_000_000, 10_000_000_000_000_000_000,
];

fn parse_line(line: &[u8]) -> Option<(u64, [u64; MAX_OPERANDS], usize)> {
    let colon_pos = line.iter().position(|&b| b == b':')?;
    let target = parse_u64(&line[..colon_pos])?;

    let mut operands = [0u64; MAX_OPERANDS];
    let mut len = 0;

    for chunk in line[colon_pos + 2..].split(|&b| b == b' ') {
        if len == MAX_OPERANDS {
            return None; // Exceeded maximum size
        }
        operands[len] = parse_u64(chunk)?;
        len += 1;
    }

    Some((target, operands, len))
}

fn parse_u64(slice: &[u8]) -> Option<u64> {
    let mut result = 0u64;
    for &b in slice {
        if b < b'0' || b > b'9' {
            return None;
        }
        result = result * 10 + (b - b'0') as u64;
    }
    Some(result)
}

fn reverse_recurse(accum: u64, operands: &[u64], length: usize, allow_concat: bool) -> bool {
    if length == 1 {
        return accum == operands[0];
    }

    let current = operands[length - 1];

    // Try subtraction
    if accum > current && reverse_recurse(accum - current, operands, length - 1, allow_concat) {
        return true;
    }

    // Try division
    if accum % current == 0 && reverse_recurse(accum / current, operands, length - 1, allow_concat) {
        return true;
    }

    // Try concatenation
    if allow_concat {
        let factor = POWERS_OF_10[(current.ilog10() as usize) + 1];
        if accum % factor == current
            && reverse_recurse(accum / factor, operands, length - 1, allow_concat)
        {
            return true;
        }
    }

    false
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    input
        .par_lines() // Parallel processing with Rayon
        .filter_map(|line| {
            let bytes = line.as_bytes();
            let (target, operands, len) = parse_line(bytes)?;
            if reverse_recurse(target, &operands, len, false) {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    input
        .par_lines() // Parallel processing with Rayon
        .filter_map(|line| {
            let bytes = line.as_bytes();
            let (target, operands, len) = parse_line(bytes)?;
            if reverse_recurse(target, &operands, len, false)
                || reverse_recurse(target, &operands, len, true)
            {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}
