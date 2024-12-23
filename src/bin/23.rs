use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;
use rand::{rngs::StdRng, Rng, SeedableRng};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<usize> {
    let mut adj: HashMap<String, HashSet<String>> = HashMap::new();
    let mut c: HashSet<String> = HashSet::new();
    for l in input.lines() {
        let (a, b) = l.split_once('-').unwrap();
        adj.entry(a.to_string()).or_default().insert(b.to_string());
        adj.entry(b.to_string()).or_default().insert(a.to_string());
        c.insert(a.to_string());
        c.insert(b.to_string());
    }
    let mut ans = 0;
    for a in c.iter() {
        for b in adj.get(a).unwrap().iter() {
            if b < a {
                continue;
            }
            for x in adj.get(b).unwrap().iter() {
                if x < b {
                    continue;
                }
                if adj.get(a).unwrap().contains(x)
                    && (a.starts_with('t') || b.starts_with('t') || x.starts_with('t'))
                {
                    ans += 1;
                }
            }
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut adj: HashMap<String, HashSet<String>> = HashMap::new();
    let mut c: HashSet<String> = HashSet::new();
    for l in input.lines() {
        let (a, b) = l.split_once('-').unwrap();
        adj.entry(a.to_string()).or_default().insert(b.to_string());
        adj.entry(b.to_string()).or_default().insert(a.to_string());
        c.insert(a.to_string());
        c.insert(b.to_string());
    }
    let c = c.into_iter().collect_vec();
    let mut rng = StdRng::seed_from_u64(42);
    let mut build_clique = || {
        let mut inc: BTreeSet<String> = BTreeSet::new();
        inc.insert(c[rng.gen_range(0..c.len())].clone());
        loop {
            let mut cand: Vec<String> = Vec::new();
            'bruh: for x in c.iter() {
                if inc.contains(x) {
                    continue;
                }
                for v in inc.iter() {
                    if !adj.get(x).unwrap().contains(v) {
                        continue 'bruh;
                    }
                }
                cand.push(x.clone());
            }
            if cand.is_empty() {
                break;
            }
            inc.insert(cand[rng.gen_range(0..cand.len())].clone());
        }
        (inc.len(), inc)
    };
    let mut max = 0;
    let mut ans = String::new();
    for _ in 0..1000 {
        let (cur, cc) = build_clique();
        if cur > max {
            max = cur;
            ans = cc.into_iter().join(",");
        }
    }
    println!("Found clique of size {}", max);
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".into()));
    }
}
