// Day 10: Factory
//
// Part 1: Configure binary indicator lights (toggle on/off)
//   GF(2): pressing button twice = no effect, find min presses
//   BFS by popcount with bitmask XOR
//
// Part 2: Integer system Ax=b with x>=0 minimized by Σx
//   Exact integer row-reduction (exploits AoC structure), then branch-and-bound over free vars

pub struct Machine {
    target_mask: u64,
    button_masks: Vec<u64>,
    button_deltas: Vec<Vec<usize>>,
    joltage: Vec<u64>,
}

#[aoc_generator(day10)]
pub fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            // Parse target state [.##.]
            let start = line.find('[').unwrap() + 1;
            let end = line.find(']').unwrap();
            let target_mask = line[start..end]
                .chars()
                .enumerate()
                .fold(0u64, |mask, (i, c)| mask | ((c == '#') as u64) << i);

            // Parse buttons (1,3) (2) etc.
            let mut button_deltas = Vec::new();
            let mut in_paren = false;
            let mut current = String::new();

            for c in line[end..].chars() {
                if c == '(' {
                    in_paren = true;
                    current.clear();
                } else if c == ')' {
                    in_paren = false;
                    let indices: Vec<usize> = current
                        .split(',')
                        .filter_map(|s| s.trim().parse().ok())
                        .collect();
                    if !indices.is_empty() {
                        button_deltas.push(indices);
                    }
                } else if in_paren {
                    current.push(c);
                }
            }

            // Convert button deltas to bitmasks for Part 1
            let button_masks: Vec<u64> = button_deltas
                .iter()
                .map(|indices| indices.iter().fold(0u64, |mask, &i| mask | (1 << i)))
                .collect();

            // Parse joltage requirements {3,5,4,7}
            let joltage = if let Some(brace_start) = line.find('{') {
                if let Some(brace_end) = line.find('}') {
                    line[brace_start + 1..brace_end]
                        .split(',')
                        .filter_map(|s| s.trim().parse().ok())
                        .collect()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            };

            Machine {
                target_mask,
                button_masks,
                button_deltas,
                joltage,
            }
        })
        .collect()
}

// Generate next mask with same popcount (Gosper's hack)
#[inline]
fn next_combination(v: u64) -> u64 {
    let t = v | (v - 1);
    (t + 1) | (((!t & (t + 1)) - 1) >> (v.trailing_zeros() + 1))
}

// XOR all masks indicated by set bits in m
#[inline(always)]
fn xor_masks(mut m: u64, masks: &[u64]) -> u64 {
    let mut state = 0u64;
    while m != 0 {
        let i = m.trailing_zeros() as usize;
        state ^= masks[i];
        m &= m - 1;
    }
    state
}

// Ceiling division for all signs (correct for Rust's truncate-toward-zero)
#[inline(always)]
fn div_ceil(n: i64, d: i64) -> i64 {
    debug_assert!(d != 0, "division by zero");
    let q = n / d;
    let r = n % d;
    if r == 0 {
        q
    } else if (r > 0) == (d > 0) {
        q + 1 // same sign => round up
    } else {
        q
    }
}

// Compute bounds for the last free variable
// Returns None if constraints are unsatisfiable
#[inline(always)]
fn bounds_last_var(
    coeff: &[i64], // Coefficients for this variable
    rhs: &[i64],   // Current RHS values
    fixed: usize,  // First equality constraint row
    init_upper: i64,
) -> Option<(i64, i64)> {
    let mut lower = 0;
    let mut upper = init_upper;

    // Check inequality constraints: a*x <= rhs (rows 0..fixed)
    for row in 0..fixed {
        let a = coeff[row];
        let r = rhs[row];

        // Fast paths for common coefficient values
        if a == 1 {
            upper = upper.min(r);
        } else if a == -1 {
            // Avoid overflow: r.checked_neg() handles i64::MIN safely
            lower = lower.max(r.checked_neg().unwrap_or(i64::MAX));
        } else if a == 0 {
            if r < 0 {
                return None; // 0*x <= r with r < 0: impossible
            }
        } else if a > 0 {
            // x <= r/a (floor division gives upper bound)
            if r < 0 {
                return None; // a > 0 and r < 0 => infeasible
            }
            upper = upper.min(r / a);
        } else {
            // a < 0: a*x <= r => x >= r/a (ceiling division gives lower bound)
            lower = lower.max(div_ceil(r, a));
        }
    }

    // Check equality constraints: a*x = rhs (rows fixed..height, exact division required)
    for row in fixed..rhs.len() {
        let a = coeff[row];
        let r = rhs[row];

        if a != 0 {
            if r % a == 0 {
                let v = r / a;
                lower = lower.max(v);
                upper = upper.min(v);
            } else {
                return None; // Not divisible
            }
        } else if r != 0 {
            return None; // a==0 but r!=0: impossible
        }
        // a == 0 && r == 0: no constraint from this row
    }

    (lower <= upper).then_some((lower, upper))
}

