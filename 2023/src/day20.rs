// Day 20: Pulse Propagation
//
// The input has a hidden structure: 4 independent 12-bit binary counters.
// Each counter is a chain of flip-flops that resets when hitting a threshold.
// We extract threshold values from the graph structure, then calculate pulse counts from those values.

use std::collections::HashMap;

type Input = [u32; 4];

pub fn parse_input(input: &str) -> Input {
    // Build graph: module name -> (children, is_flip_flop)
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut is_flipflop: HashMap<&str, bool> = HashMap::new();

    for line in input.lines() {
        // Extract lowercase identifiers (module names)
        let mut tokens = line
            .split(|c: char| !c.is_ascii_lowercase())
            .filter(|s| !s.is_empty());

        let key = tokens.next().unwrap();
        let children: Vec<&str> = tokens.collect();

        graph.insert(key, children);
        // Flip-flops start with '%', conjunctions with '&'
        is_flipflop.insert(key, !line.starts_with('&'));
    }

    // Follow chains from broadcaster, building binary counter values
    let mut stack = Vec::new();
    let mut counter_values = Vec::new();

    // Start from each broadcaster output
    for &start in &graph["broadcaster"] {
        stack.push((start, 0u32, 1u32)); // (node, value, bit_position)
    }

    while let Some((node, mut value, bit)) = stack.pop() {
        let children = &graph[node];

        // Find next flip-flop in chain
        if let Some(&next) = children.iter().find(|&&k| is_flipflop[k]) {
            // If this flip-flop has 2 children (one forward, one back to conjunction),
            // it means this bit is SET in the counter threshold
            if children.len() == 2 {
                value |= bit;
            }
            stack.push((next, value, bit << 1));
        } else {
            // Reached end of chain (conjunction), record the counter value
            counter_values.push(value | bit);
        }
    }

    counter_values.try_into().unwrap()
}

/// Count pulses by tracking state transitions with XOR operations
#[aoc(day20, part1)]
pub fn part1(input: &str) -> u32 {
    let counters = parse_input(input);

    // Verify assumption: all counter values > 1000 (no resets in first 1000 presses)
    assert!(counters.iter().all(|&n| n > 1000));

    // Calculate feedback pattern for each counter
    // Feedback width = 13 - count_ones(value)
    // (conjunctions feed back in inverse pattern to inputs, plus LSB always set)
    let pairs: Vec<(u32, u32)> = counters
        .iter()
        .map(|&n| (n, 13 - n.count_ones()))
        .collect();

    // Button and broadcaster contribute 5 low pulses per press
    let mut low = 5000;
    let mut high = 0;

    for n in 0..1000u32 {
        // Rising edge: flip-flops changing 0→1 emit HIGH pulse
        let rising: u32 = !n & (n + 1);
        high += 4 * rising.count_ones(); // 4 independent chains

        // Falling edge: flip-flops changing 1→0 emit LOW pulse
        let falling: u32 = n & !(n + 1);
        low += 4 * falling.count_ones();

        // For each counter, calculate feedback pulses
        for &(value, feedback) in &pairs {
            // Rising edges hitting counter bits
            let factor = (rising & value).count_ones();
            high += factor * (feedback + 3);
            low += factor;

            // Falling edges hitting counter bits
            let factor = (falling & value).count_ones();
            high += factor * (feedback + 2);
            low += 2 * factor;
        }
    }

    low * high
}

/// Part 2: LCM of counter values (coprime → LCM = product)
#[aoc(day20, part2)]
pub fn part2(input: &str) -> u64 {
    let counters = parse_input(input);
    counters.iter().map(|&n| n as u64).product()
}
