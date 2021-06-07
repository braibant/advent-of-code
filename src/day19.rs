use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res, value},
    multi::many0,
    IResult,
};

#[derive(Clone, Debug)]
enum T {
    Rule { id: u64, sub_rules: Vec<Vec<u64>> },
    Atom { id: u64, content: String },
}

// Numeric identifier
fn id(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse::<u64>())(input)
}

// A space separated list of rules that must match.
fn sub_rule(input: &str) -> IResult<&str, Vec<u64>> {
    nom::multi::separated_list1(nom::character::complete::space1, id)(input)
}

// The format of rule is `ID: SUB_RULE | ... | SUB_RULE`
fn rule(input: &str) -> IResult<&str, T> {
    let mut sub_rules = nom::multi::separated_list1(tag(" | "), sub_rule);

    let mut rule = nom::sequence::tuple((id, tag(": "), sub_rules));

    map(rule, |(id, _, sub_rules)| T::Rule { id, sub_rules })(input)
}

// The format of an atom is `ID: "CONTENT"`
fn atom(input: &str) -> IResult<&str, T> {
    let mut string = nom::sequence::delimited(
        char('"'),
        nom::bytes::complete::take_while(|c| c != '"'),
        char('"'),
    );
    let mut atom = nom::sequence::tuple((id, tag(": "), string));
    map(atom, |(id, _, content)| T::Atom {
        id,
        content: content.to_string(),
    })(input)
}

fn parse(input: &str) -> IResult<&str, T> {
    alt((atom, rule))(input)
}

type Language = HashSet<String>;

fn concat(a: &HashSet<String>, b: &HashSet<String>) -> HashSet<String> {
    let mut acc = HashSet::new();
    for a in a.iter() {
        for b in b.iter() {
            let s: String = format!("{}{}", a, b);
            acc.insert(s);
        }
    }

    return acc;
}

fn sub_rule_productions<F>(sub_rule: &[u64], recurse: &mut F) -> HashSet<String>
where
    F: FnMut(u64) -> HashSet<String>,
{
    let mut acc: HashSet<String> = HashSet::new();
    acc.insert("".to_string());
    for &i in sub_rule.iter() {
        let productions = recurse(i);
        acc = concat(&acc, &productions);
    }
    return acc;
}

fn rule_productions<F>(sub_rules: &Vec<Vec<u64>>, recurse: &mut F) -> HashSet<String>
where
    F: FnMut(u64) -> HashSet<String>,
{
    let mut acc: HashSet<String> = HashSet::new();
    for sub_rule in sub_rules.iter() {
        let productions = sub_rule_productions(&sub_rule, recurse);
        for x in productions.into_iter() {
            acc.insert(x);
        }
    }
    return acc;
}

fn productions_memo<'a>(
    rules: &HashMap<u64, T>,
    cache: &mut HashMap<u64, HashSet<String>>,
    id: u64,
) -> HashSet<String> {
    match cache.get(&id) {
        Some(language) => language.clone(),
        None => match rules.get(&id).unwrap() {
            T::Atom { content, id: _ } => {
                let mut s = HashSet::new();
                s.insert(content.clone());
                cache.insert(id, s.clone());
                s
            }
            T::Rule { sub_rules, id: _ } => {
                let result: Language =
                    rule_productions(sub_rules, &mut |id| productions_memo(rules, cache, id));
                cache.insert(id, result.clone());
                result
            }
        },
    }
}

pub fn run(filename: String) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let chunks: Vec<_> = contents.split("\n\n").collect();
    assert_eq!(chunks.len(), 2);

    let mut grammar: HashMap<u64, T> = HashMap::new();
    for line in chunks[0].lines() {
        let (_, t) = parse(line).unwrap();
        let &id = match &t {
            T::Rule { id, sub_rules } => id,
            T::Atom { id, content } => id,
        };
        grammar.insert(id, t);
    }

    let mut cache = HashMap::new();
    for &id in grammar.keys() {
        let language = productions_memo(&grammar, &mut cache, id);
        println!("{}, {}", id, language.len());
    }
}
