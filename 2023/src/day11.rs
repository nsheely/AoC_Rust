// Day 11: Cosmic Expansion
//
// Calculate shortest paths between galaxies in an expanding universe.
// Empty rows and columns expand by a given factor (2 for part 1, 1M for part 2).
// Uses Manhattan distance with expansion offsets.

// Parse the input string into a 2D grid representation.
// Each line of the input becomes a vector of characters.
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

// Count the number of empty rows and columns before each index in the grid.
// This information is used to adjust the galaxy coordinates for cosmic expansion.
fn count_empty_rows_cols(grid: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut empty_rows = vec![0; rows];
    let mut empty_cols = vec![0; cols];

    // Count empty rows
    for i in 1..rows {
        empty_rows[i] = empty_rows[i - 1]
            + if grid[i - 1].iter().all(|&c| c == '.') { 1 } else { 0 };
    }

    // Count empty columns
    for j in 1..cols {
        empty_cols[j] = empty_cols[j - 1]
            + if grid.iter().all(|row| row[j] == '.') { 1 } else { 0 };
    }

    (empty_rows, empty_cols)
}

// Calculate the total Manhattan distance between all unique galaxy pairs.
// The Manhattan distance is the sum of the absolute differences in their respective coordinates.
fn calculate_total_distance(galaxies: Vec<(usize, usize)>) -> usize {
    let mut total_distance = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (x1, y1) = galaxies[i];
            let (x2, y2) = galaxies[j];
            total_distance += (x1.max(x2) - x1.min(x2)) + (y1.max(y2) - y1.min(y2));
        }
    }
    total_distance
}

mod part1_impl {
    // Adjust the coordinates of each galaxy based on the cosmic expansion.
    // The coordinates are increased by the count of empty rows and columns before them.
    pub fn adjust_galaxy_coordinates(
        grid: &[Vec<char>],
        empty_rows: &[usize],
        empty_cols: &[usize],
    ) -> Vec<(usize, usize)> {
        grid.iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().filter_map(move |(j, &c)| {
                    if c == '#' {
                        Some((i + empty_rows[i], j + empty_cols[j]))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    let (empty_rows, empty_cols) = count_empty_rows_cols(&grid);
    let galaxies = part1_impl::adjust_galaxy_coordinates(&grid, &empty_rows, &empty_cols);
    calculate_total_distance(galaxies)
}

mod part2_impl {
    // Adjust the coordinates of each galaxy based on the cosmic expansion.
    // The coordinates are increased by the count of empty rows and columns before them.
    pub fn adjust_galaxy_coordinates(
        grid: &[Vec<char>],
        empty_rows: &[usize],
        empty_cols: &[usize],
    ) -> Vec<(usize, usize)> {
        grid.iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().filter_map(move |(j, &c)| {
                    if c == '#' {
                        Some((i + empty_rows[i] * 999_999, j + empty_cols[j] * 999_999))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    let (empty_rows, empty_cols) = count_empty_rows_cols(&grid);
    let galaxies = part2_impl::adjust_galaxy_coordinates(&grid, &empty_rows, &empty_cols);
    calculate_total_distance(galaxies)
}
