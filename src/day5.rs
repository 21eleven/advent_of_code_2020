use nom::bytes::complete::{take, tag};
use nom::character::complete::char;
use nom::sequence::terminated;
use nom::multi::many1;
use nom::IResult;

const NEWLINE: char = '\n';

#[derive(Debug)]
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
    let (remaining, colstr) = take(3usize)(remaining)?;
    // let (remaining, colstr) = terminated(take(3usize), char(NEWLINE))(remaining)?;
    let row = u32::from_str_radix(&rowstr.replace('F', "0").replace('B', "1"),2).unwrap();
    let col = u32::from_str_radix(&colstr.replace('L', "0").replace('R', "1"),2).unwrap();

    Ok((remaining, BoardingPass { row, col }))
}

 
#[aoc_generator(day5, part1)]
pub fn input_generator(input: &str) -> Vec<BoardingPass> {
    // match many1(parse_boarding_pass)(input) {
    //     Ok((_, passes)) => passes,
    //     Err(e) => {
    //         dbg!(e);
    //         vec![]
    //     }
    // }
    // let mut lines = vec![];

    for ln in input.split("\n") {
        dbg!(&ln);
        match parse_boarding_pass(ln.trim()) {
            Ok((_, pass)) => {
                lines.push(pass);
            }
            Err(e) => {
                dbg!(e);
            }
        }
    }
    lines
}

#[aoc(day5, part1)]
pub fn p1(input: &[BoardingPass]) -> u32 {
   input.iter().map(|x| x.id()).fold(0, |a,b| std::cmp::max(a,b)) 
}
