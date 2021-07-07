const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYERS: usize = 100; //   img.len() / (WIDTH * HEIGHT);

fn digit(img: &[u8], x: usize, y: usize, layer: usize) -> u8 {
    if x < WIDTH && y < HEIGHT {
        match img[x + y * WIDTH + layer * HEIGHT * WIDTH] {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            c => panic!("Invalid character {}", c),
        }
    } else {
        panic!("Invalid coordinates")
    }
}

fn digits(img: &[u8], layer: usize, tgt: u8) -> usize {
    let mut acc = 0;
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if digit(img, x, y, layer) == tgt {
                acc += 1;
            }
        }
    }
    acc
}

fn render_pixel(img: &[u8], x: usize, y: usize, layer: usize) -> char {
    let d = digit(img, x, y, layer);
    match d {
        0 => ' ',
        1 => 'X',
        2 => render_pixel(img, x, y, layer + 1),
        _ => panic!("Never"),
    }
}

fn print(img: &[u8]) {
    let mut acc = String::new();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            acc.push(render_pixel(img, x, y, 0))
        }
        acc.push('\n');
    }
    println!("{}", acc);
}

pub fn run(filename: String) {
    let content: String = std::fs::read_to_string(filename).unwrap();
    let img = content.as_bytes();
    let layer = (0..LAYERS)
        .min_by_key(|&layer| digits(img, layer, 0))
        .unwrap();

    println!("{}", digits(img, layer, 1) * digits(img, layer, 2));

    print(img);
}
