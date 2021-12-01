use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Each COLOR is made of a qualified (e.g. dull) and a base color (e.g. cyan).
// Each entry has the form `COLOR contains no other` or `COLOR 'contains' sep_list([int COLOR], ',')`
fn parse(s: &str) -> (String, Vec<(i32, String)>) {
    let v: Vec<&str> = s.split(" contain ").collect();
    if v.len() < 2 {
        panic!("Ill-formed input: {}", s)
    };
    let src = v[0];
    let contains = v[1];
    let mut content = Vec::new();
    if contains == "no other" {
    } else {
        for bags in contains.split(", ") {
            // The last line in the data file is facetious and contains trailing whitespace.
            let bags = bags.trim();
            let bags: Vec<&str> = bags.split(" ").collect();
            if !(bags.len() == 3) {
                println!("{} {:?}", s, bags);
                panic!("Invalid")
            };
            let count: i32 = bags[0].parse().unwrap();
            let color = String::from(format!("{} {}", bags[1], bags[2]));
            content.push((count, color));
        }
    };
    return (src.to_string(), content);
}

// Returns [true] iff `tgt` is reachable from `src`.
fn find(entries: &HashMap<String, Vec<(i32, String)>>, tgt: &str, src: &str) -> bool {
    let content = entries.get(src).unwrap();
    for (_i, bag) in content.iter() {
        if bag == tgt {
            return true;
        } else if find(entries, tgt, bag) {
            return true;
        } else {
        }
    }
    return false;
}

// Returns the number of bags required inside a bag of color [src].
fn contains(entries: &HashMap<String, Vec<(i32, String)>>, src: &str) -> i32 {
    let content = entries.get(src).unwrap();
    let mut n = 0;
    for (i, bag) in content.iter() {
        n += i * contains(entries, bag) + i
    }
    return n;
}

// Input contain lines of the form `name -> (int * name) list` (the actual input is a bit obfuscated).
pub fn run(filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut entries = HashMap::new();
    for line in reader.lines() {
        let line: String = line.unwrap();
        let line = line
            .replace("bags", "")
            .replace("bag", "")
            .replace("  ", " ")
            .replace(" ,", ",")
            .replace(" .", "");
        let (bag, content) = parse(&line);
        entries.insert(bag, content);
    }

    let mut count = 0;
    for i in entries.keys() {
        if find(&entries, "shiny gold", i) {
            count += 1
        }
    }
    println!("{}", count);

    println!("{}", contains(&entries, "shiny gold"))
}
