use crate::intcode;

fn compute_output_signal(program: &Vec<i64>, phase_settings: &Vec<i64>) -> i64 {
    let mut amp_a = intcode::T::new(program);
    let mut amp_b = intcode::T::new(program);
    let mut amp_c = intcode::T::new(program);
    let mut amp_d = intcode::T::new(program);
    let mut amp_e = intcode::T::new(program);

    amp_a.push(phase_settings[0]);
    amp_b.push(phase_settings[1]);
    amp_c.push(phase_settings[2]);
    amp_d.push(phase_settings[3]);
    amp_e.push(phase_settings[4]);

    // Propage signal
    amp_a.push(0);
    amp_b.push(amp_a.pop().unwrap());
    amp_c.push(amp_b.pop().unwrap());
    amp_d.push(amp_c.pop().unwrap());
    amp_e.push(amp_d.pop().unwrap());

    // Get final output
    amp_e.pop().unwrap()
}

fn compute_output_signal_with_feedback(program: &Vec<i64>, phase_settings: &Vec<i64>) -> i64 {
    let mut amps = vec![
        intcode::T::new(program),
        intcode::T::new(program),
        intcode::T::new(program),
        intcode::T::new(program),
        intcode::T::new(program),
    ];
    for i in 0..5 {
        amps[i].push(phase_settings[i])
    }

    amps[0].push(0);

    let mut amp_e_values = vec![];
    while !amps.iter_mut().all(|amp| amp.is_halted()) {
        for i in 0..5 {
            if let Some(x) = amps[i].pop() {
                amps[(i + 1) % 5].push(x);
                if i == 4 {
                    amp_e_values.push(x)
                }
            }
        }
    }

    while let Some(e) = amps[4].pop() {
        amp_e_values.push(e)
    }

    //    println!("{:?} {:?}", phase_settings, amp_e_values);

    amp_e_values.pop().unwrap()
}

fn generate_permutations_aux<T>(k: usize, v: &mut Vec<T>, output: &mut Vec<Vec<T>>)
where
    T: Clone,
{
    if k == 1 {
        output.push(v.clone())
    } else {
        generate_permutations_aux(k - 1, v, output);
        for i in 0..k - 1 {
            if k % 2 == 0 {
                v.swap(i, k - 1)
            } else {
                v.swap(0, k - 1)
            };
            generate_permutations_aux(k - 1, v, output)
        }
    }
}

fn generate_permutations<T>(v: &Vec<T>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut v = v.clone();
    let k = v.len();
    let mut output = vec![];
    generate_permutations_aux(k, &mut v, &mut output);
    output
}

fn best_signal_without_feedback(program: &Vec<i64>) -> Option<i64> {
    let permutations = generate_permutations(&vec![0, 1, 2, 3, 4]);

    let best_signal = permutations
        .iter()
        .map(|perm| compute_output_signal(&program, &perm))
        .max();

    best_signal
}

fn best_signal_with_feedback(program: &Vec<i64>) -> Option<i64> {
    let permutations = generate_permutations(&vec![5, 6, 7, 8, 9]);

    let best_signal = permutations
        .iter()
        .map(|perm| compute_output_signal_with_feedback(&program, &perm))
        .max();

    best_signal
}

pub fn run(filename: String) {
    let program = intcode::read_intcode_program(&filename);
    let mut permutations = generate_permutations(&vec![0, 1, 2, 3, 4]);

    let best_signal = permutations
        .iter()
        .map(|perm| compute_output_signal(&program, &perm))
        .max();
    println!("{:?}", best_signal);

    let mut permutations = generate_permutations(&vec![5, 6, 7, 8, 9]);

    let best_signal = permutations
        .iter()
        .map(|perm| compute_output_signal_with_feedback(&program, &perm))
        .max();

    println!("{:?}", best_signal);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part2_example1() {
        let program = intcode::from_string(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        );
        assert_eq!(best_signal_with_feedback(&program), Some(139629729));
    }
}
