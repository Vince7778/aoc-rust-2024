use std::collections::VecDeque;

use advent_of_code::{neighbors, repeat_2d, uints};

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<usize> {
    let mut g = repeat_2d(false, 71, 71);
    for l in input.lines().take(1024) {
        let [c, r] = uints(l)[..] else {panic!()};
        g[r][c] = true;
    }
    let mut q = VecDeque::new();
    let mut vis = repeat_2d(false, g.len(), g[0].len());
    q.push_back((0, 0, 0));
    vis[0][0] = true;
    while let Some((r, c, d)) = q.pop_front() {
        if r == g.len()-1 && c == g[0].len()-1 {
            return Some(d);
        }
        for (nr, nc, _) in neighbors(r, c, g.len(), g[0].len()) {
            if !vis[nr][nc] && !g[nr][nc] {
                q.push_back((nr, nc, d+1));
                vis[nr][nc] = true;
            }
        }
    }
    None
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum FFState {
    Unvisited,
    Start,
    End
}
use FFState::*;

fn ff(r: usize, c: usize, s: FFState, m: &mut Vec<Vec<FFState>>, g: &[Vec<bool>]) {
    m[r][c] = s;
    for (nr, nc, _) in neighbors(r, c, g.len(), g[0].len()) {
        if !g[nr][nc] && m[nr][nc] == Unvisited {
            ff(nr, nc, s, m, g)
        }
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let mut g = repeat_2d(false, 71, 71);
    for l in input.lines() {
        let [c, r] = uints(l)[..] else {panic!()};
        g[r][c] = true;
    }
    let mut m = repeat_2d(Unvisited, g.len(), g[0].len());
    ff(0, 0, Start, &mut m, &g);
    ff(g.len()-1, g[0].len()-1, End, &mut m, &g);
    for l in input.lines().rev() {
        let [c, r] = uints(l)[..] else {panic!()};
        g[r][c] = false;
        let mut s = Unvisited;
        for (nr, nc, _) in neighbors(r, c, g.len(), g[0].len()) {
            s = match (s, m[nr][nc]) {
                (Unvisited, x) | (x, Unvisited) => x,
                (x, y) if x == y => x,
                _ => { return Some(format!("{},{}", c, r)); }
            }
        }
        if s != Unvisited {
            ff(r, c, s, &mut m, &g);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    // examples are again dissimilar from input
}
