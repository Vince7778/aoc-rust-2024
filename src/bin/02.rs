use advent_of_code::int_grid;

advent_of_code::solution!(2);

fn test_row(a: &[i64]) -> bool {
    return a.windows(2).all(|v| v[0] < v[1] && v[0] >= v[1] - 3)
        || a.windows(2).all(|v| v[0] > v[1] && v[0] <= v[1] + 3);
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = int_grid(input)
        .into_iter()
        .filter(|r| test_row(r.as_slice()))
        .count();
    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = int_grid(input)
        .into_iter()
        .filter(|r| {
            (0..r.len()).any(|i| {
                let mut removed_r = r.clone();
                removed_r.remove(i);
                test_row(&removed_r)
            })
        })
        .count();
    Some(res as u32)
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
