use std::collections::{HashMap, HashSet};
use nom::IResult;
use nom::bytes::complete::take_until;
use nom::multi::many0;
use nom::bytes::complete::take_while;

pub struct Rule {
    bag: String,
    contains: Vec<(u32, String)>,
}

fn is_fill(c: char) -> bool {
    c.is_whitespace() || c.is_alphabetic() || c == ','
}
fn is_digit_char(c: char) -> bool {
    c.is_digit(10)
}
fn is_space(c: char) -> bool {
    c.is_whitespace() 
}

pub fn parse_member(input: &str) -> IResult<&str, (u32, String)> {
    let (remain, _) = take_while(is_fill)(input)?;
    let (remain, digit) = take_while(is_digit_char)(remain)?;
    let (remain, _) = take_while(is_space)(remain)?;
    let (remain, color) = take_until(" ba")(remain)?;
    let digit = digit.parse::<u32>().unwrap();
    Ok((remain, (digit, color.to_owned())))
}

pub fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (remain, bag) = take_until(" bags contain ")(input)?;
    let (remain, contents) = many0(parse_member)(remain)?;
    Ok((remain, Rule { bag: bag.to_owned(), contains: contents }))
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Rule> {
    let mut rules = vec![];
    for ln in input.split('\n') { 
        let (_, rule) = parse_rule(ln).unwrap();
        rules.push(rule);
    }
    rules
}

#[aoc(day7, part1)]
pub fn p1(input: &[Rule]) -> u32 {
    let mut has: HashMap<String, Vec<String>> = HashMap::new();
    for r in input {
        for (_, color) in r.contains.clone() {
            let parents = has.entry(color).or_insert(vec![]);
            parents.push(r.bag.clone());
        }
    }
    let mut bags: HashSet<String> = HashSet::new();
    fn dfs(color: String, has: &mut HashMap<String, Vec<String>>, bags: &mut HashSet<String>) {
        bags.insert(color.clone());
        match has.get(&color) {
            Some(parents) => {
                for p in parents.clone() {
                    dfs(p.clone(), has, bags);
                }
            }
            None => {}
        }
    }
    dfs(String::from("shiny gold"), &mut has, &mut bags);
    bags.len() as u32 - 1
}

#[aoc(day7, part2)]
pub fn p2(input: &[Rule]) -> u32 {
    let has = input.iter().map(|x| (x.bag.clone(), x.contains.clone())).collect::<HashMap<String, Vec<(u32, String)>>>();
    fn dfs(color: &String, has: &HashMap<String, Vec<(u32, String)>>) -> u32 {
        let mut bag_count = 1;
        let contents = has.get(color).unwrap();
        for (count, bag_color) in contents {
            bag_count += count * dfs(bag_color, has);
        }
        bag_count
    }
    dfs(&String::from("shiny gold"), &has) - 1
}
