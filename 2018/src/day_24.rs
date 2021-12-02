use std::{
    cell::Ref,
    collections::{HashMap, HashSet},
};

type Type = String;

#[derive(Debug, Clone, Copy)]
enum Side {
    ImmuneSystem,
    Infection,
}

#[derive(Debug, Clone)]
struct Group {
    side: Side,
    id: usize,
    count: i32,
    hit_points: i32,
    attack_type: Type,
    attack_damage: i32,
    initiative: i32,
    weakness: HashSet<Type>,
    immunity: HashSet<Type>,
}

impl Group {
    fn effective_power(&self) -> i32 {
        self.count * self.attack_damage
    }

    fn damage_from(&self, attacker: &Group) -> i32 {
        if self.immunity.contains(&attacker.attack_type) {
            0
        } else if self.weakness.contains(&attacker.attack_type) {
            2 * attacker.effective_power()
        } else {
            attacker.effective_power()
        }
    }

    fn defend(&mut self, attacker: &Group) -> i32 {
        let damage = self.damage_from(attacker);
        let killed = std::cmp::min(self.count, damage / self.hit_points);
        self.count -= killed;
        killed
    }
}

#[derive(Debug, Clone)]
struct T {
    immune_system: Vec<Group>,
    infection: Vec<Group>,
}

impl T {
    fn boost(&mut self, boost: i32) {
        for g in self.immune_system.iter_mut() {
            g.attack_damage += boost
        }
    }
}

fn pick_target(this: &Group, other: &[Group], used: &HashSet<usize>) -> Option<usize> {
    other
        .iter()
        .enumerate()
        .filter(|(i, g)| !used.contains(i) && g.damage_from(this) != 0)
        .max_by_key(|(_i, other)| {
            (
                other.damage_from(this),
                other.effective_power(),
                other.initiative,
            )
        })
        .map(|(i, _)| i)
}

fn pick_targets(this: &[Group], other: &[Group]) -> Vec<Option<usize>> {
    let mut used = HashSet::new();
    this.iter()
        .map(|this| {
            let t = pick_target(this, other, &used);
            if let Some(t) = t {
                // println!(
                //     "{:?} group {} would deal defending group {} {} damage",
                //     this.side,
                //     this.id,
                //     other[t].id,
                //     other[t].damage_from(this)
                // );
                used.insert(t);
            };
            t
        })
        .collect()
}

impl T {
    fn fight(&mut self) -> i32 {
        // Sort by decreasing effective power, breaking ties by decreasing initiative

        self.immune_system
            .sort_by_key(|g| (-g.effective_power(), -g.initiative));
        self.infection
            .sort_by_key(|g| (-g.effective_power(), -g.initiative));

        // Pick targets for each side
        let immune_system_targets = pick_targets(&self.immune_system, &self.infection);
        let infection_targets = pick_targets(&self.infection, &self.immune_system);

        let mut order = vec![];
        for (i, g) in self.immune_system.iter().enumerate() {
            if let Some(target) = immune_system_targets[i] {
                order.push((Side::ImmuneSystem, i, g.initiative, target))
            }
        }
        for (i, g) in self.infection.iter().enumerate() {
            if let Some(target) = infection_targets[i] {
                order.push((Side::Infection, i, g.initiative, target))
            }
        }

        // Sort by decreasing initiative
        order.sort_by_key(|(_, _, initiative, _)| -*initiative);

        let mut total_killed = 0;

        for (side, index, _, target) in order.into_iter() {
            match side {
                Side::ImmuneSystem => {
                    let killed = self.infection[target].defend(&self.immune_system[index]);
                    // println!(
                    //     "Immune System group {} attacks defending group {}, killing {}",
                    //     self.immune_system[index].id, self.infection[target].id, killed
                    // );
                    total_killed += killed;
                }
                Side::Infection => {
                    let killed = self.immune_system[target].defend(&self.infection[index]);
                    // println!(
                    //     "Infection group {} attacks defending group {}, killing {}",
                    //     self.infection[index].id, self.immune_system[target].id, killed
                    // );
                    total_killed += killed;
                }
            }
        }

        self.immune_system.retain(|g| g.count > 0);
        self.infection.retain(|g| g.count > 0);
        total_killed
    }

