use crate::intcode;
use std::collections::VecDeque;

struct T {
    vms: Vec<intcode::T>,
    buffers: Vec<VecDeque<i64>>,
    nat: Vec<(i64, i64)>,
    txs: usize,
}

impl T {
    fn initialize(program: &[i64]) -> T {
        let vms: Vec<_> = (0..50)
            .map(|i| {
                let mut vm = intcode::T::new(program);
                vm.push(i);
                vm
            })
            .collect();
        let buffers: Vec<VecDeque<i64>> = (0..50).map(|i| VecDeque::new()).collect();
        T {
            vms,
            buffers,
            nat: vec![],
            txs: 0,
        }
    }

    fn send(&mut self, x: i64, y: i64, addr: i64) {
        self.txs += 1;
        if addr == 255 {
            self.nat.push((x, y));
        } else if 0 <= addr && addr < 50 {
            self.buffers[addr as usize].push_back(x);
            self.buffers[addr as usize].push_back(y);
        } else {
            panic!("Invalid address: {}", addr)
        }
    }

    /// Process outputs, if any
    fn tx(&mut self) {
        for i in 0..50 {
            if 3 <= self.vms[i].outputs() {
                let addr = self.vms[i].get_output().unwrap();
                let x = self.vms[i].get_output().unwrap();
                let y = self.vms[i].get_output().unwrap();
                self.send(x, y, addr)
            };
        }
    }

    /// Process inputs, if any
    fn rx(&mut self) {
        for i in 0..50 {
            // Second, feed inputs to VMs blocked on inputs
            if self.vms[i].is_blocked_on_input() {
                if self.buffers[i].is_empty() {
                    self.vms[i].push(-1)
                } else {
                    // We should have at least three inputs
                    let x = self.buffers[i].pop_front().unwrap();
                    let y = self.buffers[i].pop_front().unwrap();
                    self.vms[i].push(x);
                    self.vms[i].push(y)
                }
            }
        }
    }

    /// Execute
    fn execute(&mut self) {
        for i in 0..50 {
            self.vms[i].execute()
        }
    }
}

fn part1(program: &[i64]) -> i64 {
    let mut state = T::initialize(&program);
    loop {
        state.execute();
        state.tx();
        state.rx();
        if state.nat.len() > 0 {
            break;
        }
    }
    let (_, y) = state.nat[0];
    y
}

fn part2(program: &[i64]) -> i64 {
    let mut state = T::initialize(&program);
    let mut pre = None;
    let mut cycles = 0;
    loop {
        cycles += 1;
        if cycles % 100 == 0 {
            eprintln!("{}: {:?} {:?}", cycles, pre, state.nat);
        }
        // We check for an idle state after trying to send packets, if any. 
        state.rx();
        state.execute();
        state.tx();
        state.execute();
        if !state.nat.is_empty()
            && state.vms.iter_mut().all(|vm| vm.is_blocked_on_input())
            && state.buffers.iter().all(|buf| buf.is_empty())
        {
            // The network is idle
            let (x, y) = state.nat.pop().unwrap();
            state.nat.clear();
            state.send(x, y, 0);
            if let Some(y_pre) = pre {
                if y_pre == y {
                    break;
                }
            };
            pre = Some(y);
        }
    }
    pre.unwrap()
}

pub fn run(filename: &str) {
    let program = intcode::read_intcode_program(filename);
    println!("{}", part1(&program));
    println!("{}", part2(&program));
}
