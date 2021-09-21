use std::collections::HashMap;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum Event {
    WakeUp,
    Sleep,
    Guard(u32),
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct LogEntry {
    date: chrono::NaiveDate,
    timestamp: u32,
    event: Event,
}

fn parse_log_entry(s: &str) -> Option<LogEntry> {
    if s == "" {
        None
    } else {
        let words: Vec<_> = s.split(' ').collect();
        let date = words[0].strip_prefix('[').unwrap();
        let time = words[1].strip_suffix(']').unwrap();
        let time: Vec<_> = time.split(':').collect();
        let event = if words[2] == "Guard" {
            let guard: u32 = words[3].strip_prefix('#').unwrap().parse().unwrap();
            Event::Guard(guard)
        } else if words[2] == "falls" {
            Event::Sleep
        } else if words[2] == "wakes" {
            Event::WakeUp
        } else {
            panic!("{}", s)
        };
        Some(LogEntry {
            date: chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap(),
            timestamp: time[1].parse().unwrap(),
            event,
        })
    }
}

struct T {
    id: u32,
    sleeping_at: Vec<u32>, // The number of times we have seen this guard sleep at this time
    sleeping: Option<u32>, // The time the guard fell asleep
}

impl T {
    fn new(id: u32) -> T {
        let sleeping_at = (0..60).map(|_i| 0).collect();
        T {
            id,
            sleeping_at,
            sleeping: None,
        }
    }

    fn total_sleep(&self) -> u32 {
        self.sleeping_at.iter().sum()
    }

    fn sleep(&mut self, timestamp: u32) {
        if self.sleeping.is_some() {
            panic!("Guard {}: Double sleep", self.id)
        };
        self.sleeping = Some(timestamp)
    }

    fn wakeup(&mut self, timestamp: u32) {
        // The concrete instance given has a sleep-guard-wakeup sequence. We
        // choose to implement the wakeup as a no-op if the guard that took his
        // shift has not fallen asleep yet.
        if !self.sleeping.is_none() {
            let mut i = self.sleeping.unwrap();
            while i != timestamp {
                self.sleeping_at[i as usize] += 1;
                i = (i + 1) % 60
            }
            self.sleeping = None
        }
    }
}

// We want to identify: for each guard, a) how many minutes they sleep in total
// accross all days in the sample; b) which minute they are asleep most. There
// are some tricky cases to consider: guard does not fall asleep in the time we
// survey, guard does not wake up in the time we survey, guard enters the play,
// then day changes, then guard sleep (in which case, we assume that the guard
// is the same). We assume that we see a "wake up" event for each "sleep" event
// (and will fail otherwise). We realize that the date part can be mostly
// ignored once the events have been ordered, and that we just need to keep for
// each guard: the total number of times they are asleep, the number of time
// they were seen sleeping at a given time

fn build_schedule(log: &[LogEntry]) -> HashMap<u32, T> {
    let mut guard: Option<(u32, T)> = None;
    let mut guards = HashMap::new();
    for entry in log.iter() {
        match entry.event {
            Event::Guard(i) => {
                if let Some((idx, mut t)) = guard {
                    if t.sleeping.is_some() {
                        t.wakeup(entry.timestamp);
                    }
                    guards.insert(idx, t);
                };
                let t = guards.remove(&i).unwrap_or(T::new(i));
                guard = Some((i, t));
            }
            Event::Sleep => {
                let (_idx, t) = guard.as_mut().unwrap();
                t.sleep(entry.timestamp)
            }
            Event::WakeUp => {
                let (_idx, t) = guard.as_mut().unwrap();
                t.wakeup(entry.timestamp)
            }
        }
    }
    if let Some((idx, t)) = guard {
        guards.insert(idx, t);
    };
    guards
}

fn part1(log: &[LogEntry]) -> u32 {
    let guards = build_schedule(log);
    let (guard_id, guard) = guards.iter().max_by_key(|(_, g)| g.total_sleep()).unwrap();
    let (best_time, _) = guard
        .sleeping_at
        .iter()
        .enumerate()
        .max_by_key(|(_, c)| *c)
        .unwrap();
    guard_id * (best_time as u32)
}

fn part2(log: &[LogEntry]) -> u32 {
    let guards = build_schedule(log);
    let (id, min, _count) = guards
        .iter()
        .flat_map(|(&id, t)| {
            t.sleeping_at
                .iter()
                .enumerate()
                .map(move |(min, count)| (id, min, count))
        })
        .max_by_key(|(_id, _min, count)| *count)
        .unwrap();
    id * (min as u32)
}

pub fn run(filename: &str) {
    let content = std::fs::read_to_string(filename).unwrap();
    let mut log: Vec<_> = content.split('\n').filter_map(parse_log_entry).collect();
    log.sort();
    println!("{}", part1(&log));
    println!("{}", part2(&log));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_log_entry() {
        let s = "[1518-11-01 00:00] Guard #10 begins shift";
        let entry = parse_log_entry(s).unwrap();
        assert_eq!(
            entry,
            LogEntry {
                event: Event::Guard(10),
                timestamp: 0,
                date: chrono::NaiveDate::from_ymd(1518, 11, 1)
            }
        )
    }
}
