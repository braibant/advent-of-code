use std::collections::{HashMap, HashSet, VecDeque};

pub trait Neighbours<N, E> {
    fn neighbours<K>(&self, n: &K) -> Vec<(N, E)>
    where
        N: std::borrow::Borrow<K>,
        K: ToOwned<Owned = N> + Clone;
}

pub fn bfs<T, N, E, K>(state: &T, src: &K, tgt: &K) -> Option<Vec<E>>
where
    N: Eq + std::hash::Hash + Clone + std::fmt::Debug,
    E: Eq + std::hash::Hash + Clone + std::fmt::Debug,
    N: std::borrow::Borrow<K>,
    K: std::hash::Hash + Eq + Clone + ToOwned<Owned = N>,
    T: Neighbours<N, E>,
{
    let mut visited = HashSet::new();
    let mut todo: VecDeque<N> = VecDeque::new();
    let mut prev: HashMap<N, (N, E)> = HashMap::new();
    todo.push_back(src.to_owned());

    while let Some(node) = todo.pop_front() {
        if node.borrow() == tgt {
            // Path reconstruction
            let mut ptr = tgt;
            let mut acc: Vec<E> = vec![];
            while ptr != src {
                let (p, e) = prev.get(&ptr).unwrap();
                acc.push(e.clone());
                ptr = p.borrow();
            }
            acc.reverse();
            return Some(acc);
        } else if visited.contains(&node) {
        } else {
            visited.insert(node.clone());
            for (next, edge) in state.neighbours(node.borrow()).into_iter() {
                if prev.contains_key(next.borrow()) {
                } else {
                    prev.insert(next.clone(), (node.to_owned(), edge));
                    todo.push_back(next.clone())
                }
            }
        }
    }
    None
}
