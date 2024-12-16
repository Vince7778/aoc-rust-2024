use std::collections::{BinaryHeap, HashMap, HashSet};

use advent_of_code::{grid, run_move, DD};

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<usize> {
    let g = grid(input);
    let s = (g.len()-2, 1);
    let e = (1, g[0].len()-2);
    
    let mut dv: HashMap<((usize, usize), usize), isize> = HashMap::new();
    let mut pq: BinaryHeap<(isize, (usize, usize), usize)> = BinaryHeap::new();
    pq.push((0, s, 1));
    while let Some((dist, p, d)) = pq.pop() {
        let dist = -dist;
        if p == e {
            return Some(dist as usize);
        }
        if dv.get(&(p, d)).is_some_and(|&x| x > dist) {
            continue;
        }
        let nxt = vec![
            (dist+1, run_move(p, DD[d]), d),
            (dist+1000, p, (d+1)%4),
            (dist+1000, p, (d+3)%4),
        ];
        for (ndist, np, nd) in nxt {
            if g[np.0][np.1] != '#' && !dv.get(&(np, nd)).is_some_and(|&cd| cd < ndist) {
                dv.insert((np, nd), ndist);
                pq.push((-ndist, np, nd));
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let g = grid(input);
    let s = (g.len()-2, 1);
    let e = (1, g[0].len()-2);
    
    let mut dv: HashMap<((usize, usize), usize), isize> = HashMap::new();
    let mut de = isize::MAX;
    let mut pq: BinaryHeap<(isize, (usize, usize), usize)> = BinaryHeap::new();
    let mut prevs: HashMap<((usize, usize), usize), Vec<((usize, usize), usize)>> = HashMap::new();
    pq.push((0, s, 1));
    while let Some((dist, p, d)) = pq.pop() {
        let dist = -dist;
        if p == e {
            de = dist;
        }
        if dist > de {
            break;
        }
        if dv.get(&(p, d)).is_some_and(|&x| x > dist) {
            continue;
        }
        let nxt = vec![
            (dist+1, run_move(p, DD[d]), d),
            (dist+1000, p, (d+1)%4),
            (dist+1000, p, (d+3)%4),
        ];
        for (ndist, np, nd) in nxt {
            if g[np.0][np.1] != '#' && !dv.get(&(np, nd)).is_some_and(|&cd| cd < ndist) {
                if dv.get(&(np, nd)).is_some_and(|&cd| cd > ndist) {
                    prevs.get_mut(&(np, nd)).unwrap().clear();
                }
                prevs.entry((np, nd)).or_default().push((p, d));
                dv.insert((np, nd), ndist);
                pq.push((-ndist, np, nd));
            }
        }
    }
    let mut spots: HashSet<(usize, usize)> = HashSet::new();
    let mut vis: HashSet<((usize, usize), usize)> = HashSet::new();
    let mut stk: Vec<((usize, usize), usize)> = Vec::new();
    for i in 0..4 {
        stk.push((e, i));
        vis.insert((e, i));
    }
    spots.insert(e);
    while let Some(p) = stk.pop() {
        if let Some(pv) = prevs.get(&p) {
            for &np in pv {
                if vis.insert(np) {
                    spots.insert(np.0);
                    stk.push(np);
                }
            }
        }
    }
    Some(spots.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(64));
    }
}
