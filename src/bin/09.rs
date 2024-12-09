use advent_of_code::parse_char;

advent_of_code::solution!(9);

#[derive(Clone, Copy, Debug)]
struct Block {
    pub start: usize,
    pub end: usize,
    pub v: usize,
}

impl Block {
    pub fn new(start: usize, end: usize, v: usize) -> Block {
        Block { start, end, v }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn score(&self) -> usize {
        let l = self.len();
        self.v * l * (2 * self.start + l - 1) / 2
    }
}

fn optimize_blocks(blocks: Vec<Block>, mut free: Vec<Block>) -> Vec<Block> {
    let mut res: Vec<Block> = Vec::new();
    for b in blocks.into_iter().rev() {
        let mut found = false;
        for i in 0..free.len() {
            if free[i].start >= b.start {
                break;
            }
            if free[i].len() >= b.len() {
                found = true;
                res.push(Block::new(free[i].start, free[i].start + b.len(), b.v));
                if free[i].len() > b.len() {
                    free[i].start += b.len();
                } else {
                    free.remove(i);
                }
                break;
            }
        }
        if !found {
            res.push(b);
        }
    }
    res
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut blocks: Vec<Block> = Vec::new();
    let mut free_blocks: Vec<Block> = Vec::new();
    input.trim().chars().enumerate().fold(0, |a, (i, c)| {
        let cc = parse_char(c) as usize;
        for bi in 0..cc {
            let b = Block::new(a + bi, a + bi + 1, i / 2);
            if i % 2 == 0 {
                blocks.push(b);
            } else {
                free_blocks.push(b);
            }
        }
        a + cc
    });
    let res = optimize_blocks(blocks, free_blocks);
    Some(res.into_iter().map(|b| b.score()).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut blocks: Vec<Block> = Vec::new();
    let mut free_blocks: Vec<Block> = Vec::new();
    input.trim().chars().enumerate().fold(0, |a, (i, c)| {
        let cc = parse_char(c) as usize;
        let b = Block::new(a, a + cc, i / 2);
        if i % 2 == 0 {
            blocks.push(b);
        } else {
            free_blocks.push(b);
        }
        a + cc
    });
    let res = optimize_blocks(blocks, free_blocks);
    Some(res.into_iter().map(|b| b.score()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
