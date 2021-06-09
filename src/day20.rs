use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// In this problem, we need to fiddle with jigsaw pieces to construct a rectangular image, and find the ids of the corner pieces. We can observe that:
// - each tile that is a corner piece will have exactly two borders that are not matching any other borders;
// - each tile that is an edge piece will have one border that does not match any other border.

// So, my guess here is that we should simply: count the number of occurences of each border, extract the corner pieces based on this, and return the product of their id (the puzzle answer).

// The problem description uses the word "flipped", but I assume that we cannot actually flip the pieces vertically or horizontally.

type Piece = (u64, Vec<bool>);

const SIZE: usize = 10;

lazy_static! {
    static ref U: Vec<usize> = {
        let mut v = Vec::new();
        for i in 0..SIZE {
            v.push(i)
        }
        v
    };
    static ref L: Vec<usize> = {
        let mut v = Vec::new();
        for i in 0..SIZE {
            v.push(i * SIZE)
        }
        v
    };
    static ref R: Vec<usize> = {
        let mut v = Vec::new();
        for i in 0..SIZE {
            v.push(9 + i * SIZE)
        }
        v
    };
    static ref D: Vec<usize> = {
        let mut v = Vec::new();
        for i in 0..SIZE {
            v.push(9 * SIZE + i)
        }
        v
    };
}

fn extract(piece: &Piece, positions: &Vec<usize>) -> Vec<bool> {
    let (_id, content) = piece;
    positions.iter().map(|&pos| content[pos]).collect()
}

fn extract_flip(piece: &Piece, positions: &Vec<usize>) -> Vec<bool> {
    let mut v = extract(piece, positions);
    v.reverse();
    v
}

fn borders(piece: &Piece) -> Vec<Vec<bool>> {
    vec![
        extract(piece, &U),
        extract(piece, &L),
        extract(piece, &R),
        extract(piece, &D),
        extract_flip(piece, &U),
        extract_flip(piece, &L),
        extract_flip(piece, &R),
        extract_flip(piece, &D),
    ]
}
fn corners(pieces: &[Piece]) -> Vec<u64> {
    // Each border will have 1 match (it's on the border of the puzzle) or two matches (it's on the inside). Each piece appear: 12 times in the index (two unique borders that can be flipped, plus two borders that match and can be flipped), 14 times, or 16 times.
    let mut index: HashMap<Vec<bool>, Vec<u64>> = HashMap::new();
    for piece in pieces.iter() {
        for border in borders(piece).iter() {
            let entry = index.entry(border.to_vec()).or_insert(vec![]);
            entry.push(piece.0)
        }
    }

    let mut result: Vec<u64> = vec![];
    for piece in pieces.iter() {
        let mut n = 0;
        for border in borders(piece).iter() {
            n += index.get(border).unwrap().len();
        }
        if n == 12 {
            result.push((piece.0))
        }
    }

    result
}

fn assembly(pieces: &[Piece], corner_piece: u64) {}

pub fn run(filename: String) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let chunks: Vec<_> = contents.split("\n\n").collect();
    let mut tiles = Vec::new();
    for chunk in chunks.iter() {
        let lines: Vec<_> = chunk.split("\n").collect();
        let id = (scan_fmt!(lines[0], "Tile {d}:", u64)).unwrap();
        let tile: Vec<bool> = lines[1..].concat().chars().map(|c| c == '#').collect();
        tiles.push((id, tile))
    }

    let corners = corners(&tiles);
    println!("{}", corners.iter().product::<u64>())
}
