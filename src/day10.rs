use std::collections::{HashMap, HashSet};

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<u64> {
    let mut input: Vec<u64> = input.split('\n').map(|x| x.parse::<u64>().unwrap()).collect();
    input.push(0);
    input.sort_unstable();
    let m = *input.iter().max().unwrap();
    input.push(m+3);
    input
}

#[aoc(day10, part1)]
pub fn p1(input: &[u64]) -> usize {
    let diffs: Vec<u64> = input.windows(2).map(|x| x[1]-x[0]).collect();
    diffs.iter().filter(|x| x==&&1).count()
        *
    diffs.iter().filter(|x| x==&&3).count()
}

// #[aoc(day10, part2)]
// pub fn p2(input: &[u64]) -> usize {
//     let mut memo: HashMap<u64, u64> = HashMap::new(); 
//     let target = input.iter().max().unwrap();
//     let seen = input.iter().map(|x| *x).collect::<HashSet<u64>>();
//     fn dp(pos: u64, target: u64, seen: &HashSet<64>, memo: &mut Hash)
// }