// Solve GF(2) system using BFS by popcount with bitmask XOR
fn solve_gf2(machine: &Machine) -> usize {
    if machine.target_mask == 0 {
        return 0;
    }

    let n_buttons = machine.button_masks.len();
    if n_buttons == 0 || n_buttons > 63 {
        return usize::MAX;
    }

    // BFS by popcount: try k=1, k=2, ... button presses
    for num_presses in 1..=n_buttons {
        let mut mask = (1u64 << num_presses) - 1;
        let limit = 1u64 << n_buttons;

        while mask < limit {
            if xor_masks(mask, &machine.button_masks) == machine.target_mask {
                return num_presses;
            }
            mask = next_combination(mask);
        }
    }

    usize::MAX
}

// Build equation system from machine, filtering dead columns
fn build_system(machine: &Machine) -> (Vec<i64>, Vec<i64>, usize, usize) {
    let height = machine.joltage.len();

    // Filter dead columns (buttons affecting no counters)
    let keep_cols: Vec<usize> = machine
        .button_deltas
        .iter()
        .enumerate()
        .filter_map(|(col, deltas)| deltas.iter().any(|&row| row < height).then_some(col))
        .collect();

    let width = keep_cols.len();
    if width == 0 {
        return (Vec::new(), Vec::new(), 0, height);
    }

    // Build flat equation matrix: [a_0, a_1, ..., a_{width-1}, rhs] per row
    let stride = width + 1;
    let mut equations = vec![0i64; height * stride];
    let mut limit = vec![i64::MAX; width];

    for (new_col, &old_col) in keep_cols.iter().enumerate() {
        for &row in &machine.button_deltas[old_col] {
            if row < height {
                equations[row * stride + new_col] = 1;
                limit[new_col] = limit[new_col].min(machine.joltage[row] as i64);
            }
        }
    }

    for (row, &joltage) in machine.joltage.iter().enumerate() {
        equations[row * stride + width] = joltage as i64;
    }

    (equations, limit, width, height)
}

// Perform integer row reduction to identify free variables
fn rref_integer(equations: &mut [i64], width: usize, height: usize) -> Vec<usize> {
    let stride = width + 1;
    let mut is_pivot = vec![false; width];
    let mut pivot_row = 0;
    let mut pivot_col = 0;

    while pivot_row < height && pivot_col < width {
        // Find pivot: prefer abs(coef)==1 (no division needed), else smallest abs nonzero
        let mut best_row = None;
        let mut best_abs = i64::MAX;

        for row in pivot_row..height {
            let coef = equations[row * stride + pivot_col];
            if coef == 0 {
                continue;
            }

            let abs_coef = coef.abs();

            // Short-circuit: abs==1 always divides cleanly, prefer it
            if abs_coef == 1 {
                best_row = Some(row);
                break;
            }

            // Check if this row can be cleanly divided by coef
            let divisible = (0..=width).all(|c| equations[row * stride + c] % coef == 0);
            if !divisible {
                continue;
            }

            // Prefer smaller abs for simpler arithmetic
            if abs_coef < best_abs {
                best_row = Some(row);
                best_abs = abs_coef;
            }
        }

        let Some(found) = best_row else {
            pivot_col += 1;
            continue;
        };

        // Swap rows if needed
        if found != pivot_row {
            for col in 0..=width {
                equations.swap(pivot_row * stride + col, found * stride + col);
            }
        }

        let coef = equations[pivot_row * stride + pivot_col];

        // Divide pivot row by coef
        for col in 0..=width {
            equations[pivot_row * stride + col] /= coef;
        }

        // Eliminate column from other rows
        for row in 0..height {
            if row != pivot_row {
                let coef = equations[row * stride + pivot_col];
                for col in 0..=width {
                    let val = equations[pivot_row * stride + col];
                    equations[row * stride + col] -= coef * val;
                }
            }
        }

        is_pivot[pivot_col] = true;
        pivot_row += 1;
        // We pivot each column at most once; move to next column after successful pivot
        pivot_col += 1;
    }

    // Return free variables (non-pivot columns)
    (0..width).filter(|&c| !is_pivot[c]).collect()
}

