const MOD: u64 = 20201227;
const SN: u64 = 7;

fn loop_size(tgt: u64) -> usize {
    let mut n = 1;
    let mut loops = 0;

    while n != tgt {
        n = (n * SN).rem_euclid(MOD);
        loops += 1;
    }

    loops
}

fn transform(iterations: usize, sn: u64) -> u64 {
    let mut n = 1;

    for _i in 0..iterations {
        n = (n * sn).rem_euclid(MOD);
    }
    n
}

fn encryption_key(n1: u64, n2: u64) -> u64 {
    let l1 = loop_size(n1);
    //    let l2 = loop_size(n2);

    transform(l1, n2)
}

pub fn run() {
    let n1: u64 = 13135480;
    let n2: u64 = 8821721;

    println!("{}", encryption_key(n1, n2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n1: u64 = 5764801;
        assert_eq!(loop_size(n1), 8);

        let n2: u64 = 17807724;
        assert_eq!(loop_size(n2), 11);

        assert_eq!(transform(8, SN), n1);
        assert_eq!(transform(11, SN), n2);

        assert_eq!(encryption_key(n1, n2), 14897079);
    }
}
