use advent_of_code::ints;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut l1, mut l2): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|x| -> (i64, i64) {
            match ints(x)[..] {
                [a, b] => (a, b),
                _ => panic!(),
            }
        })
        .unzip();
    l1.sort();
    l2.sort();
    Some(
        l1.into_iter()
            .zip(l2)
            .map(|(a, b)| (a - b).unsigned_abs() as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cnt: Vec<usize> = vec![0; 100_000];
    let l: Vec<usize> = input
        .lines()
        .map(|x| match ints(x)[..] {
            [a, b] => {
                cnt[b as usize] += 1;
                a as usize
            }
            _ => panic!(),
        })
        .collect();
    let res: usize = l.into_iter().map(|x| x * cnt[x]).sum();
    Some(res as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
