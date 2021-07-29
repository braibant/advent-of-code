fn parse(s: &str) -> Vec<u8> {
    const RADIX: u32 = 10;
    s.chars()
        .map(|c| c.to_digit(RADIX).unwrap())
        .map(|u32| u32 as u8)
        .collect()
}

fn digit(input: &[u8], d: usize) -> u8 {
    let base_pattern: Vec<i64> = vec![0, 1, 0, -1];
    let mut acc: i64 = 0;
    for i in 0..input.len() {
        acc += (input[i] as i64) * base_pattern[((i + 1) / (d + 1)) % 4]
    }
    ((acc.abs()) % 10) as u8
}

fn phase(input: &[u8]) -> Vec<u8> {
    let mut acc = vec![];
    for n in 0..input.len() {
        acc.push(digit(input, n))
    }
    acc
}

fn fft(input: &[u8], steps: usize) -> Vec<u8> {
    let mut signal = input.to_vec();
    for _i in 0..steps {
        signal = phase(&signal)
    }
    signal
}

// Part 2. We want to compute 8 values, located at offset N..N+8, where `N > len
// / 2`. We know that phase(signal)[N] = signal[N..].sum(), phase(signal)[N
// + 1]  = signal[N + 1..].sum() = V(N) - signal(N)

fn repeat(input: &[u8], n: usize) -> Vec<u8> {
    let mut acc = vec![];
    for _i in 0..n {
        acc.extend_from_slice(input)
    }
    acc
}

fn sum(signal: &[u8]) -> u8 {
    let mut acc = 0;
    for i in 0..signal.len() {
        acc += signal[i] as i64
    }
    (acc.abs() % 10) as u8
}

fn decode(input: &[u8], repetitions: usize, offset: usize, steps: usize) -> Vec<u8> {
    assert!(offset >= (input.len() * repetitions) / 2);
    let mut signal: Vec<_> = repeat(input, repetitions)[offset..].to_vec();
    for _i in 0..steps {
        let mut acc = signal.clone();
        let mut sum = sum(&signal);
        acc[0] = sum;
        for pos in 1..signal.len() {
            sum = (10 + sum - signal[pos - 1]) % 10;
            acc[pos] = sum;
        }
        signal = acc
    }
    signal[0..8].to_vec()
}

pub fn run(filename: &str) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let signal = parse(&contents);
    let result: Vec<_> = fft(&signal, 100).iter().take(8).cloned().collect();
    println!("Part 1: {:?}", result);

    // Part 2
    let offset: usize = contents[0..7].parse().unwrap();
    println!("Offset: {}", offset);
    println!("Signal len: {}", signal.len());
    println!("{:?}", decode(&signal, 10_000, offset, 100));
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let signal0 = parse("12345678");
        let signal1 = parse("48226158");
        let signal2 = parse("34040438");
        assert_eq!(phase(&signal0), signal1);
        assert_eq!(phase(&signal1), signal2);
    }

    #[test]
    fn test_2() {
        let signal0 = parse("80871224585914546619083218645595");
        let result: Vec<_> = fft(&signal0, 100).iter().take(8).cloned().collect();
        assert_eq!(result, parse("24176176"));
    }

    #[test]
    fn test_3() {
        let signal0 = parse("19617804207202209144916044189917");
        let result: Vec<_> = fft(&signal0, 100).iter().take(8).cloned().collect();
        assert_eq!(result, parse("73745418"));
    }

    #[test]
    fn test_4() {
        let signal0 = parse("69317163492948606335995924319873");
        let result: Vec<_> = fft(&signal0, 100).iter().take(8).cloned().collect();
        assert_eq!(result, parse("52432133"));
    }

    #[test]
    fn test_decode() {
        let signal = "03036732577212944063491565474664";
        let offset: usize = signal[0..7].parse().unwrap();
        assert_eq!(offset, 0303673);
        let signal = parse(signal);
        assert_eq!(decode(&signal, 10_000, offset, 100), parse("84462026"));
        // 03036732577212944063491565474664 becomes 84462026.
    }
}
