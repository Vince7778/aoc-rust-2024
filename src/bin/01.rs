use advent_of_code::int_grid;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut l1, mut l2): (Vec<_>, Vec<_>) =
        int_grid(input).into_iter().map(|x| (x[0], x[1])).unzip();
    l1.sort();
    l2.sort();
    let res: u64 = l1
        .into_iter()
        .zip(l2)
        .map(|(a, b)| (a - b).unsigned_abs())
        .sum();
    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cnt: Vec<usize> = vec![0; 100_000];
    let l: Vec<usize> = int_grid(input)
        .into_iter()
        .map(|x| {
            cnt[x[1] as usize] += 1;
            x[0] as usize
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
