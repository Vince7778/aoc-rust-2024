use advent_of_code::{ints, parse};
use itertools::Itertools;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    let ans: isize = input.lines().map(|l| {
        let (x, rest) = l.split(':').collect_tuple().unwrap();
        let x = parse(x);
        let r = ints(rest);
        for i in 0..(1 << (r.len()-1)) {
            let mut cur = r[0];
            for j in 1..r.len() {
                let oper = (i >> (j-1)) % 2;
                match oper {
                    0 => { cur += r[j]; }
                    1 => { cur *= r[j]; }
                    _ => {}
                }
            }
            if cur == x {
                return x;
            }
        }
        return 0;
    }).sum();
    Some(ans as usize)
}

pub fn part_two(input: &str) -> Option<usize> {
    let ans: isize = input.lines().map(|l| {
        let (x, rest) = l.split(':').collect_tuple().unwrap();
        let x = parse(x);
        let r = ints(rest);
        for i in 0..(3_u32.pow((r.len() as u32)-1)) {
            let mut cur = r[0];
            for j in 1..r.len() {
                let oper = (i / 3_u32.pow((j-1) as u32)) % 3;
                match oper {
                    0 => { cur += r[j]; }
                    1 => { cur *= r[j]; }
                    2 => { 
                        let digits = r[j].to_string().len();
                        cur *= 10_isize.pow(digits as u32);
                        cur += r[j];
                    }
                    _ => {}
                }
            }
            if cur == x {
                return x;
            }
        }
        return 0;
    }).sum();
    Some(ans as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
