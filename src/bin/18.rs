use itertools::Itertools;
use std::collections::VecDeque;

use advent_of_code::{neighbors, parse_u, repeat_2d};

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<usize> {
    let mut g = repeat_2d(false, 71, 71);
    for l in input.lines().take(1024) {
        let (c, r) = l.split(',').map(parse_u).collect_tuple().unwrap();
        g[r][c] = true;
    }
    let mut q = VecDeque::new();
    let mut vis = repeat_2d(false, g.len(), g[0].len());
    q.push_back((0, 0, 0));
    vis[0][0] = true;
    while let Some((r, c, d)) = q.pop_front() {
        if r == g.len() - 1 && c == g[0].len() - 1 {
            return Some(d);
        }
        for (nr, nc, _) in neighbors(r, c, g.len(), g[0].len()) {
            if !vis[nr][nc] && !g[nr][nc] {
                q.push_back((nr, nc, d + 1));
                vis[nr][nc] = true;
            }
        }
    }
    None
}

const ROW: usize = 71;
const COL: usize = 72;
const SZ: usize = ROW * COL;

#[derive(PartialEq, Eq, Clone, Copy)]
enum FFState {
    Wall,
    TouchedWall,
    Unvisited,
    Visited,
}
use FFState::*;

fn ff(x: usize, m: &mut Vec<FFState>) {
    m[x] = Visited;
    for y in [x + 1, x + COL, x.wrapping_sub(1), x.wrapping_sub(COL)] {
        if y < SZ {
            if m[y] == Wall {
                m[y] = TouchedWall;
            } else if m[y] == Unvisited {
                ff(y, m);
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let mut m = vec![Unvisited; SZ];
    for r in 1..=ROW {
        m[r * COL - 1] = Wall;
    }
    let p = input
        .lines()
        .map(|l| {
            let (c, r) = l.split(',').map(parse_u).collect_tuple().unwrap();
            m[r * COL + c] = Wall;
            r * COL + c
        })
        .collect_vec();
    ff(0, &mut m);
    for x in p.into_iter().rev() {
        if m[x] == TouchedWall {
            ff(x, &mut m);
        } else {
            m[x] = Unvisited;
        }
        if m[SZ - 2] == Visited {
            return Some(format!("{},{}", x % COL, x / COL));
        }
    }
    None
}

#[allow(clippy::upper_case_acronyms)]
struct DSU {
    p: Vec<usize>,
}

impl DSU {
    pub fn new(s: usize) -> Self {
        DSU {
            p: (0..s).collect(),
        }
    }

    pub fn find(&mut self, mut i: usize) -> usize {
        while i != self.p[i] {
            self.p[i] = self.p[self.p[i]];
            i = self.p[i];
        }
        i
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let a = self.find(a);
        let b = self.find(b);
        if a != b {
            self.p[a] = b;
        }
    }

    pub fn same_set(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

pub fn part_two_dsu(input: &str) -> Option<String> {
    let mut g = vec![false; SZ];
    for r in 1..=ROW {
        g[r * COL - 1] = true;
    }
    let p: Vec<usize> = input
        .lines()
        .map(|l| {
            let (c, r) = l.split(',').map(parse_u).collect_tuple().unwrap();
            g[r * COL + c] = true;
            r * COL + c
        })
        .collect_vec();
    let mut dsu = DSU::new(SZ);
    for i in 0..SZ {
        if g[i] {
            continue;
        }
        if i + 1 < SZ && !g[i + 1] {
            dsu.union(i, i + 1);
        }
        if i + COL < SZ && !g[i + COL] {
            dsu.union(i, i + COL);
        }
    }
    for x in p.into_iter().rev() {
        g[x] = false;
        for y in [x + 1, x + COL, x.wrapping_sub(1), x.wrapping_sub(COL)] {
            if y < SZ && !g[y] {
                dsu.union(x, y);
            }
        }
        if dsu.same_set(0, SZ - 2) {
            return Some(format!("{},{}", x % COL, x / COL));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    // examples are again dissimilar from input
}
