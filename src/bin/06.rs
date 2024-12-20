use std::collections::HashSet;

use advent_of_code::{grid_old, run_move, DD};

advent_of_code::solution!(6);

fn simulate(g: &[Vec<char>], start_pos: (usize, usize)) -> (HashSet<(usize, usize)>, bool) {
    let r = g.len();
    let c = g[0].len();
    let mut vis_pos: HashSet<(usize, usize)> = HashSet::new();
    let mut state_pos: HashSet<((usize, usize), usize)> = HashSet::new();
    let mut cur_pos = start_pos;
    let mut cur_dir = 0;
    loop {
        vis_pos.insert(cur_pos);
        if state_pos.contains(&(cur_pos, cur_dir)) {
            return (vis_pos, true);
        }
        state_pos.insert((cur_pos, cur_dir));
        let new_pos = run_move(cur_pos, DD[cur_dir]);
        if new_pos.0 >= r || new_pos.1 >= c {
            return (vis_pos, false);
        }
        if g[new_pos.0][new_pos.1] == '#' {
            cur_dir = (cur_dir + 1) % 4;
        } else {
            cur_pos = new_pos;
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let g = grid_old(input);
    let c = g[0].len();
    let start_ind = input.find('^').unwrap();
    let start_pos = (start_ind / (c + 1), start_ind % (c + 1));
    let (vis, _) = simulate(&g, start_pos);
    Some(vis.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut g = grid_old(input);
    let c = g[0].len();
    let start_ind = input.find('^').unwrap();
    let start_pos = (start_ind / (c + 1), start_ind % (c + 1));
    let mut ans = 0;
    let (vis, _) = simulate(&g, start_pos);
    for (i, j) in vis {
        g[i][j] = '#';
        ans += if simulate(&g, start_pos).1 { 1 } else { 0 };
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
