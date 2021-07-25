use crate::intcode;
use crossterm::event::{read, Event};
use crossterm::queue;
use crossterm::{
    cursor,
    style::{self, Stylize},
    // terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::collections::HashMap;
use std::io::{stdout, Write};

fn part1(program: &[i64]) {
    let mut vm = intcode::T::new(&program);
    intcode::execute(&mut vm);
    let mut part1 = 0;
    let mut o = vm.flush();
    o.reverse();
    while !o.is_empty() {
        let _x = o.pop().unwrap();
        let _y = o.pop().unwrap();
        let tile = o.pop().unwrap();
        if tile == 2 {
            part1 += 1
        };
    }
    println!("{} {}", part1, vm.steps())
}

fn draw(state: &HashMap<(i64, i64), i64>, score: i64) {
    let mut stdout = stdout();
    queue!(
        stdout,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )
    .unwrap();

    queue!(
        stdout,
        cursor::MoveTo(100, 10),
        style::PrintStyledContent((format!("{}", score)).red())
    )
    .unwrap();
    for (&(x, y), tile) in state.iter() {
        queue!(stdout, cursor::MoveTo(x as u16, y as u16)).unwrap();
        match tile {
            0 => queue!(stdout, style::PrintStyledContent("█".black())).unwrap(),
            1 => queue!(stdout, style::PrintStyledContent("█".red())).unwrap(),
            2 => queue!(stdout, style::PrintStyledContent("█".blue())).unwrap(),
            3 => queue!(stdout, style::PrintStyledContent("█".green())).unwrap(),
            4 => queue!(stdout, style::PrintStyledContent("o".yellow())).unwrap(),
            _ => panic!(),
        };
    }
    stdout.flush().unwrap();
}

// output instructions specify the x position (distance from the left), y
// position (distance from the top), and tile id. The tile id is interpreted
// as follows: 0 is an empty tile. No game object appears in this tile. 1 is
// a wall tile. Walls are indestructible barriers. 2 is a block tile. Blocks
// can be broken by the ball. 3 is a horizontal paddle tile. The paddle is
// indestructible. 4 is a ball tile. The ball moves diagonally and bounces
// off objects.
fn part2(program: &[i64]) {
    // Enable free play mode
    let mut program = program.to_vec();
    program[0] = 2;
    let mut vm = intcode::T::new(&program);
    let mut state = HashMap::new();
    let mut score = 0;
    let mut save = Box::new(None);
    let mut paddle_x = 0;
    let mut ball_x = 0;
    let mut auto = false;
    crossterm::terminal::enable_raw_mode().unwrap();
    while !(vm.is_halted() && vm.output.is_empty()) {
        draw(&state, score);
        if 3 <= vm.output.len() {
            let x = vm.pop().unwrap();
            let y = vm.pop().unwrap();
            let tile = vm.pop().unwrap();
            if x == -1 && y == 0 {
                score = tile;
            } else {
                state.insert((x, y), tile);
                if tile == 3 {
                    paddle_x = x
                };
                if tile == 4 {
                    ball_x = x
                };
            };
        } else if vm.is_blocked_on_input() {
            if auto {
                if ball_x < paddle_x {
                    vm.push(-1)
                } else if ball_x > paddle_x {
                    vm.push(1)
                } else {
                    vm.push(0)
                }
            } else {
                // `read()` blocks until an `Event` is available
                match read().unwrap() {
                    Event::Key(event) => {
                        if event.code == crossterm::event::KeyCode::Left {
                            vm.push(-1)
                        } else if event.code == crossterm::event::KeyCode::Right {
                            vm.push(1)
                        } else if event.code == crossterm::event::KeyCode::Down {
                            vm.push(0)
                        } else if event.code == crossterm::event::KeyCode::Char('z') {
                            save = Box::new(Some((vm.clone(), state.clone(), score)))
                        } else if event.code == crossterm::event::KeyCode::Char('x') {
                            let (vm1, state1, score1) = save.clone().unwrap();
                            vm = vm1;
                            state = state1;
                            score = score1;
                            draw(&state, score)
                        } else if event.code == crossterm::event::KeyCode::Char('a') {
                            auto = true
                        } else {
                            println!("{:?}", event)
                        }
                    }
                    _ => (),
                }
            }
        }
    }
    println!("Score: {} (steps: {})", score, vm.steps())
}

pub fn run(filename: &str) {
    let program = intcode::read_intcode_program(&filename);

    part1(&program);
    part2(&program);
}