// Solve the system after row reduction
fn solve_from_rref(
    equations: &[i64],
    limit: &[i64],
    free_vars: &[usize],
    width: usize,
    height: usize,
) -> usize {
    let stride = width + 1;
    let free = free_vars.len();

    // No free variables: unique solution
    if free == 0 {
        let sum: i64 = (0..height).map(|row| equations[row * stride + width]).sum();
        return sum as usize;
    }

    let fixed = width - free;
    let base_presses: i64 = (0..fixed).map(|row| equations[row * stride + width]).sum();

    // ===== Fast path: single free variable =====
    if free == 1 {
        let col = free_vars[0];
        let cost = 1
            - (0..fixed)
                .map(|row| equations[row * stride + col])
                .sum::<i64>();
        let mut lower = 0;
        let mut upper = limit[col];

        // Check inequalities: a*x <= rhs (rows 0..fixed)
        for row in 0..fixed {
            let a = equations[row * stride + col];
            let r = equations[row * stride + width];

            // Fast paths for common coefficients
            if a == 1 {
                upper = upper.min(r);
            } else if a == -1 {
                lower = lower.max(r.checked_neg().unwrap_or(i64::MAX));
            } else if a == 0 {
                if r < 0 {
                    return usize::MAX;
                }
            } else if a > 0 {
                if r < 0 {
                    return usize::MAX;
                }
                upper = upper.min(r / a);
            } else {
                lower = lower.max(div_ceil(r, a));
            }
        }

        // Check equalities: a*x = rhs (rows fixed..height, exact division required)
        for row in fixed..height {
            let a = equations[row * stride + col];
            let r = equations[row * stride + width];

            if a != 0 {
                if r % a == 0 {
                    let val = r / a;
                    lower = lower.max(val);
                    upper = upper.min(val);
                } else {
                    return usize::MAX;
                }
            } else if r != 0 {
                return usize::MAX;
            }
        }

        if lower <= upper {
            let x = if cost >= 0 { lower } else { upper };
            return (base_presses + x * cost) as usize;
        }
        return usize::MAX;
    }

    // ===== General case: multiple free variables =====
    // Build variable info and sort by (limit, -impact) for optimal branching
    let mut var_info = Vec::with_capacity(free);
    for &from in free_vars {
        let mut c = 1i64;
        for row in 0..fixed {
            c -= equations[row * stride + from];
        }

        let lim = limit[from];

        // Impact: sum of absolute coefficients (focus on equality rows for tighter constraints)
        let mut impact = 0i64;
        for row in fixed..height {
            impact += equations[row * stride + from].abs();
        }

        var_info.push((from, c, lim, impact));
    }

    // Sort by (limit, -impact): small limit first, then high impact
    var_info.sort_by_key(|(_, _, lim, impact)| (*lim, -impact));

    let cost: Vec<i64> = var_info.iter().map(|(_, c, _, _)| *c).collect();
    let ordered_limit: Vec<i64> = var_info.iter().map(|(_, _, lim, _)| *lim).collect();

    // Flatten coefficients for better cache locality (using reordered variables)
    let mut coeff_flat = vec![0i64; free * height];
    for (d, &(from, _, _, _)) in var_info.iter().enumerate() {
        for row in 0..height {
            coeff_flat[d * height + row] = equations[row * stride + from];
        }
    }

    // Precompute suffix optimistic bounds: suffix_opt[d] = best possible from vars d..end
    let mut suffix_opt = vec![0i64; free + 1];
    for d in (0..free).rev() {
        suffix_opt[d] = suffix_opt[d + 1];
        if cost[d] < 0 {
            suffix_opt[d] += cost[d] * ordered_limit[d];
        }
    }

    // Preallocate scratch space for all rhs vectors
    let mut rhs_scratch = vec![0i64; (free + 1) * height];

    // Initialize first rhs
    for row in 0..height {
        rhs_scratch[row] = equations[row * stride + width];
    }

    let mut best = i64::MAX;
    recurse_alloc_free(
        &cost,
        &ordered_limit,
        &coeff_flat,
        &mut rhs_scratch,
        &suffix_opt,
        height,
        fixed,
        base_presses,
        0,
        &mut best,
    );

    if best == i64::MAX {
        usize::MAX
    } else {
        best as usize
    }
}

