#[derive(Clone, Copy, Debug)]
struct Vector3<T> {
    x: T,
    y: T,
    z: T,
}

use std::ops::Add;
impl<T: Add<Output = T>> Add for Vector3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x: x, y: y, z: z }
    }
}

use std::ops::Index;
impl<T> Index<usize> for Vector3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index"),
        }
    }
}

#[derive(Debug)]
struct T {
    position: Vector3<i64>,
    velocity: Vector3<i64>,
}

impl T {
    fn new(position: Vector3<i64>) -> T {
        let velocity: Vector3<i64> = Vector3::new(0, 0, 0);
        T { position, velocity }
    }
}

fn apply_gravity(state: &mut [T]) {
    // `delta[i] denotes the force applied to planet i
    let mut delta = vec![];
    let zero: Vector3<i64> = Vector3::new(0, 0, 0);

    delta.resize(state.len(), zero);
    for i in 0..state.len() {
        for j in 0..state.len() {
            if i != j {
                if state[i].position.x < state[j].position.x {
                    delta[i].x += 1
                };
                if state[i].position.y < state[j].position.y {
                    delta[i].y += 1
                };
                if state[i].position.z < state[j].position.z {
                    delta[i].z += 1
                };

                if state[i].position.x > state[j].position.x {
                    delta[i].x -= 1
                };
                if state[i].position.y > state[j].position.y {
                    delta[i].y -= 1
                };
                if state[i].position.z > state[j].position.z {
                    delta[i].z -= 1
                };
            }
        }
    }

    for i in 0..delta.len() {
        state[i].velocity = state[i].velocity + delta[i]
    }
}

fn apply_velocity(state: &mut [T]) {
    for t in state.iter_mut() {
        t.position = t.position + t.velocity
    }
}

fn norm1(t: &Vector3<i64>) -> i64 {
    t.x.abs() + t.y.abs() + t.z.abs()
}
fn energy(t: &T) -> i64 {
    norm1(&t.position) * norm1(&t.velocity)
}

fn simulate(positions: &[Vector3<i64>], steps: usize) -> Vec<T> {
    let mut state: Vec<T> = positions.iter().map(|&p| T::new(p)).collect();

    for _ in 0..steps {
        apply_gravity(&mut state);
        apply_velocity(&mut state);
    }
    state
}

fn total_energy(state: &[T]) -> i64 {
    state.iter().map(|t| energy(t)).sum()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct T1 {
    position: i64,
    velocity: i64,
}

fn step_1d(state: &[T1; 4]) -> [T1; 4] {
    let mut next = *state;
    for i in 0..4 {
        for j in 0..4 {
            if i != j {
                if state[i].position < state[j].position {
                    next[i].velocity += 1
                };
                if state[i].position > state[j].position {
                    next[i].velocity -= 1
                };
            }
        }
    }
    for i in 0..4 {
        next[i].position += next[i].velocity
    }
    next
}

// Returns (n,m) where m is the index of the first state that matches a previous state (n)
fn find_period_1d(state: &[T1; 4]) -> (usize, usize) {
    use std::collections::HashMap;

    let mut state = *state;
    let mut i: usize = 0;
    let mut seen = HashMap::new();
    loop {
        match seen.get(&state) {
            None => seen.insert(state, i),
            Some(&previous) => return (previous, i),
        };
        state = step_1d(&state);
        i += 1;
    }
}

fn project(state: &[T], dimension: usize) -> [T1; 4] {
    assert!(state.len() == 4);

    let mut state_1d = [T1 {
        position: 0,
        velocity: 0,
    }; 4];
    for i in 0..4 {
        state_1d[i].position = state[i].position[dimension];
        state_1d[i].velocity = state[i].velocity[dimension];
    }
    state_1d
}

fn find_period_3d(state: &[T]) -> Vec<(usize, usize)> {
    let mut periods = vec![];
    for dimension in 0..3 {
        periods.push(find_period_1d(&project(state, dimension)))
    }
    periods
}

pub fn run() {
    let positions: Vec<Vector3<i64>> = vec![
        Vector3::new(13, -13, -2),
        Vector3::new(16, 2, -15),
        Vector3::new(7, -18, -12),
        Vector3::new(-3, -8, -8),
    ];

    let state = simulate(&positions, 1000);

    println!("{}", total_energy(&state));

    // Part 2: we can make the observation that the accelation depends only on
    // the ordering on 3 axis of the 4 bodies. Also, there is no interaction
    // between the three axis: we could compute the periodicity on each axis,
    // and find the lowest common multiple of the periodicity on each axis.

    let periods = find_period_3d(&state);

    // It turns out that the first repeating state on a given axis is the
    // projection of the initial state along this axis. This simplifies the
    // problem quite a bit, since we "just" need to compute the lcm of the
    // periods involved to figure out the period of the whole system. Here, I simply used wolfram alpha to compute the lcm
    println!("{:?}", periods);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_part1() {
        let positions = vec![
            Vector3::new(-1, 0, 2),
            Vector3::new(2, -10, -7),
            Vector3::new(4, -8, 8),
            Vector3::new(3, 5, -1),
        ];
        let state = simulate(&positions, 10);
        assert_eq!(total_energy(&state), 179);
    }
}
