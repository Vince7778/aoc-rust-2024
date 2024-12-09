use std::collections::HashSet;

use advent_of_code::{grid, run_move};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let g = grid(input);
    let mut s: HashSet<(usize, usize)> = HashSet::new();
    for r1 in 0..g.len() {
        for c1 in 0..g[0].len() {
            for r2 in 0..g.len() {
                for c2 in 0..g[0].len() {
                    if (r1 == r2 && c1 == c2) || g[r1][c1] == '.' || g[r1][c1] != g[r2][c2] {
                        continue;
                    }
                    let (rm, cm) = run_move(
                        (r2, c2),
                        ((r2 as isize) - (r1 as isize), (c2 as isize) - (c1 as isize)),
                    );
                    if rm < g.len() && cm < g[0].len() {
                        s.insert((rm, cm));
                    }
                }
            }
        }
    }
    Some(s.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let g = grid(input);
    let mut s: HashSet<(usize, usize)> = HashSet::new();
    for r1 in 0..g.len() {
        for c1 in 0..g[0].len() {
            for r2 in 0..g.len() {
                for c2 in 0..g[0].len() {
                    if (r1 == r2 && c1 == c2) || g[r1][c1] == '.' || g[r1][c1] != g[r2][c2] {
                        continue;
                    }
                    let dr = (r2 as isize) - (r1 as isize);
                    let dc = (c2 as isize) - (c1 as isize);
                    let mut i = -1;
                    loop {
                        let (rm, cm) = run_move((r1, c1), (i * dr, i * dc));
                        if rm >= g.len() || cm >= g[0].len() {
                            break;
                        }
                        s.insert((rm, cm));
                        i -= 1;
                    }
                    let mut i = 0;
                    loop {
                        let (rm, cm) = run_move((r1, c1), (i * dr, i * dc));
                        if rm >= g.len() || cm >= g[0].len() {
                            break;
                        }
                        s.insert((rm, cm));
                        i += 1;
                    }
                }
            }
        }
    }
    Some(s.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
