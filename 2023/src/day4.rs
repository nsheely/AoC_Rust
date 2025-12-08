// Day 4: Scratchcards
//
// Calculate points from scratchcards by finding matching numbers.
// Part 1: First match = 1 point, each subsequent match doubles the value
// Part 2: Winning cards spawn copies of following cards

const MAX_NUMBER: usize = 100; // Maximum expected number

// Custom parser for small numbers (less than 100)
fn parse_small_number(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return None;
    }

    let mut result = (bytes[0].checked_sub(b'0')?) as usize;
    if result > 9 {
        return None; // Not a digit
    }

    if bytes.len() == 2 {
        let digit = bytes[1].checked_sub(b'0')?;
        if digit > 9 {
            return None;
        }
        result = result * 10 + digit as usize;
    } else if bytes.len() > 2 {
        // Parse longer numbers
        for &byte in &bytes[1..] {
            let digit = byte.checked_sub(b'0')?;
            if digit > 9 {
                return None;
            }
            result = result * 10 + digit as usize;
        }
    }

    Some(result)
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            let mut score = 0;
            let mut winning_numbers = [false; MAX_NUMBER];
            let mut player_numbers_started = false;

            // Skip the "Card X: " part
            let numbers_part = &line[9..];

            for word in numbers_part.split_whitespace() {
                // Detect when player numbers start
                if word == "|" {
                    player_numbers_started = true;
                    continue;
                }

                // Parse the number using custom lightweight parser
                let Some(number) = parse_small_number(word) else { continue };
                if number >= MAX_NUMBER {
                    continue; // Skip numbers that are too large
                }

                if player_numbers_started {
                    if winning_numbers[number] {
                        score = if score == 0 { 1 } else { score * 2 };
                    }
                } else {
                    winning_numbers[number] = true;
                }
            }

            score
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut card_counts = vec![1u32; lines.len()]; // Initialize with 1 for each card

    for (card_index, line) in lines.iter().enumerate() {
        let mut winning_numbers = [false; MAX_NUMBER];
        let mut player_numbers_started = false;
        let mut match_count = 0;
        // Skip the "Card X: " part
        let numbers_part = &line[9..];

        for word in numbers_part.split_whitespace() {
            // Detect when player numbers start
            if word == "|" {
                player_numbers_started = true;
                continue;
            }

            // Parse the number using custom lightweight parser
            let Some(number) = parse_small_number(word) else { continue };
            if number >= MAX_NUMBER {
                continue; // Skip numbers that are too large
            }

            if player_numbers_started {
                if winning_numbers[number] {
                    // Add copies for each subsequent card equal to the number of matches
                    match_count += 1;
                    let target_index = card_index + match_count;
                    if target_index < card_counts.len() {
                        card_counts[target_index] += card_counts[card_index];
                    }
                }
            } else {
                winning_numbers[number] = true;
            }
        }
    }

    card_counts.iter().sum() // Sum up the total number of cards
}
