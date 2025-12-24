// Day 8: Playground
//
// Connect junction boxes in 3D space using a minimum spanning tree approach.
// Rather than generating all O(n²) edges, we use k-nearest neighbors to build a sparse graph.
//
// Part 1: Connect the 1000 closest pairs. Return product of the 3 largest connected components.
// Part 2: Continue connecting until fully connected. Return X-coordinate product of final edge.
//
// Algorithm: K-d tree for fast spatial queries + Union-Find for component tracking.

type Point = (i32, i32, i32);
type Output = Vec<Point>;

#[aoc_generator(day8)]
pub fn parse(input: &str) -> Output {
    input
        .lines()
        .map(|line| {
            let mut nums = line.split(',').map(|s| s.trim().parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap(), nums.next().unwrap())
        })
        .collect()
}

// Union-Find (disjoint set) for tracking connected components
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    #[inline]
    fn find(&mut self, mut x: usize) -> usize {
        // Path halving: point every other node to its grandparent
        while self.parent[x] != x {
            let next = self.parent[x];
            self.parent[x] = self.parent[next];
            x = next;
        }
        x
    }

    #[inline]
    fn union(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
        let (mut px, mut py) = (self.find(x), self.find(y));
        if px == py {
            return None;
        }

        // Attach smaller tree to larger (union by size)
        if self.size[px] < self.size[py] {
            (px, py) = (py, px);
        }
        self.parent[py] = px;
        self.size[px] += self.size[py];
        Some((px, self.size[px]))
    }

    fn top3_product(&mut self) -> usize {
        // Compress all paths, then find the 3 largest component sizes
        for i in 0..self.parent.len() {
            let _ = self.find(i);
        }

        let (mut a, mut b, mut c) = (0, 0, 0);
        for (i, &p) in self.parent.iter().enumerate() {
            if i == p {
                // This node is a root - check if it's in top 3
                let s = self.size[i];
                if s > a {
                    (a, b, c) = (s, a, b);
                } else if s > b {
                    (b, c) = (s, b);
                } else if s > c {
                    c = s;
                }
            }
        }
        a * b * c
    }
}

#[inline]
fn dist2(&(ax, ay, az): &Point, &(bx, by, bz): &Point) -> i64 {
    let (dx, dy, dz) = ((ax - bx) as i64, (ay - by) as i64, (az - bz) as i64);
    dx * dx + dy * dy + dz * dz
}

#[inline(always)]
fn axis_value(p: &Point, axis: usize) -> i32 {
    match axis {
        0 => p.0,
        1 => p.1,
        _ => p.2,
    }
}

// K-d tree for efficient 3D nearest neighbor search
struct KdNode {
    point: Point,
    idx: usize,
    left: Option<Box<KdNode>>,
    right: Option<Box<KdNode>>,
}

// Track k-nearest neighbors using a fixed-size array
// Maintains invariant: pairs[..len] sorted by distance (descending, worst first)
struct KNearest {
    pairs: [(i64, usize); 12],
    len: usize,
    capacity: usize,
}

impl KNearest {
    const MAX_K: usize = 12;

    fn new(k: usize) -> Self {
        assert!(k <= Self::MAX_K, "k must be <= {}", Self::MAX_K);
        Self {
            pairs: [(i64::MAX, 0); Self::MAX_K],
            len: 0,
            capacity: k,
        }
    }

    #[inline]
    fn try_insert(&mut self, dist: i64, idx: usize) {
        if self.len >= self.capacity && dist >= self.pairs[0].0 {
            return; // Full and this distance isn't better than worst
        }

        if self.len < self.capacity {
            // Insert in sorted position, maintaining descending order
            let mut pos = self.len;
            while pos > 0 && self.pairs[pos - 1].0 < dist {
                pos -= 1;
            }
            self.pairs.copy_within(pos..self.len, pos + 1);
            self.pairs[pos] = (dist, idx);
            self.len += 1;
        } else {
            // Replace worst (index 0) and bubble to maintain order
            let mut pos = 0;
            while pos < self.len - 1 && self.pairs[pos + 1].0 > dist {
                self.pairs[pos] = self.pairs[pos + 1];
                pos += 1;
            }
            self.pairs[pos] = (dist, idx);
        }
    }

