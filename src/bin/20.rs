use advent_of_code::{grid, grid_dist, grid_find};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<usize> {
    let (g, r, c) = grid(input);
    let s = grid_find(&g, 'S').unwrap();
    let dist = grid_dist(&g, s, '#');
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
                    let saved = dist[i1][j1].saturating_sub(dist[i][j]).saturating_sub(md);
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
    let dist = grid_dist(&g, s, '#');
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
                    let saved = dist[i1][j1].saturating_sub(dist[i][j]).saturating_sub(md);
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
