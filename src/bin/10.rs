use std::collections::{HashMap, HashSet};

use advent_of_code::{grid, neighbors, parse_char};

advent_of_code::solution!(10);

fn dfs(g: &Vec<Vec<char>>, r: usize, c: usize, vis: &mut HashSet<(usize, usize)>) -> usize {
    if vis.contains(&(r, c)) {
        return 0;
    }
    vis.insert((r, c));
    if g[r][c] == '9' {
        return 1;
    }
    let mut out = 0;
    for (nr, nc, _) in neighbors(r, c, g.len(), g[0].len()) {
        if parse_char(g[nr][nc]) == parse_char(g[r][c]) + 1 {
            out += dfs(g, nr, nc, vis);
        }
    }
    out
}

pub fn part_one(input: &str) -> Option<u32> {
    let g = grid(input);
    let mut ans = 0;
    for sr in 0..g.len() {
        for sc in 0..g[0].len() {
            if g[sr][sc] == '0' {
                let mut vis = HashSet::new();
                ans += dfs(&g, sr, sc, &mut vis);
            }
        }
    }
    Some(ans as u32)
}

fn dfs2(g: &Vec<Vec<char>>, r: usize, c: usize, dp: &mut HashMap<(usize, usize), usize>) -> usize {
    if dp.contains_key(&(r, c)) {
        return *dp.get(&(r, c)).unwrap();
    }
    if g[r][c] == '9' {
        return 1;
    }
    let mut out = 0;
    for (nr, nc, _) in neighbors(r, c, g.len(), g[0].len()) {
        if parse_char(g[nr][nc]) == parse_char(g[r][c]) + 1 {
            out += dfs2(g, nr, nc, dp);
        }
    }
    dp.insert((r, c), out);
    out
}

pub fn part_two(input: &str) -> Option<u32> {
    let g = grid(input);
    let mut ans = 0;
    for sr in 0..g.len() {
        for sc in 0..g[0].len() {
            if g[sr][sc] == '0' {
                let mut vis = HashMap::new();
                ans += dfs2(&g, sr, sc, &mut vis);
            }
        }
    }
    Some(ans as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
