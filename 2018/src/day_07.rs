use std::collections::HashMap;
use std::collections::HashSet;

struct T {
    happens_before: HashMap<char, Vec<char>>,
    happens_after: HashMap<char, Vec<char>>,
    universe: HashSet<char>,
}

impl T {
    fn roots(&self) -> Vec<char> {
        self.universe
            .iter()
            .filter(|x| !self.happens_before.contains_key(x))
            .copied()
            .collect()
    }
}

fn parse(s: &str) -> T {
    let mut happens_before = HashMap::new();
    let mut happens_after = HashMap::new();
    let mut universe = HashSet::new();
    for line in s.split('\n') {
        if !line.is_empty() {
            // Step S must be finished before step P can begin.
            // I.e. P after S, S before P
            // I.e. happens before P = [S], happens after S = [P]
            let words: Vec<_> = line.split(' ').collect();
            let before = words[1].chars().next().unwrap();
            let after = words[7].chars().next().unwrap();
            let entry = happens_before.entry(after).or_insert_with(Vec::new);
            entry.push(before);
            let entry = happens_after.entry(before).or_insert_with(Vec::new);
            entry.push(after);
            universe.insert(before);
            universe.insert(after);
        }
    }
    T {
        happens_before,
        happens_after,
        universe,
    }
}

// fn visit(t: &T, marks: &mut HashMap<char, usize>, n: char) -> usize {
//     match marks.get(&n) {
//         Some(&h) => h,
//         None => {
//             let before = t.before(n);
//             if before.is_empty() {
//                 marks.insert(n, 0);
//                 0
//             } else {
//                 let mut height: Vec<usize> = Vec::new();
//                 for i in t.before(n) {
//                     height.push(visit(t, marks, i))
//                 }
//                 let h = 1 + height.iter().max().unwrap();
//                 marks.insert(n, h);
//                 h
//             }
//         }
//     }
// }

// fn topological_sort(t: &T) -> Vec<char> {
//     let mut marks: HashMap<char, usize> = HashMap::new();
//     let mut todo: Vec<char> = t.universe.iter().copied().collect();
//     while !todo.is_empty() {
//         let n = todo.pop().unwrap();
//         visit(t, &mut marks, n);
//     }
//     let mut heights: Vec<_> = marks.iter().collect();
//     heights.sort_by_key(|(&n, &h)| (h, n));
//     heights.into_iter().map(|(&n, _h)| n).collect()
// }

fn part1(t: &T) -> String {
    let mut ready: Vec<char> = t.roots();
    let mut result = Vec::new();
    let mut happens_before = t.happens_before.clone();
    while !ready.is_empty() {
        let &n = ready.iter().min().unwrap();
        ready.retain(|&x| x != n);
        result.push(n);
        for a in t.happens_after.get(&n).unwrap_or(&Vec::new()).iter() {
            let mut preds = happens_before.remove(a).unwrap();
            preds.retain(|&x| x != n);
            if preds.is_empty() {
                ready.push(*a)
            } else {
                happens_before.insert(*a, preds);
            }
        }
    }
    result.iter().collect()
}

struct Job {
    finish_at: usize,
    id: char,
}

struct State<'a> {
    clock: usize,
    workers: Vec<Option<Job>>,
    ready: Vec<char>,
    happens_before: HashMap<char, Vec<char>>,
    dependencies: &'a T,
}

impl<'a> State<'a> {
    fn new(t: &'a T, n: usize) -> State {
        State {
            ready: t.roots(),
            workers: (0..n).map(|_i| None).collect(),
            clock: 0,
            happens_before: t.happens_before.clone(),
            dependencies: t,
        }
    }

    fn next_completion(&self) -> Option<usize> {
        self.workers
            .iter()
            .filter_map(|job_maybe| job_maybe.as_ref().map(|job| job.finish_at))
            .min()
    }

    // Returns the list of job we have completed
    fn advance_clock(&mut self, clock: usize) -> Vec<char> {
        self.clock = clock;
        let mut result = Vec::new();
        for i in 0..self.workers.len() {
            match &self.workers[i] {
                None => {}
                Some(job) => {
                    if job.finish_at < clock {
                        result.push(job.id);
                        self.workers[i] = None
                    }
                }
            }
        }
        result
    }
    // Returns the list of jobs that are now ready
    fn complete_job(&mut self, job: char) -> Vec<char> {
        let mut result = Vec::new();
        for a in self
            .dependencies
            .happens_after
            .get(&job)
            .unwrap_or(&Vec::new())
            .iter()
        {
            let mut preds = self.happens_before.remove(&a).unwrap();
            preds.retain(|&x| x != job);
            if preds.is_empty() {
                result.push(*a)
            } else {
                self.happens_before.insert(*a, preds);
            }
        }
        result
    }

    fn available_workers(&self) -> usize {
        self.workers.iter().filter(|j| j.is_none()).count()
    }
}

fn part2(t: &T, n: usize, speed: usize) -> usize {
    let mut state = State::new(t, n);
    loop {
        if state.ready.is_empty() && state.workers.iter().all(|j| j.is_none()) {
            break;
        } else if state.available_workers() == 0 || state.ready.is_empty() {
            // The only thing we can do is wait for a worker to complete.
            let next_completion = state.next_completion().unwrap();
            let done = state.advance_clock(next_completion + 1);
            let mut newly_ready = Vec::new();
            for d in done.iter() {
                newly_ready.extend(state.complete_job(*d))
            }
            state.ready.append(&mut newly_ready)
        } else {
            // We have some jobs to allocate to workers, and some workers
            for i in 0..state.workers.len() {
                if state.workers[i].is_none() {
                    if let Some(id) = state.ready.pop() {
                        let finish_at = state.clock + speed + (((id as u8) - b'A') as usize);
                        state.workers[i] = Some(Job { finish_at, id })
                    }
                }
            }
        }
    }
    state.clock
}

pub fn run(filename: &str) {
    let content = std::fs::read_to_string(filename).unwrap();
    let deps = parse(&content);
    println!("{}", part1(&deps));
    println!("{}", part2(&deps, 5, 60));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn test_part1() {
        let t = parse(EXAMPLE);
        assert_eq!(part1(&t), "CABDFE");
    }

    #[test]
    fn test_part2() {
        let t = parse(EXAMPLE);
        assert_eq!(part2(&t, 2, 0), 15);
    }
}
