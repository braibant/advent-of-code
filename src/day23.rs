use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

// permutation
type Permutation<T> = Vec<(usize, T)>;

fn previous(i: usize, n: usize) -> usize {
    let p = (i - 1).rem_euclid(n);
    if p == 0 {
        n
    } else {
        p
    }
}

fn step(permutation: &mut Permutation<usize>, index: &HashMap<usize, usize>, current: usize) {
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
    let destination = *index.get(&ldest).unwrap();

    // println!("destination: {}", ldest);

    let (x, ldest) = permutation[destination];

    permutation[current] = (next, lcurrent);
    permutation[destination] = (a, ldest);
    permutation[c] = (x, lc);
}

fn create<T>(v: &Vec<T>) -> (Permutation<T>, HashMap<T, usize>)
where
    T: Copy,
    T: Eq,
    T: Hash,
{
    let mut index = HashMap::new();
    let mut support = vec![];
    let len = v.len();
    for (idx, &label) in v.iter().enumerate() {
        let next = (idx + 1).rem_euclid(len);
        support.push((next, label));
        index.insert(label, idx);
    }
    (support, index)
}

fn print<T>(permutation: &Permutation<T>, current: usize)
where
    T: Debug,
    T: Copy,
{
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

fn labels<T>(permutation: &Permutation<T>, index: &HashMap<T, usize>, x: T, n: usize) -> Vec<T>
where
    T: Eq,
    T: Copy,
    T: Hash,
{
    let mut ptr: usize = *index.get(&x).unwrap();

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

fn part1(permutation: &Permutation<usize>, index: &HashMap<usize, usize>) {
    let mut permutation = permutation.clone();

    let mut current = 0;
    for i in 0..100 {
        step(&mut permutation, index, current);
        let (next, _lcurrent) = permutation[current];
        current = next;
    }

    let labels: Vec<_> = labels(&permutation, index, 1, permutation.len() - 1)
        .iter()
        .map(|n| n.to_string())
        .collect();
    println!("{}", labels.join(""))
}

fn part2(permutation: &Permutation<usize>, index: &HashMap<usize, usize>) {
    let mut permutation = permutation.clone();

    let mut current = 0;
    for i in 0..10_000_000 {
        if i % 100_000 == 0 {
            println!("{}", i)
        }
        step(&mut permutation, index, current);
        let (next, _lcurrent) = permutation[current];
        current = next;
    }

    let labels: Vec<usize> = labels(&permutation, index, 1, 2);
    println!("{:?}", labels)
}

pub fn run(_filename: String) {
    let example_seed = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
    let (permutation, index) = create(&example_seed);
    part1(&permutation, &index);

    let my_seed = vec![2, 5, 3, 1, 4, 9, 8, 6, 7];
    let (permutation, index) = create(&my_seed);
    part1(&permutation, &index);

    let mut input = my_seed.clone();
    for i in my_seed.len()..1000000 {
        input.push(i + 1);
    }
    let (permutation, index) = create(&input);

    part2(&permutation, &index)
}
