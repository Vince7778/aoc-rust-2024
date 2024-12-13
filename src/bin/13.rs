use std::cmp::min;

use advent_of_code::{parse, parse_u, vec_lines};
use itertools::Itertools;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    let ans: usize = input
        .split("\n\n")
        .map(|s| {
            let l = vec_lines(s);
            let x1 = parse_u(&l[0][12..14]);
            let y1 = parse_u(&l[0][18..20]);
            let x2 = parse_u(&l[1][12..14]);
            let y2 = parse_u(&l[1][18..20]);
            let (_, s2, yg) = l[2].split("=").collect_tuple().unwrap();
            let (xg, _) = s2.split(",").collect_tuple().unwrap();
            let xg = parse_u(xg);
            let yg = parse_u(yg);
            let mut ans = usize::MAX;
            for aa in 0..=100 {
                for bb in 0..=100 {
                    if x1 * aa + x2 * bb == xg && y1 * aa + y2 * bb == yg {
                        ans = min(ans, aa * 3 + bb);
                    }
                }
            }
            if ans == usize::MAX {
                0
            } else {
                ans
            }
        })
        .sum();
    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let ans: isize = input
        .split("\n\n")
        .map(|s| {
            let l = vec_lines(s);
            let x1 = parse(&l[0][12..14]);
            let y1 = parse(&l[0][18..20]);
            let x2 = parse(&l[1][12..14]);
            let y2 = parse(&l[1][18..20]);
            let (_, s2, yg) = l[2].split("=").collect_tuple().unwrap();
            let (xg, _) = s2.split(",").collect_tuple().unwrap();
            let xg = parse(xg) + 10000000000000;
            let yg = parse(yg) + 10000000000000;

            // a*x1 + b*x2 = xg
            // a*y1 + b*y2 = yg
            // a*y1*x2 - a*x1*y2 = x2*yg - xg*y2
            // a = (x2*yg - xg*y2) / (y1*x2 - x1*y2)
            let an = x2 * yg - xg * y2;
            let ad = y1 * x2 - x1 * y2;
            let mut a = 0;
            if ad != 0 {
                if an % ad != 0 {
                    return 0;
                }
                a = an / ad;
                if a < 0 {
                    return 0;
                }
            }
            // b = (xg - a*x1) / x2
            let bn = xg - a * x1;
            let bd = x2;
            if bn % bd != 0 {
                return 0;
            }
            let b = bn / bd;
            if b < 0 {
                return 0;
            }
            a * 3 + b
        })
        .sum();
    Some(ans as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        // no given test
    }
}
