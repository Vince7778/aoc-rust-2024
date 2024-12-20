use std::collections::HashMap;

use advent_of_code::{grid, neighbors, repeat_2d, run_move};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<usize> {
    let g = grid(input);
    let r = g.len();
    let c = g[0].len();
    let mut s = (0, 0);
    let mut e = (0, 0);
    for i in 0..r {
        for j in 0..c {
            match g[i][j] {
                'S' => { s = (i, j); }
                'E' => { e = (i, j); }
                _ => {}
            }
        }
    }
    let mut p = s;
    let mut pp = p;
    let mut d = 0;
    let mut dist = repeat_2d(0, r, c);
    while p != e {
        for (nr, nc, _) in neighbors(p.0, p.1, r, c) {
            if g[nr][nc] == '#' || (nr, nc) == pp {
                continue;
            }
            pp = p;
            p = (nr, nc);
            d += 1;
            dist[nr][nc] = d;
            break;
        }
    }
    let ddd = [(0, 2), (1, 1), (2, 0), (1, -1), (0, -2), (-1, -1), (-2, 0), (-1, 1)];
    let mut saved: HashMap<usize, usize> = HashMap::new();
    let mut ans = 0;
    for i in 0..r {
        for j in 0..c {
            if g[i][j] == '#' {
                continue;
            }
            for cd in ddd {
                let (nr, nc) = run_move((i, j), cd);
                if nr >= r || nc >= c || g[nr][nc] == '#' || dist[nr][nc] < dist[i][j] {
                    continue;
                }
                let v = dist[nr][nc] - dist[i][j] - 2;
                if v >= 100 {
                    ans += 1;
                }
            }
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let g = grid(input);
    let r = g.len();
    let c = g[0].len();
    let mut s = (0, 0);
    let mut e = (0, 0);
    for i in 0..r {
        for j in 0..c {
            match g[i][j] {
                'S' => { s = (i, j); }
                'E' => { e = (i, j); }
                _ => {}
            }
        }
    }
    let mut p = s;
    let mut pp = p;
    let mut d = 0;
    let mut dist = repeat_2d(0, r, c);
    while p != e {
        for (nr, nc, _) in neighbors(p.0, p.1, r, c) {
            if g[nr][nc] == '#' || (nr, nc) == pp {
                continue;
            }
            pp = p;
            p = (nr, nc);
            d += 1;
            dist[nr][nc] = d;
            break;
        }
    }
    let ddd = [(0, 2), (1, 1), (2, 0), (1, -1), (0, -2), (-1, -1), (-2, 0), (-1, 1)];
    let mut saved: HashMap<usize, usize> = HashMap::new();
    let mut ans = 0;
    for i in 0..r {
        for j in 0..c {
            if g[i][j] == '#' {
                continue;
            }
            for i1 in 0..r {
                for j1 in 0..c {
                    if g[i1][j1] == '#' || i.abs_diff(i1) + j.abs_diff(j1) > 20 || dist[i1][j1] <= dist[i][j] {
                        continue;
                    }
                    let v = dist[i1][j1] - dist[i][j] - (i.abs_diff(i1) + j.abs_diff(j1));
                    if v >= 100 {
                        ans += 1;
                    }
                }
            }
        }
    }
    Some(ans)
}

#[cfg(test)]
mod tests {}
