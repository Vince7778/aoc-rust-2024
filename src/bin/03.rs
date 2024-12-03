use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut ans: u32 = 0;
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        ans += a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul)\((\d{1,3}),(\d{1,3})\)|(do)\(\)()()|(don't)\(\)()()").unwrap();
    let mut ans: u32 = 0;
    let mut enabled: bool = true;
    for (_, [ty, a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        match ty {
            "mul" => {
                if enabled {
                    ans += a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
                }
            }
            "do" => {
                enabled = true;
            }
            "don't" => {
                enabled = false;
            }
            _ => panic!()
        }
    }
    Some(ans)
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
        let result: Option<u32> = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
