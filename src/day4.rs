use nom::branch::{alt, permutation};
use nom::bytes::complete::{tag, take_while, take_while_m_n};
use nom::character::complete::char;
use nom::lib::std::str::FromStr;
use nom::combinator::map_res;
use nom::sequence::{separated_pair, tuple};
use nom::{AsChar, IResult};

const BYR: &str = "byr";
const IYR: &str = "iyr";
const EYR: &str = "eyr";
const ECL: &str = "ecl";
const HCL: &str = "hcl";
const PID: &str = "pid";
const CID: &str = "cid";
const HGT: &str = "hgt";
const COLON: char = ':';
const SPACE: char = ' ';
const NEWLINE: char = '\n';

#[derive(Debug, Eq, PartialEq)]
struct Year {
    value: u32,
}

impl Year {
    pub fn new(year: u32) -> Year {
        Year { value: year }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum EyeColor {
    Amber,
    Brown,
    Gray,
    Blue,
    Green,
    Hazel,
    Other,
}

impl FromStr for EyeColor {
    type Err = String;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        match code {
            "amb" => Ok(EyeColor::Amber),
            "blu" => Ok(EyeColor::Blue),
            "grn" => Ok(EyeColor::Green),
            "brn" => Ok(EyeColor::Brown),
            "gry" => Ok(EyeColor::Gray),
            "hzl" => Ok(EyeColor::Hazel),
            "oth" => Ok(EyeColor::Other),
            code => Err(format!("Unsupported color code {}", code)),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b:u8) -> Color {
        Color { r, g, b }
    }
}

#[derive(Debug, PartialEq)]
struct Height {
    value: f64,
}

impl Height {
    pub fn from_inches(value: u32) -> Height {
        let value = value as f64;
        Height { value }
    }
    pub fn from_cm(value: u32) -> Height {
        let inches = value as f64 / 2.54;
        Height { value: inches }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct PassportId {
    value: String,
}

#[derive(Debug, Eq, PartialEq)]
struct CountryId {
    value: String,
}

#[derive(Debug, Eq, PartialEq)]
struct PassString {
    value: String
}

#[derive(Debug)]
pub struct Pass {
    byr: PassString,
    iyr: PassString,
    eyr: PassString,
    ecl: PassString,
    hcl: PassString,
    hgt: PassString,
    pid: PassString,
    cid: Option<PassString>,
}

#[derive(Debug)]
pub struct Passport {
    byr: Year,
    iyr: Year,
    eyr: Year,
    ecl: EyeColor,
    hcl: Color,
    hgt: Height,
    pid: PassportId,
    cid: Option<CountryId>,
}

impl Passport {
    pub fn is_valid(&self) -> bool {
        if !(self.byr.value >= 1920 && self.byr.value <= 2002) {
            return false;
        }
        if !(self.iyr.value >= 2010 && self.iyr.value <= 2020) {
            return false;
        }
        if !(self.eyr.value >= 2020 && self.eyr.value <= 2030) {
            return false;
        }
        if !(self.hgt.value >= 59.0 && self.hgt.value <= 76.0) {
            return false;
        }
        true
    }
}

fn parse_pass(input: &str) -> IResult<&str, Pass> {
    let (remaining, (cid, pid, iyr, eyr, byr, ecl, hcl, hgt)) = permutation((
        pass_field_cid,
        pass_field_pid,
        pass_field_iyr,
        pass_field_eyr,
        pass_field_byr,
        pass_field_ecl,
        pass_field_hcl,
        pass_field_hgt,
    ))(input)?;

    Ok((
        remaining,
        Pass {
            iyr,
            eyr,
            byr,
            pid,
            hcl,
            ecl,
            hgt,
            cid,
        },
    ))
}

fn pass_field_cid(input: &str) -> IResult<&str, Option<PassString>> {
    if input.is_empty() {
        return Ok((input, None));
    }
    let (remaining, (_, cid)) = separated_pair(tag(CID), char(COLON), pass_field)(input)?;
    let (remaining, _) = remove_trailing_whitespace(remaining)?;
    Ok((remaining, Some(cid)))
}

fn pass_field(input: &str) -> IResult<&str, PassString> {
    let (remaining, txt) = take_while(is_not_whitespace)(input)?;
    Ok((
        remaining,
        PassString {
            value: txt.to_owned(),
        },
    ))
}

fn is_not_whitespace(c: char) -> bool {
    !c.is_whitespace()
}

fn pass_field_pid(input: &str) -> IResult<&str, PassString> {
    let (remaining, (_, pid)) = separated_pair(tag(PID), char(COLON), pass_field)(input)?;
    
    let (remaining, _) = remove_trailing_whitespace(remaining)?;

    Ok((remaining, pid))
}

fn pass_field_iyr(input: &str) -> IResult<&str, PassString> {
    let (remaining, (_, iyr)) = separated_pair(tag(IYR), char(COLON), pass_field)(input)?;
    
    let (remaining, _) = remove_trailing_whitespace(remaining)?;

    Ok((remaining, iyr))
}

fn pass_field_eyr(input: &str) -> IResult<&str, PassString> {
    let (remaining, (_, eyr)) = separated_pair(tag(EYR), char(COLON), pass_field)(input)?;
    
    let (remaining, _) = remove_trailing_whitespace(remaining)?;

    Ok((remaining, eyr))
}

fn pass_field_byr(input: &str) -> IResult<&str, PassString> {
    let (remaining, (_, byr)) = separated_pair(tag(BYR), char(COLON), pass_field)(input)?;
    
    let (remaining, _) = remove_trailing_whitespace(remaining)?;

    Ok((remaining, byr))
}

fn pass_field_ecl(input: &str) -> IResult<&str, PassString> {
    let (remaining, (_, ecl)) = separated_pair(tag(ECL), char(COLON), pass_field)(input)?;
    
    let (remaining, _) = remove_trailing_whitespace(remaining)?;

    Ok((remaining, ecl))
}

fn pass_field_hcl(input: &str) -> IResult<&str, PassString> {
    let (remaining, (_, hcl)) = separated_pair(tag(HCL), char(COLON), pass_field)(input)?;
    
    let (remaining, _) = remove_trailing_whitespace(remaining)?;

    Ok((remaining, hcl))
}

fn pass_field_hgt(input: &str) -> IResult<&str, PassString> {
    let (remaining, (_, hgt)) = separated_pair(tag(HGT), char(COLON), pass_field)(input)?;
    
    let (remaining, _) = remove_trailing_whitespace(remaining)?;

    Ok((remaining, hgt))
}

fn parse_passport(input: &str) -> IResult<&str, Passport> {
    let (remaining, (cid, pid, iyr, eyr, byr, ecl, hcl, hgt)) = permutation((
        parse_field_cid,
        parse_field_pid,
        parse_field_iyr,
        parse_field_eyr,
        parse_field_byr,
        parse_field_ecl,
        parse_field_hcl,
        parse_field_hgt,
    ))(input)?;

    Ok((
        remaining,
        Passport {
            iyr,
            eyr,
            byr,
            pid,
            hcl,
            ecl,
            hgt,
            cid,
        },
    ))
}

fn parse_field_cid(input: &str) -> IResult<&str, Option<CountryId>> {
    if input.is_empty() {
        return Ok((input, None));
    }
    let (remaining, (_, cid)) = separated_pair(tag(CID), char(COLON), country_id)(input)?;
    let (remaining, _) = remove_trailing_whitespace(remaining)?;
    Ok((remaining, Some(cid)))
}

fn country_id(input: &str) -> IResult<&str, CountryId> {
    let (remaining, cid) = take_while(is_digit)(input)?;
    Ok((
        remaining,
        CountryId {
            value: cid.to_owned(),
        },
    ))
}

fn parse_field_pid(input: &str) -> IResult<&str, PassportId> {
    let (remaining, (_, pid)) = separated_pair(tag(PID), char(COLON), passport_id)(input)?;
    
    let (remaining, _) = remove_trailing_whitespace(remaining)?;

    Ok((remaining, pid))
}

fn passport_id(input: &str) -> IResult<&str, PassportId> {
    let (remaining, pid) = take_while_m_n(9, 9, is_digit)(input)?;

    Ok((
        remaining,
        PassportId {
            value: pid.to_owned(),
        }
    ))
}

fn parse_field_ecl(input: &str) -> IResult<&str, EyeColor> {
    let (remaining, (_, clr)) = separated_pair(tag(ECL), char(COLON), eye_color_code)(input)?;

    let (remaining, _) = remove_trailing_whitespace(remaining)?;

    Ok((remaining, clr))
}

fn eye_color_code(input: &str) -> IResult<&str, EyeColor> {
    let (remaining, code) = alt((
        tag("amb"),
        tag("blu"),
        tag("brn"),
        tag("gry"),
        tag("hzl"),
        tag("grn"),
        tag("oth"),
    ))(input)?;

    let color = EyeColor::from_str(code).map_err(|_e| {
        nom::Err::Failure(nom::error::Error::new(code, nom::error::ErrorKind::Tag))
    })?;

    Ok((remaining, color))
}

fn parse_field_hgt(input: &str) -> IResult<&str, Height> {
    let (remaining, (_, height)) = separated_pair(tag(HGT), char(COLON), parse_height)(input)?;

    let (remaining, _) = remove_trailing_whitespace(remaining)?;
    Ok((remaining, height))
}

fn parse_height(input: &str) -> IResult<&str, Height> {
    let (input, num) = map_res(take_while_m_n(1, 3, is_digit), u32_from_str)(input)?;
    let (input, unit) = alt((tag("cm"), tag("in")))(input)?;

    let height = if "in" == unit {
        Height::from_inches(num)
    } else {
        Height::from_cm(num)
    };

    Ok((input, height))
}

fn parse_field_byr(input: &str) -> IResult<&str, Year> {
    let (remaining, (_, year)) = separated_pair(tag(BYR), char(COLON), parse_year)(input)?;

    let (remaining, _) = remove_trailing_whitespace(remaining)?;
    Ok((remaining, year))
}

fn parse_field_eyr(input: &str) -> IResult<&str, Year> {
    let (remaining, (_, year)) = separated_pair(tag(EYR), char(COLON), parse_year)(input)?;

    let (remaining, _) = remove_trailing_whitespace(remaining)?;
    Ok((remaining, year))
}

fn parse_field_iyr(input: &str) -> IResult<&str, Year> {
    let (remaining, (_, year)) = separated_pair(tag(IYR), char(COLON), parse_year)(input)?;

    let (remaining, _) = remove_trailing_whitespace(remaining)?;
    Ok((remaining, year))
}

fn parse_year(input: &str) -> IResult<&str, Year> {
    let (input, num) = map_res(take_while_m_n(4,4, is_digit), u32_from_str)(input)?;
    Ok((input, Year::new(num)))
}

fn parse_field_hcl(input: &str) -> IResult<&str, Color> {
    let (remaining, (_, color)) = separated_pair(tag(HCL), char(COLON), parse_hex_color)(input)?;
    
    let (remaining, _) = remove_trailing_whitespace(remaining)?;
    Ok((remaining, color))
}

fn parse_hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (r, g, b)) = tuple((hex_color_code, hex_color_code, hex_color_code))(input)?;

    Ok((input, Color::new(r, g, b)))
}

fn hex_color_code(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), u8_from_hex)(input)
}

