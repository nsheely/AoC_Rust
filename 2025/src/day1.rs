// Day 1: Dial Puzzle
//
// Track position on a circular dial with 100 positions, starting at 50.
// Part 1: Count exact landings on position 0
// Part 2: Count crossings through position 0 (including partial moves)
//
// Transform left turns into equivalent right turns.
// Going left from position P is the same as going right from (100-P).
// This allows both directions to use the same crossing formula: (position + distance) / 100

const DIAL_SIZE: i32 = 100;
const START_POSITION: i32 = 50;

/// Parse a line of input into (is_left, distance).
#[inline]
fn parse_line(line: &[u8]) -> Option<(bool, i32)> {
    if line.is_empty() {
        return None;
    }

    let is_left = line[0] == b'L';

    // Parse number from bytes
    let mut num = 0i32;
    for &byte in &line[1..] {
        if byte >= b'0' && byte <= b'9' {
            num = num * 10 + (byte - b'0') as i32;
        }
    }

    Some((is_left, num))
}

/// Count crossings through position 0 for Part 2.
///
/// Right turns from position P moving D steps cross 0 exactly (P+D)/100 times.
/// Left turns use a reversal: going left from P is like going right from (100-P).
/// This allows both directions to use the same crossing formula.
#[aoc_generator(day1)]
pub fn parse(input: &str) -> (i32, i32) {
    let mut position = START_POSITION;
    let mut part1_count = 0;
    let mut part2_count = 0;

    for line in input.as_bytes().split(|&b| b == b'\n') {
        if let Some((is_left, distance)) = parse_line(line) {
            if is_left {
                // Transform left-turn into equivalent right-turn for crossing calculation
                let mirrored_position = (DIAL_SIZE - position) % DIAL_SIZE;
                part2_count += (mirrored_position + distance) / DIAL_SIZE;
                position = (position - distance).rem_euclid(DIAL_SIZE);
            } else {
                // Right turn: direct calculation
                part2_count += (position + distance) / DIAL_SIZE;
                position = (position + distance) % DIAL_SIZE;
            }

            // Part 1: Count exact landings on position 0
            part1_count += i32::from(position == 0);
        }
    }

    (part1_count, part2_count)
}

#[aoc(day1, part1)]
pub fn part1(input: &(i32, i32)) -> i32 {
    input.0
}

#[aoc(day1, part2)]
pub fn part2(input: &(i32, i32)) -> i32 {
    input.1
}
