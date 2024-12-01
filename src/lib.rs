use std::str::FromStr;

use itertools::repeat_n;

pub mod template;

fn parse_map<T: FromStr>(input: &str) -> Vec<T> {
    input
        .split_whitespace()
        .filter_map(|l| Result::ok(l.parse::<T>()))
        .collect()
}
pub fn ints(input: &str) -> Vec<i64> {
    parse_map(input)
}
pub fn floats(input: &str) -> Vec<f64> {
    parse_map(input)
}
pub fn vec_lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn pint(c: char) -> i64 {
    char::to_digit(c, 10).unwrap() as i64
}

pub fn grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}
pub fn int_grid(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect()
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

pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
pub fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

pub fn neighbors(cr: i64, cc: i64, r: usize, c: usize) -> Vec<(i64, i64, char)> {
    let dr = [-1, 0, 1, 0];
    let dc = [0, 1, 0, -1];
    let dirs = ['N', 'E', 'S', 'W'];
    let mut res = vec![];
    for i in 0..dirs.len() {
        if cr + dr[i] >= 0 && cr + dr[i] < (r as i64) && cc + dc[i] >= 0 && cc + dc[i] < (c as i64)
        {
            res.push((cr + dr[i], cc + dc[i], dirs[i]));
        }
    }
    res
}
pub fn neighbors8(cr: i64, cc: i64, r: usize, c: usize) -> Vec<(i64, i64, &'static str)> {
    let dr = [-1, -1, 0, 1, 1, 1, 0, -1];
    let dc = [0, 1, 1, 1, 0, -1, -1, -1];
    let dirs = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    let mut res = vec![];
    for i in 0..dirs.len() {
        if cr + dr[i] >= 0 && cr + dr[i] < (r as i64) && cc + dc[i] >= 0 && cc + dc[i] < (c as i64)
        {
            res.push((cr + dr[i], cc + dc[i], dirs[i]));
        }
    }
    res
}