    #[inline]
    fn max_dist(&self) -> i64 {
        if self.len < self.capacity {
            i64::MAX
        } else {
            self.pairs[0].0
        }
    }

    fn as_slice(&self) -> &[(i64, usize)] {
        &self.pairs[..self.len]
    }
}

impl KdNode {
    fn build(points: &mut [(usize, Point)], depth: usize) -> Option<Box<KdNode>> {
        if points.is_empty() {
            return None;
        }

        let axis = depth % 3;
        let mid = points.len() / 2;

        // Partition around median (linear time)
        points.select_nth_unstable_by_key(mid, |(_, p)| axis_value(p, axis));

        let (idx, point) = points[mid];

        Some(Box::new(KdNode {
            point,
            idx,
            left: Self::build(&mut points[..mid], depth + 1),
            right: Self::build(&mut points[mid + 1..], depth + 1),
        }))
    }

    fn k_nearest(
        &self,
        target: &Point,
        target_idx: usize,
        depth: usize,
        best: &mut KNearest,
    ) {
        if self.idx != target_idx {
            let dist = dist2(target, &self.point);
            if dist < best.max_dist() {
                best.try_insert(dist, self.idx);
            }
        }

        // Compute axis difference for pruning decision
        let axis = depth % 3;
        let diff = match axis {
            0 => (target.0 - self.point.0) as i64,
            1 => (target.1 - self.point.1) as i64,
            _ => (target.2 - self.point.2) as i64,
        };

        // Search near side first (more likely to contain closer points)
        let (near, far) = if diff < 0 {
            (&self.left, &self.right)
        } else {
            (&self.right, &self.left)
        };

        if let Some(n) = near {
            n.k_nearest(target, target_idx, depth + 1, best);
        }

        // Only search far side if it could contain closer points
        let thresh = best.max_dist();
        if diff * diff < thresh {
            if let Some(f) = far {
                f.k_nearest(target, target_idx, depth + 1, best);
            }
        }
    }
}

// Build sparse graph from k-nearest neighbors
fn collect_knn_edges(junctions: &Output, k: usize) -> Vec<(i64, usize, usize)> {
    let mut indexed: Vec<_> = junctions.iter().copied().enumerate().collect();
    let tree = KdNode::build(&mut indexed, 0).unwrap();

    let mut edges = Vec::with_capacity(junctions.len() * k);
    for (i, junction) in junctions.iter().enumerate() {
        let mut knn = KNearest::new(k);
        tree.k_nearest(junction, i, 0, &mut knn);

        for &(d, j) in knn.as_slice() {
            if i < j {
                edges.push((d, i, j));
            }
        }
    }

    edges.sort_unstable_by_key(|e| e.0);
    edges
}

#[aoc(day8, part1)]
pub fn part1(junctions: &Output) -> usize {
    let mut edges = collect_knn_edges(junctions, 5);

    // Partition to find 1000 smallest edges (linear time)
    let m = edges.len().min(1000);
    edges.select_nth_unstable_by_key(m - 1, |e| e.0);
    edges.truncate(m);

    let mut uf = UnionFind::new(junctions.len());
    for &(_, i, j) in &edges {
        let _ = uf.union(i, j);
    }

    uf.top3_product()
}

#[aoc(day8, part2)]
pub fn part2(junctions: &Output) -> usize {
    let edges = collect_knn_edges(junctions, 10);
    let mut uf = UnionFind::new(junctions.len());
    let target = junctions.len();

    for (_, i, j) in edges {
        if let Some((_, size)) = uf.union(i, j) {
            if size == target {
                return junctions[i].0 as usize * junctions[j].0 as usize;
            }
        }
    }
    unreachable!()
}
