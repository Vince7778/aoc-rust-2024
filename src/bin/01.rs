use advent_of_code::pint;
use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let res: i64 = input.lines().map(|x| {
        let cs = x.chars().filter(|c| c.is_digit(10)).collect_vec();
        pint(*cs.first().unwrap()) * 10 + pint(*cs.last().unwrap())
    }).sum();
    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let nums = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let res: i64 = input.lines().map(|x| {
        let vecx = x.chars().collect_vec();
        let tonum = |i: usize| -> Option<i64> {
            for j in 0..nums.len() {
                if x[i..].starts_with(nums[j]) {
                    return Some((j as i64)+1);
                }
            }
            vecx[i].is_digit(10).then(|| pint(vecx[i]))
        };
        let fst = (0..x.len()).filter_map(tonum).next().unwrap();
        let snd = (0..x.len()).rev().filter_map(tonum).next().unwrap();
        fst*10+snd
    }).sum();
    Some(res as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
