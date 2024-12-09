use advent_of_code::{parse, parse_char};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let mut v: Vec<isize> = vec![-1; input.len()*10];
    let mut vi = 0;
    for (i, c) in input.lines().next().unwrap().chars().enumerate() {
        let cc = parse_char(c);
        for j in 0..cc {
            if i % 2 == 0 {
                v[vi] = (i as isize)/2;
            }
            vi += 1;
        }
    }
    let ovi = vi;
    vi = 0;
    for i in (0..ovi).rev() {
        if v[i] != -1 {
            while vi < i && v[vi] != -1 {
                vi += 1;
            }
            if vi >= i {
                break;
            }
            v[vi] = v[i];
            v[i] = -1;
        }
    }
    let mut ans = 0;
    for i in 0..v.len() {
        if v[i] != -1 {
            ans += i*(v[i] as usize);
        }
    }
    Some(ans as usize)
}

#[derive(Clone, Copy, Debug)]
struct Block {
    pub i: usize,
    pub v: usize,
    pub len: usize,
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut v: Vec<Block> = Vec::new();
    let mut vf: Vec<Block> = Vec::new();
    let mut vi = 0;
    for (i, c) in input.lines().next().unwrap().chars().enumerate() {
        let cc = parse_char(c);
        if i % 2 == 0 {
            vf.push(Block { i: vi, v: i/2, len: cc as usize});
        } else {
            v.push(Block { i: vi, v: 0, len: cc as usize});
        }
        vi += cc as usize;
    }
    let mut vfn: Vec<Block> = Vec::new();
    for &b in vf.iter().rev() {
        let mut found = false;
        for xi in 0..v.len() {
            let x = v[xi];
            if x.i >= b.i {
                break;
            }
            if x.len < b.len {
                continue;
            }
            found = true;
            vfn.push(Block { i: x.i, v: b.v, len: b.len });
            v.remove(xi);
            if b.len < x.len {
                v.insert(xi, Block { i: x.i + b.len, v: 0, len: x.len - b.len});
            }
            break;
        }
        if !found {
            vfn.push(b);
        }
    }
    let mut ans = 0;
    for &b in vfn.iter() {
        for i in b.i..(b.i+b.len) {
            ans += i*b.v;
        }
        println!("{:?}", b);
    }
    Some(ans)
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
