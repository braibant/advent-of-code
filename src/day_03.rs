use crate::util::vector2::Vector2;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
struct Claim {
    id: u32,
    pos: Vector2<i32>,
    size: Vector2<i32>,
}

fn parse_claim(s: &str) -> Claim {
    let (id, posx, posy, sizex, sizey) =
        scan_fmt!(s, "#{d} @ {d},{d}: {d}x{d}", u32, i32, i32, i32, i32)
            .expect(&format!("Could not parse '{}'", s));
    Claim {
        id,
        pos: Vector2::new(posx, posy),
        size: Vector2::new(sizex, sizey),
    }
}

fn parse(s: &str) -> impl Iterator<Item = Claim> + '_ {
    s.split('\n')
        .filter(|&l| !l.is_empty())
        .map(|l| parse_claim(l))
}

fn iter_claims<F>(s: &str, mut f: F)
where
    F: FnMut(u32, Vector2<i32>),
{
    for claim in parse(s) {
        for dx in 0..claim.size.x {
            for dy in 0..claim.size.y {
                f(claim.id, Vector2::new(claim.pos.x + dx, claim.pos.y + dy))
            }
        }
    }
}

fn part1(s: &str) -> usize {
    // We want to compute the number of square inches of fabric that are subject
    // to more than one claim. This begs to represent the fabric that has been
    // subjected to a claim, and the fabric that has been subjected to more than
    // a claim, both which are sets of coordinates. We can then pick a dense
    // representation (e.g. a matrix), or a sparse representation (two sets, or
    // a map), or a representation that we can rasterize.
    let mut claims: HashMap<Vector2<i32>, i32> = HashMap::new();
    let f = |_id, v| {
        let entry = claims.entry(v).or_insert(0);
        *entry += 1
    };
    iter_claims(s, f);
    claims.values().filter(|&&c| c > 1).count()
}

fn part2(s: &str) {
    // We want to find the (unique) claim that does not overlap with other
    // claims.
    let mut claims: HashMap<Vector2<i32>, HashSet<u32>> = HashMap::new();
    let mut ids = HashSet::new();
    let f = |id, v| {
        ids.insert(id);
        let entry = claims.entry(v).or_insert(HashSet::new());
        entry.insert(id);
    };
    iter_claims(s, f);
    for tile in claims.values() {
        if tile.len() > 1 {
            for t in tile.iter() {
                ids.remove(t);
            }
        }
    }
    if ids.len() != 1 {
        panic!("Invalid instance {}", ids.len())
    };
    for id in ids.iter() {
        println!("{}", id)
    }
}

pub fn run(filename: &str) {
    let content =
        std::fs::read_to_string(filename).expect(&format!("Could not read file '{}'", filename));
    println!("{}", part1(&content));
    part2(&content);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_claim() {
        let s = "#123 @ 3,2: 5x4";
        assert_eq!(
            parse_claim(s),
            Claim {
                id: 123,
                pos: Vector2 { x: 3, y: 2 },
                size: Vector2 { x: 5, y: 4 }
            }
        );
    }
}
