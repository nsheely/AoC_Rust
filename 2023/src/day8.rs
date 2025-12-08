// Day 8: Haunted Wasteland
//
// Follow left/right instructions through a network of nodes.
// Part 1: Count steps from AAA to ZZZ
// Part 2: Multiple starting nodes (**A), find LCM of cycle lengths to **Z

// Part 1 implementation
mod part1_impl {
    // A closure to encode a node name into a u32. Each character (byte) in the node name is reduced by 'A'
    // to make 'A' = 0, 'B' = 1, and so on, then shifted left by 10 or 5 bits or left as is, and combined.
    #[inline]
    pub fn enc(n: &[u8]) -> u32 {
        // Ensure characters are in valid range (A-Z is 0-25)
        let c0 = n[0].saturating_sub(b'A').min(25) as u32;
        let c1 = n[1].saturating_sub(b'A').min(25) as u32;
        let c2 = n[2].saturating_sub(b'A').min(25) as u32;
        (c0 << 10) | (c1 << 5) | c2
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    // Splitting the input into instructions and node mappings
    let mut sections = input.split("\n\n");
    let instructions = sections.next().unwrap().as_bytes(); // The first part are the instructions (L/R)
    let mappings = sections.next().unwrap(); // The second part is the node mapping

    // Preparing a map for node encoding. The size is based on encoding 3 characters into a unique u32.
    // Each character is encoded into 5 bits, so 3 characters need 3*5 = 15 bits.
    let mut map = [0u32; 0b11001_11001_11001 + 1]; // +1 because array indexing starts at 0

    // Process each line in the node mappings to fill the map
    for line in mappings.lines() {
        let parts: Vec<_> = line.split(" = ").map(|s| s.as_bytes()).collect();
        let encoded_node = part1_impl::enc(&parts[0][0..3]);
        map[encoded_node as usize] = part1_impl::enc(&parts[1][1..4]) | (part1_impl::enc(&parts[1][6..9]) << 16);
    }

    // Encoding the target node 'ZZZ'
    let zzz_encoded = part1_impl::enc(b"ZZZ");
    // Starting from 'AAA'
    let mut current_node = part1_impl::enc(b"AAA");
    let mut steps = 0; // Counting steps

    // Loop until the current node is 'ZZZ'
    loop {
        // Getting the next direction from instructions, cycling through them if necessary
        let direction = instructions[steps % instructions.len()];
        // Updating the current node based on the direction and the map
        current_node = if direction == b'L' {
            map[current_node as usize] & u16::MAX as u32 // Left node is in the lower 16 bits
        } else {
            map[current_node as usize] >> 16 // Right node is in the upper 16 bits
        };

        // If the target node 'ZZZ' is reached, exit the loop
        if current_node == zzz_encoded {
            break;
        }

        steps += 1; // Increment step count
    }

    steps + 1 // Returning the total number of steps (+1 because steps start at 0)
}

// Part 2 implementation
mod part2_impl {
    // A closure to encode a node name into a u32. Each character (byte) in the node name is reduced by 'A'
    // to make 'A' = 0, 'B' = 1, and so on, then shifted left by 10 or 5 bits or left as is, and combined.
    #[inline]
    pub fn enc(n: &[u8]) -> u32 {
        // Ensure characters are in valid range (A-Z is 0-25)
        let c0 = n[0].saturating_sub(b'A').min(25) as u32;
        let c1 = n[1].saturating_sub(b'A').min(25) as u32;
        let c2 = n[2].saturating_sub(b'A').min(25) as u32;
        (c0 << 10) | (c1 << 5) | c2
    }
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    // Splitting the input into instructions and node mappings
    let mut sections = input.split("\n\n");
    let instructions = sections.next().unwrap().as_bytes(); // The first part are the instructions (L/R)
    let mappings = sections.next().unwrap(); // The second part is the node mapping

    // Preparing a map for node encoding. The size is based on encoding 3 characters into a unique u32.
    // Each character is encoded into 5 bits, so 3 characters need 3*5 = 15 bits.
    let mut map = [0u32; 0b11001_11001_11001 + 1];
    let mut starts = Vec::with_capacity(6); // Capacity is 6 as an optimization, there are 6 starting nodes, but a vector is used to make this a general solution

    // Process each line in the node mappings to fill the map
    for line in mappings.lines() {
        let parts: Vec<_> = line.split(" = ").map(|s| s.as_bytes()).collect();
        let encoded_node = part2_impl::enc(&parts[0][0..3]);
        map[encoded_node as usize] = part2_impl::enc(&parts[1][1..4]) | (part2_impl::enc(&parts[1][6..9]) << 16);

        // If the node name ends with 'A', add it to the starting nodes
        if parts[0][2] == b'A' {
            starts.push(encoded_node);
        }
    }

    // Encode the ending character 'Z'
    let z_end_encoded = (b'Z' - b'A') as u32;
    let instructions_len = instructions.len(); // Cache the length of instructions for optimization

    // Process each starting node and calculate the steps to reach a node ending with 'Z'
    starts
        .into_iter()
        .map(|node| {
            let mut current_node = node;
            let mut steps = 0;

            // Loop until the current node ends with 'Z'
            loop {
                let direction = instructions[steps % instructions_len]; // Cycle through instructions
                current_node = if direction == b'L' {
                    map[current_node as usize] & u16::MAX as u32 // Get the left node
                } else {
                    map[current_node as usize] >> 16 // Get the right node
                };

                // Check if current node ends with 'Z'
                if current_node & 0b11111 == z_end_encoded {
                    break; // Exit loop when a node ending with 'Z' is reached
                }
                steps += 1; // Increment step count
            }

            steps + 1 // Return total steps for this path (+1 for 0 indexing)
        })
        // Calculate the least common multiple (LCM) of steps across all paths
        // This finds the first time all paths end with 'Z' simultaneously
        .fold(1, num_integer::lcm)
        // NOTE: This is not a generally correct solution,
        // this is expliting how the input is structured to have 6 independent paths
        // where each start has exactly 1 end and the Z nodes always have the same destinations
        // as the A nodes.
}
