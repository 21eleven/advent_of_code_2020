
#[aoc_generator(day09)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.split('\n').map(|x| x.parse::<u64>().unwrap()).collect()
}

#[aoc(day09, part1)]
pub fn p1(input: &[u64]) -> u64 {
    for i in 25..input.len() {
        let mut summed = false;
        for j in i-25..i-1 {
            for k in j+1..i {
                if input[j]+input[k] == input[i] {
                    summed = true;
                    break;
                }
            }
            if summed == true {
                break;
            }
        }
        if summed == false {
            return input[i]
        }
    }
    0
}

#[aoc(day09, part2)]
pub fn p2(input: &[u64]) -> u64 {
    let num = p1(input);
    let mut i = 0;
    let mut j = 1;
    loop {
        let s = input[i..j].iter().sum();
        if num == s {
            return input[i..j].iter().max().unwrap()
                 + input[i..j].iter().min().unwrap()
        } else if s > num {
            i += 1;
        } else {
            j += 1;
        }
    }
}
