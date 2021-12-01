#![allow(unused_imports)]
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res},
    IResult,
};

#[derive(Clone, Debug)]
pub enum T {
    Rule { id: u64, sub_rules: Vec<Vec<u64>> },
    Atom { id: u64, content: char },
}

mod parsing {
    use super::*;
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
        let sub_rules = nom::multi::separated_list1(tag(" | "), sub_rule);

        let rule = nom::sequence::tuple((id, tag(": "), sub_rules));

        map(rule, |(id, _, sub_rules)| T::Rule { id, sub_rules })(input)
    }

    // The format of an atom is `ID: "CONTENT"`
    fn atom(input: &str) -> IResult<&str, T> {
        let string = nom::sequence::delimited(
            char('"'),
            nom::bytes::complete::take_while(|c| c != '"'),
            char('"'),
        );
        let atom = nom::sequence::tuple((id, tag(": "), string));
        map(atom, |(id, _, content)| {
            let content: Vec<char> = content.chars().collect();
            assert_eq!(content.len(), 1);
            T::Atom {
                id,
                content: content[0],
            }
        })(input)
    }

    pub fn parse(input: &str) -> IResult<&str, T> {
        alt((atom, rule))(input)
    }
}
mod part1 {
    use super::*;
    // For part1, the approach is the naive one: generate sets of strings from the language that is provided, then check for inclusion of the messages in this representation of the language. This approach is tractable, since the languages are finite. (This assumption fails in part2).

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

    #[allow(dead_code)]
    pub fn productions_memo<'a>(
        rules: &HashMap<u64, T>,
        cache: &mut HashMap<u64, HashSet<String>>,
        id: u64,
    ) -> HashSet<String> {
        match cache.get(&id) {
            Some(language) => language.clone(),
            None => match rules.get(&id).unwrap() {
                T::Atom { content, id: _ } => {
                    let mut s = HashSet::new();
                    s.insert(format!("{}", content));
                    cache.insert(id, s.clone());
                    s
                }
                T::Rule { sub_rules, id: _ } => {
                    let result: HashSet<String> =
                        rule_productions(sub_rules, &mut |id| productions_memo(rules, cache, id));
                    cache.insert(id, result.clone());
                    result
                }
            },
        }
    }
}

mod part2 {
    use super::*;

    // For part 2, we are going to completely change the approach. The goal is to find the messages that are accepted by rule 0 (an entry point). We are going to consume the messages one letter at a time, and compute the language (ie, set of words, ie, sets of vectors of rule or letters) that we need to recognize at this point.

    pub fn derive(grammar: &HashMap<u64, T>, es: &HashSet<Vec<u64>>, c: char) -> HashSet<Vec<u64>> {
        let mut acc: HashSet<Vec<u64>> = HashSet::new();
        for e in es.iter() {
            if e.len() == 0 {
                // We cannot recognize the remaining letter, drop the rest of the language.
            } else {
                match grammar.get(&e[0]).unwrap() {
                    T::Atom { id: _, content } => {
                        if *content == c {
                            acc.insert(e[1..].to_vec());
                        } else {
                            // Mismatch, drop the remaining expression.
                        }
                    }
                    T::Rule { id: _, sub_rules } => {
                        let sub_rules: HashSet<Vec<u64>> =
                            sub_rules.iter().map(|c| c.clone()).collect::<HashSet<_>>();
                        let es2 = derive(grammar, &sub_rules, c);
                        for mut e2 in es2.into_iter() {
                            e2.extend_from_slice(&e[1..]);
                            // let mut v = Vec::new();
                            // v.extend_from_slice(&e2[0..]);
                            // v.extend_from_slice(&e[1..]);
                            acc.insert(e2);
                        }
                    }
                }
            }
        }
        return acc;
    }

    pub fn valid(grammar: &HashMap<u64, T>, message: &str) -> bool {
        let mut es = HashSet::new();
        es.insert(vec![0]);
        for c in message.chars() {
            es = derive(grammar, &es, c)
        }
        es.contains(&vec![])
    }
}

use parsing::parse;

pub fn run(filename: String) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let chunks: Vec<_> = contents.split("\n\n").collect();
    assert_eq!(chunks.len(), 2);

    let mut grammar: HashMap<u64, T> = HashMap::new();
    for line in chunks[0].lines() {
        let (_, t) = parse(line).unwrap();
        let &id = match &t {
            T::Rule { id, sub_rules: _ } => id,
            T::Atom { id, content: _ } => id,
        };
        grammar.insert(id, t);
    }

    // Part 1, v1. This precomputes the set of words accepted by each rule, and then test for membership. It's slow to the tune of ~2s in release mode.
    // let start = Instant::now();
    // let mut cache = HashMap::new();
    // for &id in grammar.keys() {
    //     let language = part1::productions_memo(&grammar, &mut cache, id);
    // }
    // println!("part 1 precomputation {:?}", start.elapsed());

    // let start = Instant::now();
    // let mut part1 = 0;
    // let language = cache.get(&0).unwrap();
    // for message in chunks[1].lines() {
    //     if language.contains(message) {
    //         part1 += 1
    //     }
    // }
    // println!("part 1 check {:?}", start.elapsed());
    // println!("{}", part1);

    // Part 1, v2
    let start = Instant::now();
    let mut part1_v2 = 0;
    for message in chunks[1].lines() {
        if part2::valid(&grammar, message) {
            part1_v2 += 1
        }
    }
    println!("{}", part1_v2);

    println!("part 1 check {:?}", start.elapsed());

    // For part 2, we override a couple of rules, with the effect of making the language infinite.
    let (_, rule8) = parse("8: 42 | 42 8").unwrap();
    let (_, rule11) = parse("11: 42 31 | 42 11 31").unwrap();
    grammar.insert(8, rule8);
    grammar.insert(11, rule11);

    let start = Instant::now();
    let mut part2 = 0;
    for message in chunks[1].lines() {
        if part2::valid(&grammar, message) {
            part2 += 1
        }
    }
    println!("{}", part2);
    println!("part 2 check {:?}", start.elapsed());
}
