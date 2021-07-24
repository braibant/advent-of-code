use nom::IResult;
use std::collections::HashMap;

// We can observe that there is exactly one reaction that produces each
// component, so we don't need to "compare" chemical pathways in any way. The
// RHS of reactions has stoechiometric coefficient C greater than 1 in some
// cases. This means that we can only produce this compound in multiple of C.
// Let's tabulate the cost of producing C units of the compound, expressed in
// units of fuel (N). Unfortunately, just summing the cost up only gives us an
// upper bound on the fuel cost, since there might be some leftover reactants
// from one reaction that could be reused in the next one. It might be the case
// that this would provide an optimal strategy for the problem input at hand,
// but that's hardly satisfing.

// Let's pose the problem as a tuple, ORE, RESIDUE, NEED, with the semantics
// that ORE + NEED = 1 FUEL + RESIDUE. We process need one element at a time,
// applying the unique reaction that produces this element in reverse, modifying
// NEED (with the reactants we need to apply this reaction) and RESIDUE (with
// the left over reactants).

#[derive(Debug, Clone)]
struct Reaction {
    lhs: Vec<(i64, String)>,
    rhs: (i64, String),
}

fn ident(input: &str) -> IResult<&str, String> {
    nom::combinator::map(nom::character::complete::alpha1, |c: &str| c.to_string())(input)
}

fn num(input: &str) -> IResult<&str, i64> {
    nom::combinator::map_res(nom::character::complete::digit1, |s: &str| s.parse::<i64>())(input)
}

fn parse_line(line: &str) -> IResult<&str, Reaction> {
    let lhs = nom::multi::separated_list0(
        nom::bytes::complete::tag(", "),
        nom::sequence::separated_pair(num, nom::character::complete::multispace1, ident),
    );
    let rhs = nom::sequence::separated_pair(num, nom::character::complete::multispace1, ident);
    let reaction = nom::sequence::separated_pair(lhs, nom::bytes::complete::tag(" => "), rhs);
    nom::combinator::map(reaction, |(lhs, rhs)| Reaction { lhs, rhs })(line)
}

fn parse(s: &str) -> IResult<&str, Vec<Reaction>> {
    nom::multi::separated_list0(nom::character::complete::line_ending, parse_line)(s)
}

fn ore_cost(index: &HashMap<String, Reaction>, fuel: i64) -> i64 {
    let mut ore = 0;
    let mut need: HashMap<String, i64> = HashMap::new();
    let mut residue: HashMap<String, i64> = HashMap::new();
    need.insert("FUEL".to_owned(), fuel);
    // Invariant: domain(need) \inter domain(residue) == \emptyset
    while !need.is_empty() {
        // Pick one product we need.
        let product: String = need.keys().next().unwrap().to_string();
        // We need `n` molecules of `product`.
        let n: i64 = need.remove(&product).unwrap();
        let reaction = index.get(&product).unwrap();
        // We produce `product` by packs of `c`
        let c = reaction.rhs.0;
        // we need to do ceil(n/c) iterations of the reaction to cover our needs.
        let scale = (n + c - 1) / c;
        for (coef, reactant) in reaction.lhs.iter() {
            if reactant == "ORE" {
                ore += coef * scale;
            } else {
                let n = coef * scale - residue.remove(reactant).unwrap_or(0)
                    + need.remove(reactant).unwrap_or(0);
                if n > 0 {
                    need.insert(reactant.clone(), n);
                } else if n < 0 {
                    // We have too much left.
                    residue.insert(reactant.clone(), -n);
                }
            }
        }
        let rest = scale * c - n;
        if rest > 0 {
            if residue.insert(product, rest).is_some() {
                panic!("Invariant broken")
            }
        }
    }
    // need is empty
    ore
}

fn index(reactions: &[Reaction]) -> HashMap<String, Reaction> {
    let mut the_index: HashMap<String, Reaction> = HashMap::new();
    for r in reactions.iter() {
        the_index.insert(r.rhs.1.clone(), r.clone());
    }
    the_index
}

fn fuel_produced(index: &HashMap<String, Reaction>, ore: i64) -> i64 {
    let ore_per_unit = ore_cost(index, 1);
    // The number of fuel units we can build is greater than ore / ore_per_unit,
    // since we can use the residues. Let's compute the number of units we can
    // build by doing a binary search.
    let mut l = 0;
    let mut r = 2 * ore / ore_per_unit - 1;
    let mut ocl = ore_cost(&index, l);
    let mut ocr = ore_cost(&index, r);
    loop {
        println!("{} {} -- {} {} ({})", l, ocl, r, ocr, (r - l));
        assert!(ocl < ore);
        assert!(ore <= ocr);
        if l == r - 1 {
            return l;
        };
        let m = l + (r - l) / 2;
        let ocm = ore_cost(&index, m);
        if ocm < ore {
            l = m;
            ocl = ocm;
        } else if ore < ocm {
            r = m;
            ocr = ocm;
        } else if ore == m {
            return m;
        }
    }
}

pub fn run(filename: &str) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let (_, reactions) = parse(&contents).unwrap();
    let index = index(&reactions);
    println!("{}", ore_cost(&index, 1));
    println!("{}", fuel_produced(&index, 1_000_000_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_part_1() {
        let s = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
        let (_, reactions) = parse(&s).unwrap();
        let index = index(&reactions);
        assert_eq!(ore_cost(&index, 1), 2210736);
        assert_eq!(fuel_produced(&index, 1_000_000_000_000), 460664);
    }
}
