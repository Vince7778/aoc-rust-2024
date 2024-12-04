use advent_of_code::{grid, mat_rotate};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut g = grid(input);
    let mut ans = 0;
    for _ in 0..4 {
        for r in 0..g.len() {
            for c in 0..g[r].len() - 3 {
                if g[r][c..c + 4] == ['X', 'M', 'A', 'S'] {
                    ans += 1;
                }
                if r < g.len() - 3
                    && g[r][c] == 'X'
                    && g[r + 1][c + 1] == 'M'
                    && g[r + 2][c + 2] == 'A'
                    && g[r + 3][c + 3] == 'S'
                {
                    ans += 1;
                }
            }
        }
        g = mat_rotate(g);
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut g = grid(input);
    let mut ans = 0;
    for _ in 0..4 {
        for r in 0..g.len() - 2 {
            for c in 0..g[r].len() - 2 {
                if g[r][c] == 'M'
                    && g[r + 1][c + 1] == 'A'
                    && g[r + 2][c + 2] == 'S'
                    && g[r + 2][c] == 'M'
                    && g[r][c + 2] == 'S'
                {
                    ans += 1;
                }
            }
        }
        g = mat_rotate(g);
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
