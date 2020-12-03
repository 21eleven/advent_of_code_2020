#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    let mut lines = vec![];

    for ln in input.split("\n") {
        lines.push(String::from(ln));
    }
    lines
}


#[aoc(day3, part1)]
pub fn p1(input: &[String]) -> usize {
    count_trees(input, 3, 1)
 }

fn count_trees(input: &[String], right: usize, down: usize) -> usize {
    let length = input[0].len();
    let mut i = 0;
    let mut j = 0;
    let mut trees = 0;
    loop {
        i += down;
        j += right;
        j %= length;
        if i < input.len() {
            if input[i].as_bytes()[j] == b'#' {
                trees += 1
            }
        } else {break}
    }
    trees
}


#[aoc(day3, part2)]
pub fn p2(input: &[String]) -> usize {
    count_trees(input, 1,1)*count_trees(input, 3, 1)*count_trees(input, 5, 1)*count_trees(input, 7, 1)*count_trees(input, 1, 2)
 }
