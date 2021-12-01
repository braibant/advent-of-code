struct T {
    tree: Vec<usize>,
}

fn parse(s: &str) -> T {
    let tree = s
        .split(' ')
        .map(|i| match i.parse() {
            Ok(i) => i,
            Err(e) => panic!("Could not parse {:?} ({:?})", i, e),
        })
        .collect();
    T { tree }
}

// Returns the sum of the metadata in that tree, and the position of the next
// node at this level.
fn visit(t: &T, node: usize) -> (usize, usize) {
    let children = t.tree[node];
    let metadata = t.tree[node + 1];
    let mut sum = 0;
    let mut current = node + 2;
    for _i in 0..children {
        let (s, p) = visit(t, current);
        sum += s;
        current = p;
    }
    for _i in 0..metadata {
        sum += t.tree[current];
        current += 1;
    }
    (sum, current)
}

fn part1(t: &T) -> usize {
    let (s, _) = visit(t, 0);
    s
}

// We want to compute the value of each node, and the position of the next node
// if any.
fn value(t: &T, node: usize) -> (usize, usize) {
    let children = t.tree[node];
    let metadata = t.tree[node + 1];
    if children == 0 {
        let mut value = 0;
        let mut current = node + 2;
        for _i in 0..metadata {
            value += t.tree[current];
            current += 1;
        }
        (value, current)
    } else {
        let mut current = node + 2;
        let mut values = Vec::new();
        for _i in 0..children {
            let (value, next) = value(t, current);
            current = next;
            values.push(value);
        }
        let mut value = 0;
        for _i in 0..metadata {
            let m = t.tree[current];
            if 0 < m && m <= values.len() {
                value += values[m - 1]
            };
            current += 1;
        }
        (value, current)
    }
}

fn part2(t: &T) -> usize {
    let (v, _) = value(t, 0);
    v
}

pub fn run(filename: &str) {
    let mut contents = std::fs::read_to_string(&filename).unwrap();
    if contents.ends_with('\n') {
        contents.pop();
    };
    let t = parse(&contents);
    println!("{}", part1(&t));
    println!("{}", part2(&t));
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
    #[test]
    fn test_part_2() {
        let t = parse(EXAMPLE);
        assert_eq!(value(&t, 0), (66, 16));
        assert_eq!(value(&t, 9), (99, 12));
    }
}
