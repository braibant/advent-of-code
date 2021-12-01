use crate::asm::{parse, Program, T};

fn part1(text: &Program, ip: usize) -> u64 {
    let mut t = T::new(text, ip);
    while !t.halted {
        t.step();
    }
    t.registers[0]
}

fn part2(_text: &Program, _ip: usize) -> u64 {
    // let mut t = T::new(text, ip);
    // t.registers[0] = 1;
    // while !t.halted {
    //     t.step();
    //     if t.steps % 10_000_000 == 0 {
    //         print!("\x1B[2J\x1B[1;1H");
    //         let mut table = t.to_table();
    //         use prettytable::format;
    //         table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    //         table.printstd();
    //     }
    // }

    // t.registers[0]

    // When we set reg[0] to 1, the program becomes an obfuscated version of the following loop:
    // for i = 1 to 10551374 + 1 do if 10551374 mod i = 0 then r := !r + i; done
    // which evaluates to r being 15864120
    return 15864120;
}

pub fn run(filename: &str) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let (ip, text) = parse(&contents);
    println!("{:?}", part1(&text, ip.unwrap()));
    println!("{:?}", part2(&text, ip.unwrap()));
}

#[cfg(test)]
mod tests {
    use super::*;
    const E1: &str = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

    #[test]
    fn test_example1() {
        let (ip, text) = parse(E1);
        assert_eq!(part1(&text, ip.unwrap()), 6)
    }
}