fn u8_from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_hex_digit()
}

fn u32_from_str(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.parse()
}

fn is_digit(c: char) -> bool {
    c.is_dec_digit()
}

fn remove_trailing_whitespace(input: &str) -> IResult<&str, ()> {
    let (remaining, _) = alt((
        take_while_m_n(1, 1, |x| x == NEWLINE),
        take_while(|x| x == SPACE),
    ))(input)?;

    Ok((remaining, ()))
}

#[aoc_generator(day4, part1)]
pub fn input_generator(input: &str) -> Vec<Pass> {
    let mut lines = vec![];

    for ln in input.split("\n\n") {
        match parse_pass(ln.trim()) {
            Ok((_, passport)) => {
                lines.push(passport);
            }
            Err(_e) => {
		// println!("Malformed passport data: {:?}:\n{}\n", e, ln);
            }
        }
    }
    lines
}

#[aoc_generator(day4, part2)]
pub fn input_generator2(input: &str) -> Vec<Passport> {
    let mut lines = vec![];

    for ln in input.split("\n\n") {
        match parse_passport(ln.trim()) {
            Ok((_, passport)) => {
                lines.push(passport);
            }
            Err(_e) => {
		// println!("Malformed passport data: {:?}:\n{}\n", e, ln);
            }
        }
    }
    lines
}

#[aoc(day4, part1)]
pub fn p1(input: &[Pass]) -> u32 {
   // dbg!(&input[0]);
   input.len() as u32
}

#[aoc(day4, part2)]
pub fn p2(input: &[Passport]) -> u32 {
    let mut valid = 0;
    for p in input {
        if p.is_valid() {
            valid += 1;
        } else {
            if cfg!(debug_assertions) {
                println!("Invalid passport: {:#?}", p);
            }
        }
    }
    valid
}
