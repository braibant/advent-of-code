use std::collections::HashMap;

type T = HashMap<String, Vec<String>>;

fn print(t: &T) {
    for (center_of_mass, objects) in t.iter() {
        println!("{} -> {}", center_of_mass, format!("{:?}", objects))
    }
}

// `w - 1` is the number of objects that center_of_mass orbits.
fn indirect(orbits: &T, center_of_mass: &str, w: usize) -> usize {
    match orbits.get(center_of_mass) {
        None => 0,
        Some(objects) => {
            let mut acc = w * objects.len();
            for object in objects.iter() {
                acc += indirect(orbits, object, w + 1)
            }
            acc
        }
    }
}

// Compute the distance between two nodes, assuming that they have a common ancestor.
fn distance(parent: &HashMap<String, String>, left: &str, right: &str) -> usize {
    let mut visited_left = HashMap::new();
    let mut visited_right = HashMap::new();
    let mut distance_left = 0;
    let mut distance_right = 0;
    visited_left.insert(left.to_string(), 0);
    visited_right.insert(right.to_string(), 0);

    let mut left = left;
    let mut right = right;
    if left == right {
        return 0;
    };

    // Loop invariant:
    // - left \notin visited_right;
    // - right \notin visited_left;
    // - left \in visited_left;
    // - right \in visited_right;
    loop {
        if let Some(next) = parent.get(left) {
            left = next;
            distance_left += 1;
            visited_left.insert(left.to_string(), distance_left);
        }
        if let Some(distance) = visited_right.get(left) {
            return distance_left + distance;
        };
        if let Some(next) = parent.get(right) {
            right = next;
            distance_right += 1;
            visited_right.insert(right.to_string(), distance_right);
        };
        if let Some(distance) = visited_left.get(right) {
            return distance_right + distance;
        };
    }
}

fn parse(s: &str) -> (T, HashMap<String, String>) {
    let lines: Vec<_> = s.split("\n").collect();
    let raw_orbits: Vec<(String, String)> = lines
        .iter()
        .map(|s| {
            let o: Vec<_> = s.split(")").collect();
            assert_eq!(o.len(), 2);
            (o[0].to_string(), o[1].to_string())
        })
        .collect();

    let mut orbits: T = HashMap::new();
    let mut parents: HashMap<String, String> = HashMap::new();
    for (center_of_mass, object) in raw_orbits.into_iter() {
        parents.insert(object.clone(), center_of_mass.clone());
        let mut objects = orbits.entry(center_of_mass).or_insert(vec![]);
        objects.push(object)
    }
    (orbits, parents)
}

pub fn run(filename: String) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let (orbits, parents) = parse(&contents);
    println!("{}", indirect(&orbits, "COM", 1));
    println!("{}", distance(&parents, "YOU", "SAN") - 2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let s = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        let (orbits, _) = parse(s);
        assert_eq!(42, indirect(&orbits, "COM", 1))
    }

    fn test_example_part2() {
        let s = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        let (_, parents) = parse(s);
        assert_eq!(distance(&parents, "SAN", "YOU"), 6)
    }
}
