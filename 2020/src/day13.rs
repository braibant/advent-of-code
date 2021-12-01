use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run(filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let start: u64 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut bus_ids = {
        let mut bus_ids = Vec::new();
        let line: String = lines.next().unwrap().unwrap();
        let entries: Vec<&str> = line.split(",").collect();
        for (index, bus) in entries.iter().enumerate() {
            if bus == &"x" {
                continue;
            } else {
                let bus_id: u64 = bus.parse().unwrap();
                bus_ids.push((index as u64, bus_id))
            }
        }
        bus_ids
    };

    // part 1
    let (_offset, earliest_bus) = bus_ids
        .iter()
        .min_by_key(|(_offset, id)| {
            let q = start / *id;
            let time_to_wait = (q + 1) * id - start;
            time_to_wait
        })
        .unwrap();

    let time_to_wait = (1 + start / earliest_bus) * earliest_bus - start;
    println!("{}", earliest_bus * time_to_wait);

    // part 2
    // the input describes some congruences. We want to find the smallest t such that `t + offset_i mod bus_i = 0`.
    // It turns out that the bus_ids that are provided are prime numbers, and we can search for a solution
    // using a sieve (see https://en.wikipedia.org/wiki/Chinese_remainder_theorem).

    bus_ids.sort_by_key(|(_offset, id)| *id);
    bus_ids.reverse();
    let (a1, n1) = bus_ids[0];
    let mut t: u64 = (n1 - a1) % n1;
    let mut n: u64 = n1;
    for (offset, id) in bus_ids[1..].iter() {
        while (t + offset) % id != 0 {
            t += n;
        }
        n = n * *id;
    }
    println!("{}", t);
}
