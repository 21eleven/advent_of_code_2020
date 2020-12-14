
#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (usize, Vec<(usize, usize)>) {
    let lines = input.split('\n').collect::<Vec<&str>>();
    let t = lines[0].parse::<usize>().unwrap();
    let buses = lines[1].split(',').enumerate().filter(|(_i,v)| v != &"x").map(|(i,v)| (i, v.parse::<usize>().unwrap())).collect();
    (t, buses)
}

#[aoc(day13, part1)]
pub fn p1(data: &(usize, Vec<(usize, usize)>)) -> usize {
    let t = data.0;
    let (diff, bus) = data.1.iter().map(|(_i, v)| {
        let mut n = *v;
        while n < t {
            n += v;
        }
        (n - t, v)
    }).min().unwrap();
    diff * bus
}
