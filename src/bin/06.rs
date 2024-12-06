use std::collections::HashSet;

use advent_of_code::{grid, run_move, DD};

advent_of_code::solution!(6);

fn get_vis(g: &Vec<Vec<char>>, start_pos: (usize, usize)) -> HashSet<(usize, usize)> {
    let r = g.len();
    let c = g[0].len();
    let mut vis_pos: HashSet<(usize, usize)> = HashSet::new();
    let mut cur_pos = start_pos;
    let mut cur_dir = 0;
    loop {
        vis_pos.insert(cur_pos);
        let new_pos = run_move(cur_pos, DD[cur_dir]);
        if new_pos.0 >= r || new_pos.1 >= c {
            return vis_pos;
        }
        if g[new_pos.0][new_pos.1] == '#' {
            cur_dir = (cur_dir+1)%4;
        } else {
            cur_pos = new_pos;
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let g = grid(input);
    let c = g[0].len();
    let start_ind = input.find('^').unwrap();
    let start_pos = (
        start_ind / (c+1),
        start_ind % (c+1)
    );
    Some(get_vis(&g, start_pos).len() as u32)
}

fn loops(g: &Vec<Vec<char>>, start_pos: (usize, usize)) -> bool {
    let r = g.len();
    let c = g[0].len();
    let mut vis_pos: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut cur_pos = start_pos;
    let mut cur_dir = 0;
    loop {
        let state = (cur_pos.0, cur_pos.1, cur_dir);
        if vis_pos.contains(&state) {
            return true;
        }
        vis_pos.insert(state);
        let new_pos = run_move(cur_pos, DD[cur_dir]);
        if new_pos.0 >= r || new_pos.1 >= c {
            return false;
        }
        if g[new_pos.0][new_pos.1] == '#' {
            cur_dir = (cur_dir+1)%4;
        } else {
            cur_pos = new_pos;
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut g = grid(input);
    let c = g[0].len();
    let start_ind = input.find('^').unwrap();
    let start_pos = (
        start_ind / (c+1),
        start_ind % (c+1)
    );
    let mut ans = 0;
    for (i, j) in get_vis(&g, start_pos) {
        g[i][j] = '#';
        ans += if loops(&g, start_pos) {1} else {0};
        g[i][j] = '.';
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
