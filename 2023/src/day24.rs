// Day 24: Never Tell Me The Odds
//
// Part 1: Find pairs of hailstones whose paths intersect in XY plane within range
// Part 2: Find a rock trajectory that will hit all hailstones
//
// Uses 2D line intersection (part 1) and 3D vector geometry (part 2)

use num_integer::Integer;
use rayon::prelude::*;

const RANGE_MIN: f64 = 200_000_000_000_000.0;
const RANGE_MAX: f64 = 400_000_000_000_000.0;

#[derive(Clone, Copy)]
struct Vector {
    x: i128,
    y: i128,
    z: i128,
}

impl Vector {
    fn from_i64(x: i64, y: i64, z: i64) -> Self {
        Vector {
            x: x as i128,
            y: y as i128,
            z: z as i128,
        }
    }

    fn cross(self, other: Self) -> Self {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn gcd(self) -> Self {
        let gcd = self.x.gcd(&self.y).gcd(&self.z);
        Vector {
            x: self.x / gcd,
            y: self.y / gcd,
            z: self.z / gcd,
        }
    }
}

impl std::ops::Sub for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[aoc_generator(day24)]
pub fn parse_input(input: &str) -> Vec<[i64; 6]> {
    input
        .lines()
        .map(|line| {
            let nums: Vec<i64> = line
                .split(&[',', '@'])
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            [nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]]
        })
        .collect()
}

#[aoc(day24, part1)]
pub fn part1(hail: &[[i64; 6]]) -> usize {
    (0..hail.len())
        .into_par_iter()
        .map(|i| {
            let mut count = 0;
            for j in (i + 1)..hail.len() {
                let [a, b, _, c, d, _] = hail[i].map(|v| v as f64);
                let [e, f, _, g, h, _] = hail[j].map(|v| v as f64);

                let determinant = d * g - c * h;
                if determinant == 0.0 {
                    continue;
                }

                let t = (g * (f - b) - h * (e - a)) / determinant;
                let u = (c * (f - b) - d * (e - a)) / determinant;

                if t < 0.0 || u < 0.0 {
                    continue;
                }

                let x = a + t * c;
                let y = b + t * d;

                if x >= RANGE_MIN && x <= RANGE_MAX && y >= RANGE_MIN && y <= RANGE_MAX {
                    count += 1;
                }
            }
            count
        })
        .sum()
}

#[aoc(day24, part2)]
pub fn part2(hail: &[[i64; 6]]) -> i128 {
    let h0 = hail[0];
    let h1 = hail[1];
    let h2 = hail[2];

    let p0 = Vector::from_i64(h0[0], h0[1], h0[2]);
    let v0 = Vector::from_i64(h0[3], h0[4], h0[5]);

    let p1 = Vector::from_i64(h1[0], h1[1], h1[2]);
    let v1 = Vector::from_i64(h1[3], h1[4], h1[5]);

    let p2 = Vector::from_i64(h2[0], h2[1], h2[2]);
    let v2 = Vector::from_i64(h2[3], h2[4], h2[5]);

    // Relativize to first hailstone
    let p3 = p1 - p0;
    let v3 = v1 - v0;
    let p4 = p2 - p0;
    let v4 = v2 - v0;

    // Find rock direction via plane intersection
    let n3 = v3.cross(p3);
    let n4 = v4.cross(p4);
    let s = n3.cross(n4).gcd();

    // Find collision times
    let t = (p3.y * s.x - p3.x * s.y) / (v3.x * s.y - v3.y * s.x);
    let u = (p4.y * s.x - p4.x * s.y) / (v4.x * s.y - v4.y * s.x);

    // Calculate rock's initial position
    let a = h1[0] as i128 + h1[1] as i128 + h1[2] as i128;
    let b = h2[0] as i128 + h2[1] as i128 + h2[2] as i128;
    let c = (v3 - v4).x + (v3 - v4).y + (v3 - v4).z;

    (u * a - t * b + u * t * c) / (u - t)
}
