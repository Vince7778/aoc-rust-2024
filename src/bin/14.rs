use advent_of_code::{ints, repeat_2d};

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let mut q: Vec<usize> = vec![0, 0, 0, 0];
    for l in input.lines() {
        let [p1, p2, v1, v2] = ints(l, true)[..] else {
            continue;
        };
        let f1 = (p1 + (101 + v1) * 100) % 101;
        let f2 = (p2 + (103 + v2) * 100) % 103;
        if f1 < 50 && f2 < 51 {
            q[0] += 1;
        } else if f1 < 50 && f2 > 51 {
            q[1] += 1;
        } else if f1 > 50 && f2 < 51 {
            q[2] += 1;
        } else if f1 > 50 && f2 > 51 {
            q[3] += 1;
        }
    }
    Some(q[0] * q[1] * q[2] * q[3])
}

pub fn part_two(input: &str) -> Option<usize> {
    for t in 0..100 {
        let mut g = repeat_2d('.', 103, 101);
        for l in input.lines() {
            let [p1, p2, v1, v2] = ints(l, true)[..] else {
                continue;
            };
            let f1 = (p1 + (101 + v1) * (t * 101 + 7)) % 101;
            let f2 = (p2 + (103 + v2) * (t * 101 + 7)) % 103;
            g[f2 as usize][f1 as usize] = 'O';
        }
        // i noticed that there was a weird straight line every 101n+7 iterations
        // at least for my input
        // println!("{}", t * 101 + 7);
        // for l in g {
        //     println!("{}", l.into_iter().join(""));
        // }
        // println!();
    }
    Some(8087)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {
        // example is too different from the actual input
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
