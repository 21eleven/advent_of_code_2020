use std::collections::HashSet;

use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till};
use nom::multi::many0;
use nom::bytes::complete::take_while;
use nom::lib::std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Op {
    ACC,
    JMP,
    NOP,
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acc" => Ok(Op::ACC),
            "jmp" => Ok(Op::JMP),
            "nop" => Ok(Op::NOP),
            _ => Err(format!("Unsupported instruction: {}", s)),
        }
    }
}

fn is_space(c: char) -> bool {
    c.is_whitespace() 
}


fn parse_op(input: &str) -> IResult<&str, (Op, i32)> {
    let (remain, op_str) = alt((
            tag("acc"), 
            tag("jmp"), 
            tag("nop")))(input)?;
     
    let op = Op::from_str(op_str).unwrap();
    let (remain, _) = take_while(is_space)(remain)?;
    let (remain, number) = take_till(|c| c == '\n')(remain)?;
    let (remain, _) = take_while(is_space)(remain)?;
    let number = number.parse::<i32>().unwrap();

    Ok((remain, (op, number)))
}


#[aoc_generator(day08)]
pub fn input_generator(input: &str) -> Vec<(Op, i32)> {
    let (_, ops) = many0(parse_op)(input).unwrap();
    ops
}

#[aoc(day08, part1)]
pub fn p1(input: &[(Op, i32)]) -> i32 {
    let mut i = 0;
    let mut acc = 0;
    let mut seen = HashSet::new();
    while !seen.contains(&i) {
        let (op, n) = input[i as usize];
        seen.insert(i);
        match op {
            Op::ACC => {
                acc += n;
                i += 1;
            },
            Op::JMP => {
                i += n;
            },
            Op::NOP => {
                i += 1;
            }
        } 
    }
    acc
}
#[aoc(day08, part2)]
pub fn p2(input: &[(Op, i32)]) -> i32 {
    for j in 0..input.len() {
        let mut i = 0i32;
        let mut acc = 0;
        let mut seen = HashSet::new();
        while !seen.contains(&i) {
            if i == input.len() as i32 {
                return acc
            }
            let (op, n) = input[i as usize];
            seen.insert(i);
            match op {
                Op::ACC => {
                    acc += n;
                    i += 1;
                },
                Op::JMP => {
                    if i == j as i32 {
                        i += 1;
                    } else {
                        i += n;
                    }
                },
                Op::NOP => {
                    if i == j as i32 {
                        i += n;
                    } else {
                        i += 1;
                    }
                }
            } 
        }
    }
    -1
}
