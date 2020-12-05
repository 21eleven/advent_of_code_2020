use nom::bytes::complete::take;
use nom::character::complete::char;
use nom::sequence::terminated;
use nom::multi::many1;
use nom::IResult;

const NEWLINE: char = '\n';

#[derive(Debug, Copy, Clone)]
pub struct BoardingPass {
    row: u32,
    col: u32,
}

impl BoardingPass {
    pub fn id(&self) -> u32 {
        self.row * 8 + self.col
    }
}

pub fn parse_boarding_pass(input: &str) -> IResult<&str, BoardingPass> {

    let (remaining, rowstr) = take(7usize)(input)?;
    let (remaining, colstr) = terminated(take(3usize), char(NEWLINE))(remaining)?;
    let row = u32::from_str_radix(&rowstr.replace('F', "0").replace('B', "1"),2).unwrap();
    let col = u32::from_str_radix(&colstr.replace('L', "0").replace('R', "1"),2).unwrap();

    Ok((remaining, BoardingPass { row, col }))
}

 
#[aoc_generator(day5, part1)]
pub fn input_generator(input: &str) -> Vec<BoardingPass> {
    let (_, passes) = many1(parse_boarding_pass)(input).unwrap();
    passes
}

// #aoc_generator(day5, part2)]
// pub fn input_generator2(input: &str) -> Vec<BoardingPass> {
//     let (_, passes) = many1(parse_boarding_pass)(input).unwrap();
//     passes
// }

#[aoc(day5, part1)]
pub fn p1(input: &[BoardingPass]) -> u32 {
   input.iter().map(|x| x.id()).max().unwrap()//.fold(0, |a,b| std::cmp::max(a,b)) 
}

#[aoc(day5, part2)]
pub fn p2(input: &[BoardingPass]) -> u32 {
    let mut ids = input.iter().map(|x| x.id()).collect::<Vec<u32>>();
    ids.sort_unstable();
    for win in ids.windows(2) {
        let a = win[0];
        let b = win[1];
        if b - a != 1 {
            println!("No pass between {} and {}",a,b);
            return a+1
        }
    }
    0
}
