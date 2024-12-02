use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let res: u32 = input.lines().map(|l| {
        let a: Vec<i32> = l.split_whitespace().map(|x| x.parse().unwrap()).collect_vec();
        if (a.len() <= 1) {return 1;}
        if (a[0] == a[1]) {return 0;}
        if (a[0] < a[1]) {
            for i in 1..a.len() {
                if a[i-1] >= a[i] || a[i-1] <= a[i]-4 {
                    return 0;
                }
            }
            return 1;
        } else {
            for i in 1..a.len() {
                if a[i-1] <= a[i] || a[i-1] >= a[i]+4 {
                    return 0;
                }
            }
            return 1;
        }
    }).sum();
    Some(res)
}

fn tt(a: &[i32]) -> u32 {
    if (a.len() <= 1) {return 1;}
    if (a[0] == a[1]) {return 0;}
    if (a[0] < a[1]) {
        for i in 1..a.len() {
            if a[i-1] >= a[i] || a[i-1] <= a[i]-4 {
                return 0;
            }
        }
        return 1;
    } else {
        for i in 1..a.len() {
            if a[i-1] <= a[i] || a[i-1] >= a[i]+4 {
                return 0;
            }
        }
        return 1;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let res: u32 = input.lines().map(|l| {
        let a: Vec<i32> = l.split_whitespace().map(|x| x.parse().unwrap()).collect_vec();
        for i in 0..a.len() {
            let aa = a.iter().cloned().enumerate().filter_map(|(ii, v)| {
                if ii == i {
                    None
                } else {
                    Some(v)
                }
            }).collect_vec();
            if (tt(&aa) == 1) {
                return 1;
            }
        }
        return 0;
    }).sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
