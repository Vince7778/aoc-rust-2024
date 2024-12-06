use advent_of_code::parse;
use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let res: isize = re
        .captures_iter(input)
        .map(|c| c.extract::<2>())
        .map(|(_, [a, b])| parse(a) * parse(b))
        .sum();
    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul)\((\d{1,3}),(\d{1,3})\)|(do(?:n't)?)\(\)()()").unwrap();
    let mut enabled = true;
    let res: isize = re
        .captures_iter(input)
        .map(|c| c.extract::<3>())
        .filter_map(|(_, [ty, a, b])| match ty {
            "mul" => enabled.then_some(parse(a) * parse(b)),
            "do" => {
                enabled = true;
                None
            }
            "don't" => {
                enabled = false;
                None
            }
            _ => panic!("unexpected type"),
        })
        .sum();
    Some(res as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u32> = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
