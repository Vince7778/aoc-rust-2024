use itertools::Itertools;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<usize> {
    let (p, d) = input.split_once("\n\n").unwrap();
    let p = p.split(", ").collect_vec();
    let mut ans = 0;
    for l in d.lines() {
        let mut x = vec![false; l.len() + 1];
        x[0] = true;
        for i in 0..l.len() {
            if !x[i] {
                continue;
            }
            for pp in p.iter() {
                if i + pp.len() <= l.len() && &&l[i..i + pp.len()] == pp {
                    x[i + pp.len()] = true;
                }
            }
        }
        if x[l.len()] {
            ans += 1;
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (p, d) = input.split_once("\n\n").unwrap();
    let p = p.split(", ").collect_vec();
    let mut ans = 0;
    for l in d.lines() {
        let mut x = vec![0; l.len() + 1];
        x[0] = 1;
        for i in 0..l.len() {
            if x[i] == 0 {
                continue;
            }
            for pp in p.iter() {
                if i + pp.len() <= l.len() && &&l[i..i + pp.len()] == pp {
                    x[i + pp.len()] += x[i];
                }
            }
        }
        ans += x[l.len()];
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
