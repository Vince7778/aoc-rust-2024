use itertools::Itertools;

use advent_of_code::{grid_dist, parse_u, repeat_2d};

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<usize> {
    let mut g = repeat_2d(false, 71, 71);
    for l in input.lines().take(1024) {
        let (c, r) = l.split(',').map(parse_u).collect_tuple().unwrap();
        g[r][c] = true;
    }
    let dist = grid_dist(&g, (0, 0), true);
    Some(dist[70][70])
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

const COMMA: u8 = 44;
const ZERO: u8 = 48;

fn cv(arr: &[u8], i: usize) -> usize {
    (arr[i] - ZERO) as usize
}
fn stupid_input(l: &str) -> usize {
    let arr = l.as_bytes();
    match arr.len() {
        3 => cv(arr, 0) + cv(arr, 2) * COL,
        4 => {
            if arr[1] == COMMA {
                cv(arr, 0) + (cv(arr, 2) * 10 + cv(arr, 3)) * COL
            } else {
                cv(arr, 0) * 10 + cv(arr, 1) + cv(arr, 3) * COL
            }
        }
        5 => cv(arr, 0) * 10 + cv(arr, 1) + (cv(arr, 3) * 10 + cv(arr, 4)) * COL,
        _ => unreachable!(),
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
            let x = stupid_input(l);
            m[x] = Wall;
            x
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
