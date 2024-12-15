use std::collections::HashSet;

use advent_of_code::{grid, repeat_2d, run_move};
use itertools::Itertools;

advent_of_code::solution!(15);

fn shift(g: &mut Vec<Vec<char>>, p: (usize, usize), d: (isize, isize)) -> bool {
    let np = run_move(p, d);
    match g[np.0][np.1] {
        '#' => false,
        '.' => {
            g[np.0][np.1] = g[p.0][p.1];
            true
        }
        'O' => {
            let r = shift(g, np, d);
            if r {
                g[np.0][np.1] = g[p.0][p.1];
            }
            r
        }
        x => panic!("unexpected char in shift {}", x),
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (g, moves) = input.split("\n\n").collect_tuple().unwrap();
    let mut g = grid(g);
    let mut rb = None;
    for r in 0..g.len() {
        for c in 0..g[0].len() {
            if g[r][c] == '@' {
                rb = Some((r, c));
                break;
            }
        }
        if rb.is_some() {
            break;
        }
    }
    let mut rb = rb.unwrap();
    for m in moves.chars() {
        let d = match m {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => {
                continue;
            }
        };
        if shift(&mut g, rb, d) {
            g[rb.0][rb.1] = '.';
            rb = run_move(rb, d);
        }
    }
    let mut ans = 0;
    for r in 0..g.len() {
        for c in 0..g[0].len() {
            if g[r][c] == 'O' {
                ans += r * 100 + c;
            }
        }
    }
    Some(ans)
}

fn shift2v(g: &mut Vec<Vec<char>>, r: usize, cs: Vec<usize>, up: bool) -> bool {
    let mut ncs = HashSet::new();
    let nr = if up { r - 1 } else { r + 1 };
    for c in cs.clone() {
        match g[nr][c] {
            '#' => {
                return false;
            }
            '[' => {
                ncs.insert(c);
                ncs.insert(c + 1);
            }
            ']' => {
                ncs.insert(c);
                ncs.insert(c - 1);
            }
            '.' => {}
            x => panic!("unexpected char in shift {}", x),
        }
    }
    if !ncs.is_empty() {
        let v = ncs.into_iter().collect_vec();
        if !shift2v(g, nr, v, up) {
            return false;
        }
    }
    for c in cs {
        g[nr][c] = g[r][c];
        g[r][c] = '.';
    }
    true
}

fn shift2h(g: &mut Vec<Vec<char>>, r: usize, c: usize, left: bool) -> bool {
    let nc = if left { c - 1 } else { c + 1 };
    match g[r][nc] {
        '#' => {
            return false;
        }
        '[' | ']' => {
            if !shift2h(g, r, nc, left) {
                return false;
            }
        }
        '.' => {}
        x => panic!("unexpected char in shift {}", x),
    }
    g[r][nc] = g[r][c];
    g[r][c] = '.';
    true
}

pub fn part_two(input: &str) -> Option<usize> {
    let (g, moves) = input.split("\n\n").collect_tuple().unwrap();
    let g = grid(g);
    let mut ng = repeat_2d('.', g.len(), g[0].len() * 2);
    for r in 0..g.len() {
        for c in 0..g[0].len() {
            (ng[r][2 * c], ng[r][2 * c + 1]) = match g[r][c] {
                '#' => ('#', '#'),
                '.' => ('.', '.'),
                '@' => ('@', '.'),
                'O' => ('[', ']'),
                x => panic!("unexpected char in parse {}", x),
            };
        }
    }
    let mut g = ng;
    let mut rb = None;
    for r in 0..g.len() {
        for c in 0..g[0].len() {
            if g[r][c] == '@' {
                rb = Some((r, c));
                break;
            }
        }
        if rb.is_some() {
            break;
        }
    }
    let mut rb = rb.unwrap();
    for m in moves.chars() {
        let (res, d) = match m {
            '<' => (shift2h(&mut g, rb.0, rb.1, true), (0, -1)),
            '>' => (shift2h(&mut g, rb.0, rb.1, false), (0, 1)),
            '^' => (shift2v(&mut g, rb.0, vec![rb.1], true), (-1, 0)),
            'v' => (shift2v(&mut g, rb.0, vec![rb.1], false), (1, 0)),
            '\n' => {
                continue;
            }
            _ => panic!(),
        };
        if res {
            g[rb.0][rb.1] = '.';
            rb = run_move(rb, d);
        }
    }
    let mut ans = 0;
    for r in 0..g.len() {
        for c in 0..g[0].len() {
            if g[r][c] == '[' {
                ans += r * 100 + c;
            }
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
        assert_eq!(result, Some(10092));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
