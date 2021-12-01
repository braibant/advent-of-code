use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_range(s: &str) -> (u64, u64) {
    if let Ok((low, high)) = scan_fmt!(s, "{d}-{d}", u64, u64) {
        return (low, high);
    } else {
        panic!("Invalid input {}", s)
    }
}

fn parse_ticket(s: &str) -> Vec<u64> {
    let mut result = Vec::new();
    let nums: Vec<&str> = s.split(",").collect();
    for num in nums.iter() {
        let num: u64 = num.parse().unwrap_or_else(|_| panic!("{}", s));
        result.push(num)
    }
    result
}

type Range = (u64, u64);

fn in_range(num: u64, r: &Range) -> bool {
    let (low, high) = r;
    *low <= num && num <= *high
}

fn ticket_error_rate(fields: &Vec<(String, Range, Range)>, ticket: &Vec<u64>) -> u64 {
    let mut result = 0;
    for num in ticket.iter() {
        let mut valid_in_one_field = false;
        for (_field, r1, r2) in fields.iter() {
            valid_in_one_field |= in_range(*num, r1) || in_range(*num, r2)
        }
        if !valid_in_one_field {
            result += num
        }
    }
    return result;
}

fn part1(fields: &Vec<(String, Range, Range)>, tickets: &Vec<Vec<u64>>) -> u64 {
    tickets
        .iter()
        .fold(0, |sum, ticket| ticket_error_rate(&fields, &ticket) + sum)
}

/// Check that the range is valid for column [i] in this tickets
fn range_is_valid(ticket: &Vec<u64>, i: usize, r1: &Range, r2: &Range) -> bool {
    let num = ticket[i];
    in_range(num, r1) || in_range(num, r2)
}

/// Check that the range is valid for column [i] in all tickets.
fn range_is_valid_for_tickets(ticket: &Vec<Vec<u64>>, i: usize, r1: &Range, r2: &Range) -> bool {
    ticket
        .iter()
        .all(|ticket| range_is_valid(ticket, i, r1, r2))
}

fn find_permutation(
    fields: &Vec<(String, Range, Range)>,
    tickets: &mut Vec<Vec<u64>>,
    i: usize,
) -> bool {
    // columns in 0..i are valid for all tickets.
    if i == fields.len() {
        return true;
    } else {
        let (_, r1, r2) = fields[i];
        for j in i..fields.len() {
            if range_is_valid_for_tickets(&tickets, j, &r1, &r2) {
                // swap the tickets
                for n in 0..tickets.len() {
                    let ticket = &mut tickets[n];
                    ticket.swap(i, j)
                }
                if find_permutation(fields, tickets, i + 1) {
                    return true;
                } else {
                }
            }
        }
        return false;
    }
}

// Check the number of matching columns with indices greater [i] for a given field
fn number_of_matching_columns(
    tickets: &Vec<Vec<u64>>,
    i: usize,
    len: usize,
    r1: &Range,
    r2: &Range,
) -> u64 {
    let mut count = 0;
    for j in i..len {
        if range_is_valid_for_tickets(tickets, j, r1, r2) {
            count += 1
        }
    }
    return count;
}

fn part2(fields: &mut Vec<(String, Range, Range)>, tickets: &Vec<Vec<u64>>) -> u64 {
    let len = fields.len();
    let mut valid_tickets = Vec::new();

    for ticket in tickets
        .iter()
        .filter(|ticket| ticket_error_rate(&fields, &ticket) == 0)
    {
        valid_tickets.push(ticket.clone())
    }

    // Heuristic: sort the fields by increasing number of matching columns
    fields.sort_by_cached_key(|(_, r1, r2)| {
        number_of_matching_columns(&valid_tickets, 0, len, r1, r2)
    });

    // The goal here is to find a permutation of the fields, such that, for every ticket T, field(T) is valid. My first assumption was to assume that there was a unique fit for each column, and that we could greadily assign matches, but it's apparently not the case.
    let _found = find_permutation(fields, &mut valid_tickets, 0);

    // Check that we have a solution: columns in 0..i are valid for all tickets.
    for i in 0..len {
        let (_, r1, r2) = fields[i];
        assert!(range_is_valid_for_tickets(&valid_tickets, i, &r1, &r2))
    }

    let mut result: u64 = 1;
    let your_ticket = &valid_tickets[0];
    for i in 0..fields.len() {
        let (field, _, _) = &fields[i];
        if field.starts_with("departure") {
            result *= your_ticket[i]
        }
    }
    return result;
}

pub fn run(filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut fields = Vec::new();

    let mut tickets = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if let Ok((field, range1, range2)) =
            scan_fmt!(&line, "{[^:]}: {[^ ]} or {[^ ]}", String, String, String)
        {
            let range1 = parse_range(&range1);
            let range2 = parse_range(&range2);
            fields.push((field, range1, range2))
        } else if &line == "your ticket:" || &line == "nearby tickets:" || &line == "" {
            continue;
        } else {
            let ticket = parse_ticket(&line);
            tickets.push(ticket);
        }
    }

    println!("{}", part1(&fields, &tickets));

    println!("{}", part2(&mut fields, &tickets))
}
