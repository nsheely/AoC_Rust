// Day 25: Snowverload
//
// Find 3 wires to disconnect to split the component network into two groups.
// This is a graph min-cut problem where the cut size is exactly 3.
//
// Strategy: Use edge betweenness - edges connecting the two groups will be
// on many shortest paths. Count how often each edge is used in shortest paths
// between random pairs, then remove the top 3.

use std::collections::VecDeque;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

type Graph = HashMap<String, Vec<String>>;

#[aoc_generator(day25)]
pub fn parse_input(input: &str) -> Graph {
    let mut graph = HashMap::default();

    for line in input.lines() {
        let (left, right) = line.split_once(": ").unwrap();
        let neighbors: Vec<&str> = right.split_whitespace().collect();

        // Add edges in both directions (undirected graph)
        for neighbor in neighbors {
            graph.entry(left.to_string())
                .or_insert_with(Vec::new)
                .push(neighbor.to_string());
            graph.entry(neighbor.to_string())
                .or_insert_with(Vec::new)
                .push(left.to_string());
        }
    }

    graph
}

fn bfs_path(graph: &Graph, start: &str, end: &str) -> Option<Vec<String>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::default();
    let mut parent: HashMap<String, String> = HashMap::default();

    queue.push_back(start.to_string());
    visited.insert(start.to_string());

    while let Some(node) = queue.pop_front() {
        if node == end {
            // Reconstruct path
            let mut path = vec![node.clone()];
            let mut current = node;
            while let Some(p) = parent.get(&current) {
                path.push(p.clone());
                current = p.clone();
            }
            path.reverse();
            return Some(path);
        }

        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                if visited.insert(neighbor.clone()) {
                    parent.insert(neighbor.clone(), node.clone());
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    None
}

fn count_component_size(graph: &Graph, start: &str) -> usize {
    let mut visited = HashSet::default();
    let mut queue = VecDeque::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(node) = queue.pop_front() {
        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if visited.insert(neighbor.as_str()) {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    visited.len()
}

#[aoc(day25, part1)]
pub fn part1(graph: &Graph) -> usize {
    let mut nodes: Vec<_> = graph.keys().cloned().collect();
    nodes.sort();  // Ensure deterministic node ordering
    let mut edge_counts: HashMap<(String, String), usize> = HashMap::default();

    // Sample shortest paths to count edge usage
    // The 3 edges connecting the two components will have highest betweenness
    let sample_size = nodes.len().min(35);
    for i in 0..sample_size {
        for j in (i + 1)..sample_size {
            if let Some(path) = bfs_path(graph, &nodes[i], &nodes[j]) {
                for window in path.windows(2) {
                    let (a, b) = (&window[0], &window[1]);
                    let edge = if a < b {
                        (a.clone(), b.clone())
                    } else {
                        (b.clone(), a.clone())
                    };
                    *edge_counts.entry(edge).or_insert(0) += 1;
                }
            }
        }
    }

    // Find top 3 edges by usage count
    let mut edges: Vec<_> = edge_counts.iter().collect();
    edges.sort_by_key(|(_, &count)| std::cmp::Reverse(count));

    let to_remove: Vec<_> = edges.iter().take(3).map(|(edge, _)| (*edge).clone()).collect();

    // Create new graph without these 3 edges
    let mut new_graph = graph.clone();
    for (a, b) in &to_remove {
        if let Some(neighbors) = new_graph.get_mut(a) {
            neighbors.retain(|n| n != b);
        }
        if let Some(neighbors) = new_graph.get_mut(b) {
            neighbors.retain(|n| n != a);
        }
    }

    // Count size of one component
    let start = nodes[0].as_str();
    let size1 = count_component_size(&new_graph, start);
    let size2 = nodes.len() - size1;

    size1 * size2
}
