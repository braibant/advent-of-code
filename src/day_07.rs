use crate::intcode::*;

fn compute_output_signal(program: &Vec<i64>, phase_settings: &Vec<i64>) -> i64 {
    let out_a = execute(&mut program.clone(), &mut vec![0, phase_settings[0]]);
    let out_b = execute(&mut program.clone(), &mut vec![out_a[0], phase_settings[1]]);
    let out_c = execute(&mut program.clone(), &mut vec![out_b[0], phase_settings[2]]);
    let out_d = execute(&mut program.clone(), &mut vec![out_c[0], phase_settings[3]]);
    let out_e = execute(&mut program.clone(), &mut vec![out_d[0], phase_settings[4]]);
    out_e[0]
}

fn generate_permutations_aux<T>(k: usize, v: &mut Vec<T>, output: &mut Vec<Vec<T>>)
where
    T: Clone,
{
    if k == 1 {
        output.push(v.clone())
    } else {
        generate_permutations_aux(k - 1, v, output);
        for i in 0..k {
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

pub fn run(filename: String) {
    let program = read_intcode_program(&filename);
    let mut permutations = generate_permutations(&vec![0, 1, 2, 3, 4]);

    let best_signal = permutations
        .iter()
        .map(|perm| compute_output_signal(&program, &perm))
        .max();
    println!("{:?}", best_signal)
}
