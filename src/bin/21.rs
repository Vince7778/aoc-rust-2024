use std::{cmp::Ordering, collections::HashMap};

use advent_of_code::{grid_find, grid_old, parse_u};
use itertools::Itertools;

advent_of_code::solution!(21);

trait Pad {
    fn calc_path_cost(&mut self, s: &str) -> usize;
}

struct RobotPad {
    grid: Vec<Vec<char>>,
    linked: Box<dyn Pad>,
    cache: HashMap<String, usize>,
}

impl RobotPad {
    fn new_numpad(l: Box<dyn Pad>) -> Self {
        let g = grid_old("789\n456\n123\n 0A");
        RobotPad {
            grid: g,
            linked: l,
            cache: HashMap::new(),
        }
    }

    fn new_dirpad(l: Box<dyn Pad>) -> Self {
        let g = grid_old(" ^A\n<v>");
        RobotPad {
            grid: g,
            linked: l,
            cache: HashMap::new(),
        }
    }

    fn calc_cost(&mut self, a: char, b: char) -> usize {
        self.get_paths(a, b)
            .into_iter()
            .map(|p| self.linked.calc_path_cost(&p))
            .min()
            .unwrap()
    }

    fn get_paths_dfs(
        &self,
        dr: isize,
        dc: isize,
        goal: (usize, usize),
        s: String,
        res: &mut Vec<String>,
    ) {
        if self.grid[goal.0.wrapping_add_signed(dr)][goal.1.wrapping_add_signed(dc)] == ' ' {
            return;
        }
        if dr == 0 && dc == 0 {
            res.push(s + "A");
        } else {
            match dr.cmp(&0) {
                Ordering::Less => self.get_paths_dfs(dr + 1, dc, goal, s.clone() + "v", res),
                Ordering::Greater => self.get_paths_dfs(dr - 1, dc, goal, s.clone() + "^", res),
                _ => (),
            }
            match dc.cmp(&0) {
                Ordering::Less => self.get_paths_dfs(dr, dc + 1, goal, s.clone() + ">", res),
                Ordering::Greater => self.get_paths_dfs(dr, dc - 1, goal, s.clone() + "<", res),
                _ => (),
            }
        }
    }

    fn get_paths(&self, a: char, b: char) -> Vec<String> {
        let ap = grid_find(&self.grid, a).unwrap();
        let bp = grid_find(&self.grid, b).unwrap();
        let dr = (ap.0 as isize) - (bp.0 as isize);
        let dc = (ap.1 as isize) - (bp.1 as isize);
        let mut res = Vec::new();
        self.get_paths_dfs(dr, dc, bp, "".into(), &mut res);
        res
    }
}

impl Pad for RobotPad {
    fn calc_path_cost(&mut self, s: &str) -> usize {
        if let Some(c) = self.cache.get(s) {
            return *c;
        }
        let mut ans = self.calc_cost('A', s.chars().next().unwrap());
        for (c1, c2) in s.chars().tuple_windows() {
            ans += self.calc_cost(c1, c2);
        }
        self.cache.insert(s.to_string(), ans);
        ans
    }
}

struct PersonPad {}

impl Pad for PersonPad {
    fn calc_path_cost(&mut self, s: &str) -> usize {
        s.len()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let you = PersonPad {};
    let r1 = RobotPad::new_dirpad(Box::new(you));
    let r2 = RobotPad::new_dirpad(Box::new(r1));
    let mut rn = RobotPad::new_numpad(Box::new(r2));
    let res = input
        .lines()
        .map(|l| {
            let cost = rn.calc_path_cost(l);
            let val = parse_u(&l[..3]);
            val * cost
        })
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<usize> {
    let you = PersonPad {};
    let mut r: Box<dyn Pad> = Box::new(you);
    for _ in 0..25 {
        r = Box::new(RobotPad::new_dirpad(r));
    }
    let mut rn = RobotPad::new_numpad(r);
    let res = input
        .lines()
        .map(|l| {
            let cost = rn.calc_path_cost(l);
            let val = parse_u(&l[..3]);
            val * cost
        })
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }
}
