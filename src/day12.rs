use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::bytes::complete::take_till;
use nom::character::complete::char;
use nom::multi::many0;
use nom::IResult;

fn is_space(c: char) -> bool {
    c.is_whitespace()
}

fn parse_dir(input: &str) -> IResult<&str, (char, i32)> {
    let (remain, dir) = alt((
        char('N'),
        char('E'),
        char('S'),
        char('W'),
        char('R'),
        char('L'),
        char('F'),
    ))(input)?;

    let (remain, number) = take_till(|c| c == '\n')(remain)?;
    let (remain, _) = take_while(is_space)(remain)?;
    let number = number.parse::<i32>().unwrap();

    Ok((remain, (dir, number)))
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<(char, i32)> {
    let (_, dirs) = many0(parse_dir)(input).unwrap();
    dirs
}

fn go(d: &char, val: i32, x: &mut i32, y: &mut i32) {
    if d == &'E' {
        *x += val;
    } else if d == &'W' {
        *x -= val;
    } else if d == &'N' {
        *y += val;
    } else if d == &'S' {
        *y -= val;
    }
}

fn rrotate(d: &char, amt: i32) -> char {
    let dirs = ['N', 'E', 'S', 'W'];
    let mut index = dirs.iter().position(|&c| c == *d).unwrap();
    let quad = (amt / 90) as usize;
    index += quad;
    index %= 4;
    dirs[index]
}

fn lrotate(d: &char, amt: i32) -> char {
    let dirs = ['W', 'S', 'E', 'N'];
    let mut index = dirs.iter().position(|&c| c == *d).unwrap();
    let quad = (amt / 90) as usize;
    index += quad;
    index %= 4;
    dirs[index]
}

#[aoc(day12, part1)]
pub fn p1(input: &[(char, i32)]) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut d = 'E';

    for (c, val) in input {
        match c {
            'F' => go(&d, *val, &mut x, &mut y),
            'R' => d = rrotate(&mut d, *val),
            'L' => d = lrotate(&mut d, *val),
            _ => go(&c, *val, &mut x, &mut y),
        }

    }
    x.abs()+y.abs()
}

fn l(wx: &mut i32, wy: &mut i32) {
    *wx ^= *wy;
    *wy ^= *wx;
    *wx ^= *wy;
    *wx *= -1;
}

fn r(wx: &mut i32, wy: &mut i32) {
    *wx ^= *wy;
    *wy ^= *wx;
    *wx ^= *wy;
    *wy *= -1;
}

fn follow_waypoint(val: i32, x: &mut i32, y: &mut i32, wx: &i32, wy: &i32) {
    *x += *wx * val;
    *y += *wy * val;
}

#[aoc(day12, part2)]
pub fn p2(input: &[(char, i32)]) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut wx = 10;
    let mut wy = 1;
    
    for (c, val) in input {
        match c {
            'F' => follow_waypoint(*val, &mut x, &mut y, &wx, &wy),
            'R' => {
                for _ in 0..*val/90 {
                    r(&mut wx, &mut wy);
                }
            }, 
            'L' => {
                for _ in 0..*val/90 {
                    l(&mut wx, &mut wy);
                }
            }, 
            _ => go(c, *val, &mut wx, &mut wy),
        }
    }
    x.abs()+y.abs()
}
