use advent_of_code::{parse, repeat_2d};
use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, pats) = input.split("\n\n").collect_tuple().unwrap();
    let mut reqs: Vec<Vec<i64>> = Vec::new();
    for _ in 0..100 {
        reqs.push(Vec::new());
    }
    for l in rules.lines() {
        let (a, b) = l.split("|").map(parse).collect_tuple().unwrap();
        reqs[b as usize].push(a);
    }
    let mut ans = 0;
    for l in pats.lines() {
        let v = l.split(",").map(parse).collect_vec();
        let mut inc: Vec<bool> = vec![false; 100];
        for x in &v {
            inc[*x as usize] = true;
        }
        let mut seen: Vec<bool> = vec![false; 100];
        let mut works = true;
        for x in &v {
            for r in &reqs[*x as usize] {
                if inc[*r as usize] && !seen[*r as usize] {
                    works = false;
                    break;
                }
            }
            if !works {
                break;
            }
            seen[*x as usize] = true;
        }
        if works {
            ans += v[v.len()/2];
        }
    }
    Some(ans as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, pats) = input.split("\n\n").collect_tuple().unwrap();
    let mut reqs: Vec<Vec<i64>> = Vec::new();
    for _ in 0..100 {
        reqs.push(Vec::new());
    }
    for l in rules.lines() {
        let (a, b) = l.split("|").map(parse).collect_tuple().unwrap();
        reqs[b as usize].push(a);
    }
    let mut ans = 0;
    for l in pats.lines() {
        let v = l.split(",").map(parse).collect_vec();
        let mut inc: Vec<bool> = vec![false; 100];
        for x in &v {
            inc[*x as usize] = true;
        }
        let mut seen: Vec<bool> = vec![false; 100];
        let mut works = true;
        for x in &v {
            for r in &reqs[*x as usize] {
                if inc[*r as usize] && !seen[*r as usize] {
                    works = false;
                    break;
                }
            }
            if !works {
                break;
            }
            seen[*x as usize] = true;
        }
        if works {
            continue;
        }
        seen = vec![false; 100];
        for i in 0..v.len()/2+1 {
            let mut ii = -1;
            for x in &v {
                if seen[*x as usize] {
                    continue;
                }
                let mut ww = true;
                for r in &reqs[*x as usize] {
                    if inc[*r as usize] && !seen[*r as usize] {
                        ww = false;
                        break;
                    }
                }
                if ww {
                    ii = *x;
                    break;
                }
            }
            assert!(ii != -1);
            seen[ii as usize] = true;
            if i == v.len()/2 {
                ans += ii;
                break;
            }
        }
    }
    Some(ans as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
