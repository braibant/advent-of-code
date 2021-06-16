use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

// permutation
type Permutation<T> = Vec<(usize, T)>;

fn step(v: &mut Permutation<u32>, current: usize) {
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

    // select the label of the destination
    let mut ldest = (lcurrent + len - 1).rem_euclid(len);
    loop {
        if ldest == la || ldest == lb || ldest == lc {
            ldest = (ldest + len - 1).rem_euclid(len);
        } else {
            continue;
        }
    }

    // find the position of the destination
    let (destination, _) = v
        .iter()
        .find(|&&(_next, label)| label == ldest)
        .unwrap()
        .clone();

    let (x, ldest) = v[destination];

    v[current] = (next, lcurrent);
    v[destination] = (a, ldest);
    v[c] = (x, lc);
}

fn create<T>(v: &Vec<T>) -> Permutation<T>
where
    T: Copy,
{
    v.iter()
        .enumerate()
        .map(|(idx, label)| (idx + 1, label).clone())
        .collect()
}

fn print<T>(permutation: &Permutation<T>)
where
    T: Debug,
{
    let mut ptr = 0;
    for _i in 0..permutation.len() {
        let (next, label) = premutation[ptr];
        let ptr = next;
        print!("{:?} ", label)
    }
    println!("")
}

pub fn run() {
    let seed = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
    let permutation = create(&seed);

    let mut current = 0;
    for _i in 0..10 {
        print(permutation);
        step(permutation)
    }
}
