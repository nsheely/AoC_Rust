// Day 12: Christmas Tree Farm
//
// Part 1: Check if region has enough space
//   Red herring puzzle - just compare total areas!

#[derive(Clone)]
pub struct Problem {
    shape_areas: Vec<usize>,
    regions: Vec<Region>,
}

#[derive(Clone)]
struct Region {
    width: usize,
    height: usize,
    presents: Vec<usize>,
}

#[aoc_generator(day12)]
pub fn parse(input: &str) -> Problem {
    let mut shape_areas = Vec::new();
    let mut regions = Vec::new();
    let mut in_shape = false;
    let mut current_area = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            if in_shape && current_area > 0 {
                shape_areas.push(current_area);
                current_area = 0;
            }
            in_shape = false;
            continue;
        }

        if line.ends_with(':') {
            if current_area > 0 {
                shape_areas.push(current_area);
                current_area = 0;
            }
            in_shape = true;
        } else if let Some((dims, presents_str)) = line.split_once(':') {
            if let Some((w_str, h_str)) = dims.split_once('x') {
                regions.push(Region {
                    width: w_str.parse().unwrap(),
                    height: h_str.parse().unwrap(),
                    presents: presents_str
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect(),
                });
            }
        } else if in_shape {
            current_area += line.bytes().filter(|&b| b == b'#').count();
        }
    }

    if current_area > 0 {
        shape_areas.push(current_area);
    }

    Problem {
        shape_areas,
        regions,
    }
}

#[aoc(day12, part1)]
pub fn part1(problem: &Problem) -> usize {
    problem
        .regions
        .iter()
        .filter(|r| {
            let region_area = r.width * r.height;
            let needed: usize = r
                .presents
                .iter()
                .enumerate()
                .map(|(i, &count)| problem.shape_areas[i] * count)
                .sum();
            region_area > needed
        })
        .count()
}
