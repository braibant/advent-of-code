use std::fmt::Debug;
use std::fs::File;
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

// Find the index that contains a given label
fn find<T>(permutation: &Permutation<T>, label: T) -> Option<usize>
where
    T: Eq,
{
    // find the position of the cell with the given label
    permutation
        .iter()
        .enumerate()
        .find(|(idx, (_next, l))| *l == label)
        .map(|(idx, _)| idx)
}

fn step(v: &mut Permutation<usize>, current: usize) {
    let len = v.len();

    // before:
    // current -> a, a -> b, b -> c, c -> next -> ...
    // destination -> x

    // after:
    // current -> next -> ...
    // destination -> a, a -> b, b -> c, c -> x

    let (a, lcurrent) = v[current];
    let (b, la) = v[a];
    let (c, lb) = v[b];
    let (next, lc) = v[c];

    // println!("pick up: {}, {}, {}", la, lb, lc);

    // select the label of the destination
    let mut ldest = previous(lcurrent, len);
    while ldest == la || ldest == lb || ldest == lc {
        ldest = previous(ldest, len);
    }

    // find the position of the destination
    let destination = find(v, ldest).unwrap();

    // println!("destination: {}", ldest);

    let (x, ldest) = v[destination];

    v[current] = (next, lcurrent);
    v[destination] = (a, ldest);
    v[c] = (x, lc);
}

fn create<T>(v: &Vec<T>) -> Permutation<T>
where
    T: Clone,
{
    let len = v.len();
    v.iter()
        .cloned()
        .enumerate()
        .map(|(idx, label)| ((idx + 1).rem_euclid(len), label))
        .collect()
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

fn labels<T>(permutation: &Permutation<T>, x: T, n: usize) -> Vec<T>
where
    T: Eq,
    T: Copy,
{
    let mut ptr = find(permutation, x).unwrap();

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

fn part1(permutation: &Permutation<usize>) {
    let mut permutation = permutation.clone();

    let mut current = 0;
    for i in 0..100 {
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

fn part2(permutation: &Permutation<usize>) {
    let mut permutation = permutation.clone();

    let mut current = 0;
    for i in 0..10_000_000 {
        if i % 100_000 == 0 {
            println!("{}", i)
        }
        step(&mut permutation, current);
        let (next, _lcurrent) = permutation[current];
        current = next;
    }

    let labels: Vec<usize> = labels(&permutation, 1, 2);
    println!("{:?}", labels)
}

pub fn run(_filename: String) {
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
