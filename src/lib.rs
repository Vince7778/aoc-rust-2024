use std::{collections::VecDeque, str::FromStr};

use itertools::repeat_n;

pub mod template;

fn parse_map<T: FromStr>(input: &str) -> Vec<T> {
    input
        .split_whitespace()
        .filter_map(|l| Result::ok(l.parse::<T>()))
        .collect()
}
pub fn naive_ints(input: &str) -> Vec<isize> {
    parse_map(input)
}
pub fn ints(input: &str, negatives: bool) -> Vec<isize> {
    input
        .split(|c: char| !(c.is_ascii_digit() || negatives && c == '-'))
        .filter_map(|s| Result::ok(s.parse::<isize>()))
        .collect()
}
pub fn uints(input: &str) -> Vec<usize> {
    input
        .split(|c: char| !c.is_ascii_digit())
        .filter_map(|s| Result::ok(s.parse::<usize>()))
        .collect()
}
pub fn floats(input: &str) -> Vec<f64> {
    parse_map(input)
}
pub fn vec_lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn parse(s: &str) -> isize {
    s.parse().unwrap()
}
pub fn parse_u(s: &str) -> usize {
    s.parse().unwrap()
}
pub fn parse_char(c: char) -> isize {
    char::to_digit(c, 10).unwrap() as isize
}

pub fn grid_old(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}
pub fn grid(input: &str) -> (Vec<Vec<char>>, usize, usize) {
    let g: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let r = g.len();
    let c = g[0].len();
    (g, r, c)
}
pub fn int_grid(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect()
}

pub fn grid_find<T: Eq>(g: &[Vec<T>], v: T) -> Option<(usize, usize)> {
    g.iter()
        .enumerate()
        .find_map(|(i, r)| r.iter().position(|x| *x == v).map(|j| (i, j)))
}

pub fn grid_dist<T: Eq>(g: &[Vec<T>], s: (usize, usize), wall: T) -> Vec<Vec<usize>> {
    let r = g.len();
    let c = g[0].len();
    let mut res = repeat_2d(usize::MAX, r, c);
    res[s.0][s.1] = 0;
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back(s);
    while let Some(p) = q.pop_front() {
        for (nr, nc, _) in neighbors(p.0, p.1, r, c) {
            if g[nr][nc] != wall && res[nr][nc] == usize::MAX {
                res[nr][nc] = res[p.0][p.1] + 1;
                q.push_back((nr, nc));
            }
        }
    }
    res
}

pub fn repeat_2d<T: Clone>(val: T, r: usize, c: usize) -> Vec<Vec<T>> {
    repeat_n(repeat_n(val, c).collect(), r).collect()
}
pub fn mat_transpose<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..matrix[0].len())
        .map(|i| matrix.iter().map(|row| row[i].clone()).collect())
        .collect()
}
// rotate clockwise
pub fn mat_rotate<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rc = matrix.len();
    let cc = matrix[0].len();
    (0..cc)
        .map(|i| (0..rc).rev().map(|j| matrix[j][i].clone()).collect())
        .collect()
}

pub fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
pub fn lcm(a: isize, b: isize) -> isize {
    a / gcd(a, b) * b
}
pub fn count_digits(mut x: usize) -> u32 {
    let mut ans = 0;
    while x > 0 {
        ans += 1;
        x /= 10;
    }
    ans
}

pub const DD: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn run_move(p: (usize, usize), d: (isize, isize)) -> (usize, usize) {
    (p.0.wrapping_add_signed(d.0), p.1.wrapping_add_signed(d.1))
}
pub fn neighbors(cr: usize, cc: usize, r: usize, c: usize) -> Vec<(usize, usize, char)> {
    let dirs = ['N', 'E', 'S', 'W'];
    let mut res = vec![];
    for i in 0..DD.len() {
        let (nr, nc) = run_move((cr, cc), DD[i]);
        if nr < r && nc < c {
            res.push((nr, nc, dirs[i]));
        }
    }
    res
}
pub fn neighbors8(cr: isize, cc: isize, r: usize, c: usize) -> Vec<(isize, isize, &'static str)> {
    let dr = [-1, -1, 0, 1, 1, 1, 0, -1];
    let dc = [0, 1, 1, 1, 0, -1, -1, -1];
    let dirs = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    let mut res = vec![];
    for i in 0..dirs.len() {
        if cr + dr[i] >= 0
            && cr + dr[i] < (r as isize)
            && cc + dc[i] >= 0
            && cc + dc[i] < (c as isize)
        {
            res.push((cr + dr[i], cc + dc[i], dirs[i]));
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::{grid_dist, grid_find, grid_old, ints, uints};

    #[test]
    fn test_parse_uints() {
        let s = "1asdf141 -42 something!?\"4343 -34  04 00 0";
        assert_eq!(uints(s), vec![1, 141, 42, 4343, 34, 4, 0, 0]);
    }

    #[test]
    fn test_parse_ints() {
        let s = "1asdf141 -42 something!?\"4343 -34  04 00 0";
        assert_eq!(ints(s, true), vec![1, 141, -42, 4343, -34, 4, 0, 0]);
    }

    #[test]
    fn test_parse_ints_no_negative() {
        let s = "1asdf141 -42 something!?\"4343 -34  04 00 0";
        assert_eq!(ints(s, false), vec![1, 141, 42, 4343, 34, 4, 0, 0]);
    }

    #[test]
    fn test_grid_find() {
        let g = vec![vec![0, 2, 3], vec![4, -1, 7]];
        for i in 0..g.len() {
            for j in 0..g[0].len() {
                assert_eq!(grid_find(&g, g[i][j]), Some((i, j)));
            }
        }
        assert_eq!(grid_find(&g, 1), None)
    }

    #[test]
    fn test_grid_dist() {
        /*
        12345
        0#4#6
        #65##
        876#-
        */
        let g = grid_old(".....\n.#.#.\n#..##\n...#.");
        let gf = grid_dist(&g, (1, 0), '#');
        let m = usize::MAX;
        let expected = vec![
            vec![1, 2, 3, 4, 5],
            vec![0, m, 4, m, 6],
            vec![m, 6, 5, m, m],
            vec![8, 7, 6, m, m],
        ];
        assert_eq!(gf, expected);
    }
}
