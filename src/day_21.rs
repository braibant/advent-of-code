use std::collections::HashSet;

enum State {
    L6,
    L8,
    L28,
}

#[derive(PartialEq)]
enum Mode {
    Min,
    Max,
}

// The program that we are given implement the following construct
fn secret(seed: u64, mode: Mode) -> u64 {
    let mut r1 = 0;
    let mut r2 = 0;
    let mut r4 = 0;
    let mut pre_r4 = 0;
    let mut visited = HashSet::new();

    let mut state = State::L6;

    loop {
        match state {
            State::L6 => {
                r2 = r4 | 0b10000000000000000;
                r4 = 6152285;
                state = State::L8
            }
            State::L8 => {
                r1 = r2 & 0b11111111;
                r4 = (r4 + r1) & 0b111111111111111111111111;
                r4 = (r4 * 65899) & 0b111111111111111111111111;
                if 256 > r2 {
                    state = State::L28;
                } else {
                    r2 = r2 / 256;
                    state = State::L8
                }
            }
            State::L28 => {
                if mode == Mode::Max && !visited.insert(r4) {
                    // We have found a cycle in the values in R4.
                    // return the previous value
                    return pre_r4;
                }
                if mode == Mode::Min {
                    return r4;
                }
                pre_r4 = r4;
                if r4 == seed {
                    panic!();
                } else {
                    state = State::L6;
                }
            }
        }
    }
}
// }

// let r0 = ref 0
// let r1 = ref 0
// let r2 = ref 0
// let r4 = ref 0
// let visited = Hashtbl.create 52

// let reset () = r0 := 0; r1 := 0; r2 := 0; r4 := 0; Hashtbl.clear visited
// let print () = Printf.printf "r0:%i r1:%i r2: %i r4: %i\n" !r0 !r1 !r2 !r4;;

// exception Loop

// let rec l6 () =
//    r2 := !r4 lor 0b10000000000000000;
//    r4 := 6152285;
//    l8 ()
// and l8 () =
//    r1 := !r2 land 0b11111111;
//    r4 := (!r4 + !r1) land 0b111111111111111111111111;
//    r4 := (!r4 * 65899) land 0b111111111111111111111111;
//    if 256 > !r2
//    then l28 ()
//    else (r2 := (!r2 / 256) ; l8 ())
// and l28 () =
// print () ;
// if Hashtbl.mem visited !r4
// then (raise Loop);
// Hashtbl.add visited !r4 ();
//    if !r4 = !r0
//    then ()
// else l6 ()
// ;;

pub fn run(s: &str) {
    // let contents = std::fs::read_to_string(s).unwrap();
    // let (ipr, text) = crate::asm::parse(&contents);
    // let mut t = crate::asm::T::new(&text, ipr.unwrap());

    println!("{}", secret(0, Mode::Min));
    println!("{}", secret(0, Mode::Max));
}