// Allocation-free recursion with scratch buffer
#[inline]
#[allow(clippy::too_many_arguments)]
fn recurse_alloc_free(
    cost: &[i64],
    limit: &[i64],
    coeff: &[i64],           // Flattened: coeff[depth*height + row]
    rhs_scratch: &mut [i64], // len = (free+1)*height
    suffix_opt: &[i64],      // Precomputed optimistic bounds from each depth
    height: usize,
    fixed: usize,
    presses: i64,
    depth: usize,
    best: &mut i64,
) {
    // Current rhs is at rhs_scratch[depth*height..(depth+1)*height]
    let rhs_offset = depth * height;
    let coeff_offset = depth * height;

    // Last variable: compute directly
    if depth + 1 == cost.len() {
        let a = &coeff[coeff_offset..coeff_offset + height];
        let rhs = &rhs_scratch[rhs_offset..rhs_offset + height];

        let Some((lower, upper)) = bounds_last_var(a, rhs, fixed, limit[depth]) else {
            return;
        };

        let x = if cost[depth] >= 0 { lower } else { upper };
        let total = presses + cost[depth] * x;
        if total < *best {
            *best = total;
        }
        return;
    }

    // Node-level pruning: check if entire subtree can be pruned
    if presses + suffix_opt[depth] >= *best {
        return;
    }

    // Recursive case: try all values
    let next_rhs_offset = (depth + 1) * height;
    let remaining_optimistic = suffix_opt[depth + 1];
    let lim = limit[depth];

    // Split into forward/reverse to avoid branching inside loop
    if cost[depth] >= 0 {
        // Positive cost: iterate forward (0..=lim), early exit on prune
        for x in 0..=lim {
            let next_presses = presses + cost[depth] * x;

            // Prune if even the best-case from here can't beat current best
            if next_presses + remaining_optimistic >= *best {
                break;
            }

            // Compute next rhs in-place (no allocation)
            for row in 0..height {
                rhs_scratch[next_rhs_offset + row] =
                    rhs_scratch[rhs_offset + row] - x * coeff[coeff_offset + row];
            }

            recurse_alloc_free(
                cost,
                limit,
                coeff,
                rhs_scratch,
                suffix_opt,
                height,
                fixed,
                next_presses,
                depth + 1,
                best,
            );
        }
    } else {
        // Negative cost: iterate backward (lim..=0), continue on prune
        for x in (0..=lim).rev() {
            let next_presses = presses + cost[depth] * x;

            // Prune if even the best-case from here can't beat current best
            if next_presses + remaining_optimistic >= *best {
                continue;
            }

            // Compute next rhs in-place (no allocation)
            for row in 0..height {
                rhs_scratch[next_rhs_offset + row] =
                    rhs_scratch[rhs_offset + row] - x * coeff[coeff_offset + row];
            }

            recurse_alloc_free(
                cost,
                limit,
                coeff,
                rhs_scratch,
                suffix_opt,
                height,
                fixed,
                next_presses,
                depth + 1,
                best,
            );
        }
    }
}

// Main solver: build system, reduce, and solve
fn solve_diophantine(machine: &Machine) -> usize {
    // Early exit
    if machine.joltage.is_empty() || machine.joltage.iter().all(|&v| v == 0) {
        return 0;
    }

    // Build equation system
    let (mut equations, limit, width, height) = build_system(machine);

    if width == 0 {
        return if machine.joltage.iter().all(|&v| v == 0) {
            0
        } else {
            usize::MAX
        };
    }

    // Row reduce to identify free variables
    let free_vars = rref_integer(&mut equations, width, height);

    // Solve the reduced system
    solve_from_rref(&equations, &limit, &free_vars, width, height)
}

#[aoc(day10, part1)]
pub fn part1(machines: &[Machine]) -> usize {
    use rayon::prelude::*;
    machines.par_iter().map(solve_gf2).sum()
}

#[aoc(day10, part2)]
pub fn part2(machines: &[Machine]) -> usize {
    use rayon::prelude::*;
    machines.par_iter().map(solve_diophantine).sum()
}
