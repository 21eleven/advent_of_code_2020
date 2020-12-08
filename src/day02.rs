type Policy = (usize, usize, char, String);

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Policy> {
    input
        .lines()
        .map(|l| {
            let (n1, l2) = l.split_once("-").unwrap();
            let n1 = n1.parse::<usize>().unwrap();
            let (n2, l3) = l2.split_once(" ").unwrap();
            let n2 = n2.parse::<usize>().unwrap();
            let (c, l4) = l3.split_once(":").unwrap();
            let c = c.chars().next().unwrap();
            let s = String::from(l4.trim());
            (n1, n2, c, s)
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn p1(input: &[Policy]) -> usize {
    input
        .into_iter()
        .map(
            |(a, b, l, s)| 
            (a..=b).contains(
                &&s.chars().filter(|ch| ch == l).count()
            )
        )
        .filter(|x| x == &true)
        .count()
}


#[aoc(day2, part2)]
pub fn p2(input: &[Policy]) -> usize {
    input
        .into_iter()
        .map(
            |(a, b, l, s)| 
            (&(s.chars().nth(a-1).unwrap()) == l)
            ^
            (&(s.chars().nth(b-1).unwrap()) == l)
        )
        .filter(|x| x == &true).count()
}
