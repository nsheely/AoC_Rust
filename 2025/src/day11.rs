// Day 11: Reactor
//
// Part 1: Count all paths from "you" to "out" in DAG
//   Memoized DFS
//
// Part 2: Count paths from "svr" to "out" visiting both "dac" AND "fft"
//   Forward DP on reachable subgraph with 2-bit state mask
//   Postorder DFS to collect nodes, then topological DP (no recursion overhead)

use rustc_hash::FxHashMap;

const UNK: usize = usize::MAX;

type Mask = u8;
const DAC: Mask = 1;
const FFT: Mask = 2;
const BOTH: Mask = DAC | FFT;
const MASKS: usize = 4;

#[derive(Clone, Copy, PartialEq, Debug)]
enum State {
    Unseen,
    Visiting,
    Done,
}

pub struct GraphData {
    adj: Vec<Vec<usize>>,
    tag: Vec<Mask>,
    you: usize,
    svr: usize,
    out: usize,
}

fn intern<'a>(
    ids: &mut FxHashMap<&'a str, usize>,
    adj: &mut Vec<Vec<usize>>,
    name: &'a str,
) -> usize {
    if let Some(&id) = ids.get(name) {
        return id;
    }
    let id = adj.len();
    ids.insert(name, id);
    adj.push(Vec::new());
    id
}

#[aoc_generator(day11)]
pub fn parse(input: &str) -> GraphData {
    let mut ids = FxHashMap::default();
    let mut adj = Vec::new();

    for line in input.lines() {
        let Some((device, rest)) = line.split_once(':') else {
            continue;
        };

        let u = intern(&mut ids, &mut adj, device);
        for name in rest.split_whitespace() {
            let v = intern(&mut ids, &mut adj, name);
            adj[u].push(v);
        }
    }

    let &you = ids.get("you").expect("'you' not found");
    let &svr = ids.get("svr").expect("'svr' not found");
    let &out = ids.get("out").expect("'out' not found");
    let &dac = ids.get("dac").expect("'dac' not found");
    let &fft = ids.get("fft").expect("'fft' not found");

    let mut tag = vec![0; adj.len()];
    tag[dac] = DAC;
    tag[fft] = FFT;

    GraphData {
        adj,
        tag,
        you,
        svr,
        out,
    }
}

#[inline]
fn paths_memo(adj: &[Vec<usize>], u: usize, out: usize, memo: &mut [usize]) -> usize {
    if u == out {
        return 1;
    }
    if memo[u] != UNK {
        return memo[u];
    }

    let total = adj[u].iter().map(|&v| paths_memo(adj, v, out, memo)).sum();
    memo[u] = total;
    total
}

fn topo_from(adj: &[Vec<usize>], start: usize) -> Vec<usize> {
    fn dfs(adj: &[Vec<usize>], u: usize, state: &mut [State], order: &mut Vec<usize>) {
        match state[u] {
            State::Done => return,
            State::Visiting => panic!("cycle detected"),
            State::Unseen => {}
        }
        state[u] = State::Visiting;
        for &v in &adj[u] {
            dfs(adj, v, state, order);
        }
        state[u] = State::Done;
        order.push(u);
    }

    let mut state = vec![State::Unseen; adj.len()];
    let mut post = Vec::with_capacity(adj.len());
    dfs(adj, start, &mut state, &mut post);
    post.reverse();
    post
}

fn paths_with_requirements(adj: &[Vec<usize>], tag: &[Mask], start: usize, out: usize) -> usize {
    let order = topo_from(adj, start);

    let mut dp = vec![[0usize; MASKS]; adj.len()];
    dp[start][tag[start] as usize] = 1;

    for &u in &order {
        for mask in 0..MASKS {
            let count = dp[u][mask];
            if count == 0 {
                continue;
            }
            for &v in &adj[u] {
                dp[v][mask | tag[v] as usize] += count;
            }
        }
    }

    dp[out][BOTH as usize]
}

#[aoc(day11, part1)]
pub fn part1(g: &GraphData) -> usize {
    let mut memo = vec![UNK; g.adj.len()];
    paths_memo(&g.adj, g.you, g.out, &mut memo)
}

#[aoc(day11, part2)]
pub fn part2(g: &GraphData) -> usize {
    paths_with_requirements(&g.adj, &g.tag, g.svr, g.out)
}
