// Day 9: Movie Theater
// Part 1: Find max rectangle using convex hull corners paired with all tiles
// Part 2: Scanline sweep tracking valid interior intervals for rectangle corners

type Point = (i32, i32);

#[inline(always)]
fn rect_area((x1, y1): Point, (x2, y2): Point) -> i64 {
    ((x1 - x2).unsigned_abs() as i64 + 1) * ((y1 - y2).unsigned_abs() as i64 + 1)
}

#[inline]
fn cross(o: Point, a: Point, b: Point) -> i64 {
    ((a.0 - o.0) as i64) * ((b.1 - o.1) as i64) - ((a.1 - o.1) as i64) * ((b.0 - o.0) as i64)
}

fn convex_hull(points: &[Point]) -> Vec<Point> {
    let mut pts = points.to_vec();
    pts.sort_unstable();
    pts.dedup();
    if pts.len() <= 3 {
        return pts;
    }

    fn build_half<'a, I: Iterator<Item = &'a Point>>(iter: I, cap: usize) -> Vec<Point> {
        let mut hull = Vec::with_capacity(cap);
        for &p in iter {
            // <= 0 drops collinear points for a strict hull
            while hull.len() >= 2 && cross(hull[hull.len() - 2], hull[hull.len() - 1], p) <= 0 {
                hull.pop();
            }
            hull.push(p);
        }
        hull
    }

    let cap = pts.len();
    let mut lower = build_half(pts.iter(), cap);
    let mut upper = build_half(pts.iter().rev(), cap);
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

#[aoc_generator(day9)]
pub fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.is_empty() {
                return None;
            }
            let (a, b) = l.split_once(',')?;
            Some((a.trim().parse().ok()?, b.trim().parse().ok()?))
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(tiles: &[Point]) -> i64 {
    let hull = convex_hull(tiles);
    let mut best = 0i64;
    for &h in &hull {
        for &t in tiles {
            best = best.max(rect_area(h, t));
        }
    }
    best
}

#[derive(Clone, Copy)]
struct Interval {
    l: i32,
    r: i32,
}

impl Interval {
    #[inline]
    fn contains(self, x: i32) -> bool {
        self.l <= x && x <= self.r
    }

    #[inline]
    fn intersect(self, other: Interval) -> Option<Interval> {
        let l = self.l.max(other.l);
        let r = self.r.min(other.r);
        (l <= r).then_some(Interval { l, r })
    }
}

#[inline(always)]
fn find_interval(intervals: &[Interval], x: i32) -> Option<usize> {
    let idx = intervals.partition_point(|it| it.l <= x);
    (idx != 0 && intervals[idx - 1].contains(x)).then_some(idx - 1)
}

#[inline]
fn toggle_sorted(xs: &mut Vec<i32>, x: i32) {
    match xs.binary_search(&x) {
        Ok(i) => {
            xs.remove(i);
        }
        Err(i) => {
            xs.insert(i, x);
        }
    }
}

#[inline]
fn rebuild_intervals(edges: &[i32], out: &mut Vec<Interval>) {
    out.clear();
    for pair in edges.chunks_exact(2) {
        out.push(Interval {
            l: pair[0],
            r: pair[1],
        });
    }
}

#[derive(Clone, Copy)]
struct Candidate {
    x: i32,
    y: i32,
    interval: Interval,
}

#[aoc(day9, part2)]
pub fn part2(input: &[Point]) -> i64 {
    let mut tiles = input.to_vec();
    tiles.sort_unstable_by_key(|&(x, y)| (y, x));

    let mut best = 0i64;
    let mut candidates: Vec<Candidate> = Vec::with_capacity(tiles.len() / 4);
    let mut edges: Vec<i32> = Vec::with_capacity(64);
    let mut intervals: Vec<Interval> = Vec::with_capacity(32);
    let mut row_xs: Vec<i32> = Vec::with_capacity(32);

    // Scanline state:
    //   edges: sorted x positions where boundary crosses this y (parity toggles)
    //   intervals: interior x-spans derived from edge pairs
    //   candidates: top corners from previous rows, with shrinking valid x-ranges

    let mut i = 0;
    while i < tiles.len() {
        let y = tiles[i].1;
        row_xs.clear();

        // 1) Collect unique x values and toggle edges
        // (row_xs ends up sorted unique since tiles is sorted by (y,x))
        while i < tiles.len() && tiles[i].1 == y {
            let x = tiles[i].0;
            if row_xs.last().copied() != Some(x) {
                row_xs.push(x);
                toggle_sorted(&mut edges, x);
            }
            i += 1;
        }

        // 2) Rebuild intervals from edge pairs
        rebuild_intervals(&edges, &mut intervals);

        // 3) Check each candidate against current row's x positions
        match row_xs.len() {
            1 => {
                let x = row_xs[0];
                for &c in &candidates {
                    if c.interval.contains(x) {
                        best = best.max(rect_area((c.x, c.y), (x, y)));
                    }
                }
            }
            2 => {
                let (x1, x2) = (row_xs[0], row_xs[1]);
                for &c in &candidates {
                    if c.interval.contains(x1) {
                        best = best.max(rect_area((c.x, c.y), (x1, y)));
                    }
                    if c.interval.contains(x2) {
                        best = best.max(rect_area((c.x, c.y), (x2, y)));
                    }
                }
            }
            _ if !row_xs.is_empty() => {
                // For many x's, only check extremes (leftmost/rightmost in valid range maximize width)
                for &c in &candidates {
                    let lo = row_xs.partition_point(|&x| x < c.interval.l);
                    let hi = row_xs.partition_point(|&x| x <= c.interval.r);
                    if lo < hi {
                        let (xl, xr) = (row_xs[lo], row_xs[hi - 1]);
                        best = best.max(rect_area((c.x, c.y), (xl, y)));
                        if xr != xl {
                            best = best.max(rect_area((c.x, c.y), (xr, y)));
                        }
                    }
                }
            }
            _ => {}
        }

        // 4) Keep candidates still inside intervals, updating their valid x-ranges
        candidates.retain_mut(|c| {
            if let Some(k) = find_interval(&intervals, c.x) {
                if let Some(int) = c.interval.intersect(intervals[k]) {
                    c.interval = int;
                    return true;
                }
            }
            false
        });

        // 5) Create new candidates from boundary x positions on this row
        for &x in &row_xs {
            if let Some(k) = find_interval(&intervals, x) {
                candidates.push(Candidate {
                    x,
                    y,
                    interval: intervals[k],
                });
            }
        }
    }

    best
}
