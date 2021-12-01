use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::ops::{Index, IndexMut};

type T = usize;

#[derive(Clone)]
struct Permutation {
    support: Vec<(usize, T)>,
    index: HashMap<T, usize>,
}

impl Permutation {
    fn len(&self) -> usize {
        self.support.len()
    }
}

impl Index<usize> for Permutation {
    type Output = (usize, T);

    fn index(&self, i: usize) -> &Self::Output {
        &self.support[i]
    }
}

impl IndexMut<usize> for Permutation {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.support[index]
    }
}

fn previous(i: usize, n: usize) -> usize {
    let p = (i - 1).rem_euclid(n);
    if p == 0 {
        n
    } else {
        p
    }
}

fn step(permutation: &mut Permutation, current: usize) {
    let len = permutation.len();

    // before:
    // current -> a, a -> b, b -> c, c -> next -> ...
    // destination -> x

    // after:
    // current -> next -> ...
    // destination -> a, a -> b, b -> c, c -> x

    let (a, lcurrent) = permutation[current];
    let (b, la) = permutation[a];
    let (c, lb) = permutation[b];
    let (next, lc) = permutation[c];

    // println!("pick up: {}, {}, {}", la, lb, lc);

    // select the label of the destination
    let mut ldest = previous(lcurrent, len);
    while ldest == la || ldest == lb || ldest == lc {
        ldest = previous(ldest, len);
    }

    // find the position of the destination
    let destination = *permutation.index.get(&ldest).unwrap();

    // println!("destination: {}", ldest);

    let (x, ldest) = permutation[destination];

    permutation[current] = (next, lcurrent);
    permutation[destination] = (a, ldest);
    permutation[c] = (x, lc);
}

fn create(v: &Vec<T>) -> Permutation {
    let mut index = HashMap::new();
    let mut support = vec![];
    let len = v.len();
    for (idx, &label) in v.iter().enumerate() {
        let next = (idx + 1).rem_euclid(len);
        support.push((next, label));
        index.insert(label, idx);
    }
    Permutation { support, index }
}

#[allow(dead_code)]
fn print(permutation: &Permutation, current: usize) {
    let mut ptr = 0;
    for _i in 0..permutation.len() {
        let (next, label) = permutation[ptr];
        if ptr == current {
            print!("({:?}) ", label);
        } else {
            print!("{:?} ", label);
        }
        ptr = next;
    }
    println!("")
}

fn labels(permutation: &Permutation, x: T, n: usize) -> Vec<T> {
    let mut ptr: usize = *permutation.index.get(&x).unwrap();

    // skip 1
    ptr = permutation[ptr].0;

    let mut acc = vec![];
    for _i in 0..n {
        let (next, label) = permutation[ptr];
        acc.push(label);
        ptr = next
    }

    return acc;
}

fn part1(permutation: &Permutation) {
    let mut permutation = permutation.clone();

    let mut current = 0;
    for _i in 0..100 {
        step(&mut permutation, current);
        let (next, _lcurrent) = permutation[current];
        current = next;
    }

    let labels: Vec<_> = labels(&permutation, 1, permutation.len() - 1)
        .iter()
        .map(|n| n.to_string())
        .collect();
    println!("{}", labels.join(""))
}

fn part2(permutation: &Permutation) {
    let mut permutation = permutation.clone();

    let mut current = 0;
    for _i in 0..10_000_000 {
        step(&mut permutation, current);
        let (next, _lcurrent) = permutation[current];
        current = next;
    }

    let labels: Vec<usize> = labels(&permutation, 1, 2);
    println!("{:?}", labels);
    println!("{}", labels.iter().product::<usize>());
}

pub fn run() {
    let example_seed = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
    let permutation = create(&example_seed);
    part1(&permutation);

    let my_seed = vec![2, 5, 3, 1, 4, 9, 8, 6, 7];
    let permutation = create(&my_seed);
    part1(&permutation);

    let mut input = my_seed.clone();
    for i in my_seed.len()..1000000 {
        input.push(i + 1);
    }
    let permutation = create(&input);

    part2(&permutation)
}
