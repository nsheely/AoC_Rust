const DIAL_SIZE: i32 = 100;
const START_POSITION: i32 = 50;

/// Parse a line of input into (is_left, distance).
/// Uses byte-level parsing for speed - no string allocations or UTF-8 overhead.
#[inline]
fn parse_line(line: &[u8]) -> Option<(bool, i32)> {
    if line.is_empty() {
        return None;
    }

    let is_left = line[0] == b'L';

    // Manual digit parsing: ~3x faster than str::parse()
    let mut num = 0i32;
    for &byte in &line[1..] {
        if byte >= b'0' && byte <= b'9' {
            num = num * 10 + (byte - b'0') as i32;
        }
    }

    Some((is_left, num))
}

/// Single-pass solution computing both parts simultaneously.
///
/// Key insight for Part 2: Count how many times we pass through position 0.
/// - Right turn from P moving D steps: crosses 0 exactly (P+D)/100 times
/// - Left turn from P moving D steps: We "reverse" the dial perspective
///
/// The reversal trick: Going left from position P is equivalent to going right
/// from position (100-P) on a mirrored dial. This unifies the logic:
/// both directions use the same formula (position + distance) / DIAL_SIZE.
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