    fn remaining_army_size(&mut self) -> i32 {
        while self.infection.len() != 0 && self.immune_system.len() != 0 {
            self.fight();
            //     println!("");
        }

        let a: i32 = self.infection.iter().map(|g| g.count).sum();
        let b: i32 = self.immune_system.iter().map(|g| g.count).sum();

        return a + b;
    }
}

fn parse_special(s: &str) -> (HashSet<String>, HashSet<String>) {
    let mut immunity = HashSet::new();
    let mut weakness = HashSet::new();

    let words = s.replace(";", "").replace(",", "");
    let mut next = &mut immunity;
    for word in words.split(" ") {
        if word == "immune" {
            next = &mut immunity;
        } else if word == "weak" {
            next = &mut weakness;
        } else if word == "to" {
        } else {
            next.insert(word.to_string());
        }
    }
    (weakness, immunity)
}

fn parse_group(side: Side, id: usize, s: &str) -> Group {
    use scan_fmt::scan_fmt;
    let parts: Vec<_> = s.split(&['(', ')'][..]).collect();

    if parts.len() == 3 {
        let (count, hit_points) =
            scan_fmt!(parts[0], "{} units each with {} hit points ", i32, i32).unwrap();

        let (weakness, immunity) = parse_special(&parts[1]);
        let (attack_damage, attack_type, initiative) = scan_fmt!(
            parts[2],
            " with an attack that does {} {} damage at initiative {}",
            i32,
            String,
            i32
        )
        .unwrap();

        // let (count, hit_points, special, attack_damage, attack_type, initiative) =
        // scan_fmt!(s, "{} units each with {} hit points ({}) with an attack that does {} {} damage at initiative {}", i32, i32, String, i32, String, i32).unwrap();

        Group {
            side,
            id,
            count,
            hit_points,
            attack_damage,
            attack_type,
            initiative,
            immunity,
            weakness,
        }
    } else {
        let (count, hit_points, attack_damage, attack_type, initiative) =
        scan_fmt!(s, "{} units each with {} hit points with an attack that does {} {} damage at initiative {}", i32, i32,  i32, String, i32).unwrap();

        Group {
            side,
            id,
            count,
            hit_points,
            attack_damage,
            attack_type,
            initiative,
            immunity: HashSet::new(),
            weakness: HashSet::new(),
        }
    }
}

fn parse_part(side: Side, s: &str) -> Vec<Group> {
    s.lines()
        .enumerate()
        .skip(1)
        .map(|(i, g)| parse_group(side, i, g))
        .collect()
}

fn parse(s: &str) -> T {
    let parts: Vec<_> = s.split("\n\n").collect();
    let immune_system = parse_part(Side::ImmuneSystem, parts[0]);
    let infection = parse_part(Side::Infection, parts[1]);
    T {
        immune_system,
        infection,
    }
}

fn immune_system_wins(t: &T, boost: i32) -> Option<i32> {
    let mut t = t.clone();
    t.boost(boost);

    while t.infection.len() != 0 && t.immune_system.len() != 0 {
        if t.fight() == 0 {
            return None;
        }
    }
    if t.immune_system.len() > 0 {
        Some(t.immune_system.iter().map(|g| g.count).sum())
    } else {
        None
    }
}

fn part2(t: &T) -> i32 {
    let mut boost = 5000;
    while immune_system_wins(t, boost).is_none() {
        boost *= 2;
    }
    let mut l = 0;
    let mut r = boost;

    while l + 1 < r {
        let mid = l + (r - l) / 2;
        if immune_system_wins(t, mid).is_some() {
            r = mid
        } else {
            l = mid
        }
    }

    for i in 0..=r {
        if let Some(rest) = immune_system_wins(t, i) {
            return rest;
        }
    }
    // Unreachable.
    panic!()
}

pub fn run(filename: &str) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let t = parse(&contents);
    println!("{:?}", t);
    println!("{}", t.clone().remaining_army_size());
    println!("{}", part2(&t));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let e = parse_group(
            Side::Infection,
            1,
            "18 units each with 729 hit points (weak to fire; immune to cold, slashing)
 with an attack that does 8 radiation damage at initiative 10",
        );
        assert_eq!(e.effective_power(), 144)
    }
}
