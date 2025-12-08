#[aoc(day5, part1)]
pub fn part1(input: &str) -> u32 {
    const MAX_PAGE: usize = 100;
    const MAX_UPDATE_LEN: usize = 32;
    let mut before = [[0u64; 2]; MAX_PAGE];  // Use bits
    let mut sum = 0;
    
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();
    
    // Process rules
    for line in rules_str.lines() {
        let (x, y) = line.split_once('|').unwrap();
        let x: usize = x.parse().unwrap();
        let y: usize = y.parse().unwrap();
        before[x][y/64] |= 1 << (y%64);
    }
    
    // Build transitive closure in blocks
    for k in 0..MAX_PAGE {
        let mut changed = false;
        for i in 0..MAX_PAGE {
            if (before[i][k/64] & (1 << (k%64))) != 0 {
                for j_block in 0..2 {
                    let old = before[i][j_block];
                    before[i][j_block] |= before[k][j_block];
                    changed |= old != before[i][j_block];
                }
            }
        }
        if !changed { break; }
    }

    // Process updates with fixed array
    let mut update = [0usize; MAX_UPDATE_LEN];
    for line in updates_str.lines() {
        let mut len = 0;
        for n in line.split(',') {
            update[len] = n.parse().unwrap();
            len += 1;
        }
        
        let mut valid = true;
        'outer: for i in 0..len {
            for j in i+1..len {
                let a = update[j];
                let b = update[i];
                if (before[a][b/64] & (1 << (b%64))) != 0 {
                    valid = false;
                    break 'outer;
                }
            }
        }
        
        if valid {
            sum += update[len/2] as u32;
        }
    }
    
    sum
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> u32 {
    const MAX_PAGE: usize = 100;
    const MAX_UPDATE_LEN: usize = 32;
    let mut before = [[0u64; 2]; MAX_PAGE];
    let mut sum = 0;
    
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();
    
    // Process rules
    for line in rules_str.lines() {
        let (x, y) = line.split_once('|').unwrap();
        let x: usize = x.parse().unwrap();
        let y: usize = y.parse().unwrap();
        before[x][y/64] |= 1 << (y%64);
    }
    
    // Build transitive closure
    for k in 0..MAX_PAGE {
        let mut changed = false;
        for i in 0..MAX_PAGE {
            if (before[i][k/64] & (1 << (k%64))) != 0 {
                for j_block in 0..2 {
                    let old = before[i][j_block];
                    before[i][j_block] |= before[k][j_block];
                    changed |= old != before[i][j_block];
                }
            }
        }
        if !changed { break; }
    }

    // insertion sort for small arrays
    fn insertion_sort(arr: &mut [usize], len: usize, before: &[[u64; 2]; MAX_PAGE]) {
        for i in 1..len {
            let x = arr[i];
            let mut j = i;
            while j > 0 && (before[arr[j-1]][x/64] & (1 << (x%64))) != 0 {
                arr[j] = arr[j-1];
                j -= 1;
            }
            arr[j] = x;
        }
    }

    let mut update = [0usize; MAX_UPDATE_LEN];
    for line in updates_str.lines() {
        let mut len = 0;
        for n in line.split(',') {
            update[len] = n.parse().unwrap();
            len += 1;
        }
        
        let mut valid = true;
        for i in 0..len {
            for j in i+1..len {
                let a = update[j];
                let b = update[i];
                if (before[a][b/64] & (1 << (b%64))) != 0 {
                    valid = false;
                    break;
                }
            }
        }
        
        if !valid {
            insertion_sort(&mut update, len, &before);
            sum += update[len/2] as u32;
        }
    }
    
    sum
}