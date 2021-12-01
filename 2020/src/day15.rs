use std::collections::HashMap;

fn step(state: &mut HashMap<u32, u32>, turn: u32, num: u32) -> u32 {
    let previous = state.entry(num).or_insert(turn - 1);
    let result = turn - 1 - *previous;
    *previous = turn - 1;
    result
}

fn play(nums: &Vec<u32>, until: u32) -> u32 {
    let mut state: HashMap<u32, u32> = HashMap::new();
    let mut most_recent_num: u32 = 0;
    let mut turn: u32 = 1;

    // setup
    for num in nums.iter() {
        // println!("{}: {}", turn, num);
        state.insert(*num, turn);
        most_recent_num = *num;
        turn += 1
    }

    // play
    while turn <= until {
        most_recent_num = step(&mut state, turn, most_recent_num);
        // println!("{}: {}", turn, most_recent_num);
        turn += 1;
    }

    most_recent_num
}

pub fn run() {
    let nums = vec![15, 5, 1, 4, 7, 0];
    println!("{}", play(&nums, 2020));
    println!("{}", play(&nums, 30000000));
}
