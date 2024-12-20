use advent_of_code::{grid, grid_find, neighbors, repeat_2d};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<usize> {
    let (g, r, c) = grid(input);
    let s = grid_find(&g, 'S').unwrap();
    let e = grid_find(&g, 'E').unwrap();
    let mut cur = s;
    let mut prev = cur;
    let mut dist = repeat_2d(0, r, c);
    while cur != e {
        for (nr, nc, _) in neighbors(cur.0, cur.1, r, c) {
            if g[nr][nc] == '#' || (nr, nc) == prev {
                continue;
            }
            dist[nr][nc] = dist[cur.0][cur.1] + 1;
            prev = cur;
            cur = (nr, nc);
            break;
        }
    }
    let mut ans = 0;
    for i in 0..r {
        for j in 0..c {
            if g[i][j] == '#' {
                continue;
            }
            for i1 in 0..r {
                for j1 in 0..c {
                    if g[i1][j1] == '#' {
                        continue;
                    }
                    let md = i.abs_diff(i1) + j.abs_diff(j1);
                    let saved = dist[i1][j1] - dist[i][j] - (md as i32);
                    if md == 2 && saved >= 100 {
                        ans += 1;
                    }
                }
            }
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (g, r, c) = grid(input);
    let s = grid_find(&g, 'S').unwrap();
    let e = grid_find(&g, 'E').unwrap();
    let mut cur = s;
    let mut prev = cur;
    let mut dist = repeat_2d(0, r, c);
    while cur != e {
        for (nr, nc, _) in neighbors(cur.0, cur.1, r, c) {
            if g[nr][nc] == '#' || (nr, nc) == prev {
                continue;
            }
            dist[nr][nc] = dist[cur.0][cur.1] + 1;
            prev = cur;
            cur = (nr, nc);
            break;
        }
    }
    let mut ans = 0;
    for i in 0..r {
        for j in 0..c {
            if g[i][j] == '#' {
                continue;
            }
            for i1 in 0..r {
                for j1 in 0..c {
                    if g[i1][j1] == '#' {
                        continue;
                    }
                    let md = i.abs_diff(i1) + j.abs_diff(j1);
                    let saved = dist[i1][j1] - dist[i][j] - (md as i32);
                    if md <= 20 && saved >= 100 {
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
