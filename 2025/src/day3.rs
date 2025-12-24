// Day 3: Lobby
//
// Find maximum joltage from battery banks.
// Each bank is a line of digits. Turn on N batteries to form an N-digit number.
// Part 1: Sum of maximum joltage (2 batteries, 2-digit number)
// Part 2: Sum of maximum joltage (12 batteries, 12-digit number)

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64 {
    solve::<2>(input)
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
    solve::<12>(input)
}

fn solve<const N: usize>(input: &str) -> u64 {
    let mut batteries = [0u8; N];

    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let bytes = line.as_bytes();
            let len = bytes.len();

            if len < N {
                return 0;
            }

            // Initialize with last N digits
            let end = len - N;
            batteries[..N].copy_from_slice(&bytes[end..]);

            // Scan backwards, maintaining the N largest digits in sorted order
            // Each candidate swaps through the array until finding its position
            for &b in bytes[..end].iter().rev() {
                let mut candidate = b;
                for slot in &mut batteries {
                    if candidate < *slot {
                        break; // Candidate too small, done
                    }
                    // Swap candidate in, continue with displaced value
                    std::mem::swap(&mut *slot, &mut candidate);
                }
            }

            // Convert digit bytes to decimal number
            batteries
                .iter()
                .fold(0u64, |acc, &b| acc * 10 + (b - b'0') as u64)
        })
        .sum()
}
