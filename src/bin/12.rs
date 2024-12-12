use advent_of_code::{grid, neighbors, repeat_2d};

advent_of_code::solution!(12);

fn dfs(r: usize, c: usize, peri: &Vec<Vec<usize>>, g: &Vec<Vec<char>>, vis: &mut Vec<Vec<bool>>) -> (usize, usize) {
    if vis[r][c] {
        return (0, 0);
    }
    vis[r][c] = true;
    let mut res = (1, peri[r][c]);
    for (nr, nc, _) in neighbors(r, c, g.len(), g[0].len()) {
        if g[nr][nc] != g[r][c] {
            continue;
        } else {
            let r = dfs(nr, nc, peri, g, vis);
            res.0 += r.0;
            res.1 += r.1;
        }
    }
    res
}

pub fn part_one(input: &str) -> Option<usize> {
    let g = grid(input);
    let mut ans = 0;

    let mut peri = repeat_2d(0_usize, g.len(), g[0].len());
    for r in 0..g.len() {
        for c in 0..g[0].len() {
            if r == 0 || g[r][c] != g[r-1][c] {
                peri[r][c] += 1;
            }
            if c == 0 || g[r][c] != g[r][c-1] {
                peri[r][c] += 1;
            }
            if r == g.len()-1 || g[r][c] != g[r+1][c] {
                peri[r][c] += 1;
            }
            if c == g[0].len()-1 || g[r][c] != g[r][c+1] {
                peri[r][c] += 1;
            }
        }
    }

    let mut vis = repeat_2d(false, g.len(), g[0].len());
    for r in 0..g.len() {
        for c in 0..g[0].len() {
            let (a, p) = dfs(r, c, &peri, &g, &mut vis);
            ans += a*p;
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let g = grid(input);

    let mut peri = repeat_2d(0_usize, g.len(), g[0].len());
    for r in 0..g.len() {
        for c in 0..g[0].len() {
            if (r == 0 || g[r][c] != g[r-1][c]) && (c == 0 || g[r][c] != g[r][c-1] || (r != 0 && g[r][c-1] == g[r-1][c-1])) {
                peri[r][c] += 1;
            }
            if (c == 0 || g[r][c] != g[r][c-1]) && (r == 0 || g[r][c] != g[r-1][c] || (c != 0 && g[r-1][c] == g[r-1][c-1])) {
                peri[r][c] += 1;
            }
            if (r == g.len()-1 || g[r][c] != g[r+1][c]) && (c == 0 || g[r][c] != g[r][c-1] || (r != g.len()-1 && g[r][c-1] == g[r+1][c-1])) {
                peri[r][c] += 1;
            }
            if (c == g[0].len()-1 || g[r][c] != g[r][c+1]) && (r == 0 || g[r][c] != g[r-1][c] || (c != g[0].len()-1 && g[r-1][c] == g[r-1][c+1])) {
                peri[r][c] += 1;
            }
        }
    }

    let mut ans = 0;
    let mut vis = repeat_2d(false, g.len(), g[0].len());
    for r in 0..g.len() {
        for c in 0..g[0].len() {
            let (a, p) = dfs(r, c, &peri, &g, &mut vis);
            ans += a*p;
        }
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(772));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(436));
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(1206));
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 4));
        assert_eq!(result, Some(236));
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 5));
        assert_eq!(result, Some(368));
    }
}
