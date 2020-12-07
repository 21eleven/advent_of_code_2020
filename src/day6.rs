use nom::bytes::complete::take_while;
use nom::character::complete::newline;
use nom::multi::many1;
use nom::IResult;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Person {
    questions: HashSet<char>,
}

#[derive(Debug, Clone)]
pub struct Group {
    members: Vec<Person>,
}

impl Group {
    pub fn total_questions(&self) -> HashSet<char> {
        let mut total = HashSet::new();
        for p in self.members.clone() {
            total.extend(p.questions)
        }
        total
    }
    pub fn questions_answered_by_all(&self) -> usize {
        let total = self.total_questions();
        let mut see_by_all = 0;
        for c in total {
            let mut all = true;
            for p in self.members.clone() {
                if !p.questions.contains(&c) {
                    all = false;
                    break;
                }
            }
            if all {
                see_by_all += 1;
            }
        }
        see_by_all
    }
}

pub fn parse_person(input: &str) -> IResult<&str, Person> {
    let (remaining, questionstr) = take_while(is_not_whitespace)(input)?;
    let (remaining, _) = newline(remaining)?;
    let questions = questionstr.chars().collect();
    Ok((
        remaining, Person { questions }
    ))
}

fn is_not_whitespace(c: char) -> bool {
    !c.is_whitespace()
}

pub fn parse_group(input: &str) -> IResult<&str, Group> {
    let (last_person, persons) = many1(parse_person)(input)?;
    let mut persons = persons;
    if last_person.len() > 0 {
        let questions = last_person.chars().collect();
        persons.push( Person { questions });
    }
    Ok((
        last_person, Group { members: persons } 
    ))
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Group> {
    let mut groups = vec![];
    for sstr in input.split("\n\n") {
        if sstr.contains('\n') {
            let (_, group) = parse_group(sstr).unwrap();
            groups.push(group);
        } else {
            groups.push( Group { members: vec![ Person { questions: sstr.chars().collect() } ] } );

        }

    }
    groups
}

#[aoc(day6, part1)]
pub fn p1(input: &[Group]) -> u32 {
    input.iter().map(|x| x.total_questions().len() as u32).sum()
}

#[aoc(day6, part2)]
pub fn p2(input: &[Group]) -> u32 {
    input.iter().map(|x| x.questions_answered_by_all() as u32).sum()
}
