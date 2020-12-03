use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    let mut output = input
        .lines()
        .map(|n| n.trim().parse().unwrap())
        .collect::<Vec<u32>>();
    output.sort_unstable();
    output
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    for a in input.iter() {
        for b in input.iter() {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    0
}

#[aoc(day1, part1, v2)]
pub fn solve_part1_v2(input: &[u32]) -> u32 {
    let mut chk = HashSet::new();
    for n in input {
        if chk.len() == 0 {
            chk.insert(n);
        } else {
            if chk.contains(&(2020 - n)) {
                return n * (2020 - n);
            } else {
                chk.insert(n);
            }
        }
    }
    0
}

#[aoc(day1, part2)]
pub fn p2(input: &[u32]) -> u32 {
    let n = input.len();
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                let x = input[i] + input[j] + input[k];
                if x == 2020 {
                    return input[i] * input[j] * input[k];
                } else if x > 2020 {
                    break;
                } else {
                    continue;
                }
            }
        }
    }
    0
}
