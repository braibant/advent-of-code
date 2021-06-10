use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

// In this problem, we need to fiddle with jigsaw pieces to construct a rectangular image, and find the ids of the corner pieces. We can observe that:
// - each tile that is a corner piece will have exactly two borders that are not matching any other borders;
// - each tile that is an edge piece will have one border that does not match any other border.

// So, my guess here is that we should simply: count the number of occurences of each border, extract the corner pieces based on this, and return the product of their id (the puzzle answer).

// The problem description uses the word "flipped", but I assume that we cannot actually flip the pieces vertically or horizontally.

type Piece = (u64, Vec<bool>);

const SIZE: usize = 10;

const IMAGE_SIZE: usize = 96;

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

fn get(i: usize, j: usize, piece: &Vec<bool>) -> bool {
    piece[i * SIZE + j]
}

fn rotate(piece: &Piece) -> Piece {
    let mut p = vec![];
    for i in 0..SIZE {
        for j in 0..SIZE {
            p.push(get(j, SIZE - i - 1, &piece.1))
        }
    }
    return (piece.0, p);
}

fn hflip(piece: &Piece) -> Piece {
    let mut p = vec![];
    for i in 0..SIZE {
        for j in 0..SIZE {
            p.push(get(i, SIZE - j - 1, &piece.1))
        }
    }
    return (piece.0, p);
}

fn vflip(piece: &Piece) -> Piece {
    let mut p = vec![];
    for i in 0..SIZE {
        for j in 0..SIZE {
            p.push(get(SIZE - i - 1, j, &piece.1))
        }
    }
    return (piece.0, p);
}

#[derive(Clone, Copy)]
enum Operation {
    Rotate,
    Vflip,
    Hflip,
}

fn apply_operation(op: Operation, piece: &Piece) -> Piece {
    match op {
        Operation::Rotate => rotate(piece),
        Operation::Vflip => vflip(piece),
        Operation::Hflip => hflip(piece),
    }
}

fn matching(piece: &Piece, positions: &Vec<usize>, border: &Vec<bool>) -> bool {
    let tile = &piece.1;
    for (i, &p) in positions.iter().enumerate() {
        if tile[p] != border[i] {
            return false;
        }
    }
    return true;
}

lazy_static! {
 // Iterating through OPS gives us all orientations
    static ref OPS: Vec<Operation> = {
        let mut ops = vec![];
        for i in 0..4 {
            ops.push(Operation::Rotate)
        }
        ops.push(Operation::Vflip);
        for i in 0..4 {
            ops.push(Operation::Rotate)
        }
        ops.push(Operation::Hflip);
        for i in 0..4 {
            ops.push(Operation::Rotate)
        }
        return ops;
    };
}

// Rotate, flip the piece until finding a matching orientation w.r.t. the given positions / border combination.
fn find_matching_orientation(
    piece: &Piece,
    positions: &Vec<usize>,
    border: &Vec<bool>,
) -> Option<Piece> {
    let mut piece = piece.clone();

    for &op in OPS.iter() {
        if matching(&piece, positions, border) {
            return Some(piece);
        } else {
            piece = apply_operation(op, &piece)
        }
    }
    return None;
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

type Index = HashMap<Vec<bool>, Vec<u64>>;

fn create_index(pieces: &[Piece]) -> Index {
    let mut index: Index = HashMap::new();
    for piece in pieces.iter() {
        for border in borders(piece).iter() {
            let entry = index.entry(border.to_vec()).or_insert(vec![]);
            entry.push(piece.0)
        }
    }

    return index;
}

fn cardinal(index: &Index, border: &Vec<bool>) -> usize {
    index.get(border).unwrap().len()
}

// Let's orientate one of the corner pieces so that it's upper left borders are unique. This does not need to consider flips (we will flip the whole image later, if needed.)
fn orientate_ul_corner(index: &Index, piece: &Piece) -> Piece {
    let mut piece = piece.clone();
    while cardinal(index, &extract(&piece, &U)) != 1 && cardinal(index, &extract(&piece, &L)) != 1 {
        piece = rotate(&piece)
    }
    return piece;
}

// Returns the four corner pieces
fn corners(index: &Index, pieces: &[Piece]) -> Vec<Piece> {
    let mut result: Vec<Piece> = vec![];
    // Each border will have 1 match (it's on the border of the puzzle) or two matches (it's on the inside). Each piece appear: 12 times in the index (two unique borders that can be flipped, plus two borders that match and can be flipped), 14 times, or 16 times.
    for piece in pieces.iter() {
        let mut n = 0;
        for border in borders(piece).iter() {
            n += index.get(border).unwrap().len();
        }
        if n == 12 {
            result.push((piece.clone()))
        }
    }

    result
}

// Returns the (unique) unused piece matching `border`, and marks it as used.
fn find_and_orient_relevant_unused_piece(
    index: &Index,
    pieces: &HashMap<u64, Piece>,
    used: &mut HashSet<u64>,
    border: &Vec<bool>,
    positions: &Vec<usize>,
) -> Piece {
    let matching_pieces: Vec<&u64> = index
        .get(border)
        .unwrap()
        .iter()
        .filter(|id| !used.contains(id))
        .collect();

    assert_eq!(matching_pieces.len(), 1);
    let piece_id = *matching_pieces[0];
    used.insert(piece_id);
    let piece = pieces.get(&piece_id).unwrap();
    return find_matching_orientation(piece, positions, border).unwrap();
}

// Render a matrix of tiles as a matrix of booleans, having removed the tile borders

fn extract_line(pieces: &Vec<Vec<Piece>>, i: usize) -> Vec<bool> {
    let line = &pieces[i / SIZE];

    let mut acc = vec![];
    for tile in line.iter() {
        for j in 0..SIZE {
            acc.push(get(i % SIZE, j, &tile.1))
        }
    }
    acc
}

// Remove the borders, i.e. indices which are 0 % SIZE or SIZE - 1 % SIZE.
fn remove_borders<T>(v: &Vec<T>) -> Vec<T>
where
    T: Clone,
{
    let mut acc = vec![];
    for (i, b) in v.iter().enumerate() {
        if !(i % SIZE == 0 || i % SIZE == SIZE - 1) {
            acc.push(b.clone())
        }
    }
    acc
}

fn render(pieces: &Vec<Vec<Piece>>) -> Vec<Vec<bool>> {
    let number_of_tiles = pieces.len();
    let mut lines = vec![];
    for i in 0..(number_of_tiles * SIZE - 1) {
        let line = remove_borders(&extract_line(pieces, i));
        lines.push(line);
    }
    return remove_borders(&lines);
}

// Build the puzzle, one piece at a time, starting from the UL corner. This assumes that the UL corner has been properly oriented.
fn assembly(index: &Index, pieces: &[Piece], ul_corner: &Piece) -> Vec<Vec<Piece>> {
    // First, let's build an index of pieces by piece id.
    let pieces: HashMap<u64, Piece> = {
        let mut acc: HashMap<u64, Piece> = HashMap::new();
        for p in pieces {
            acc.insert(p.0, p.clone());
        }
        acc
    };

    // We will keep track of the pieces that we have already used.
    let mut used: HashSet<u64> = HashSet::new();
    // This will force the first iteration of the loops below to pick the UL corner piece, in the relevant orientation.
    let mut top = extract(ul_corner, &U);

    let mut acc: Vec<Vec<Piece>> = Vec::new();
    for _row in 0..12 {
        let left_piece = find_and_orient_relevant_unused_piece(index, &pieces, &mut used, &top, &U);
        let mut row_acc = vec![left_piece.clone()];
        let mut fringe: Vec<bool> = extract(&left_piece, &R);
        for _col in 1..12 {
            let piece =
                find_and_orient_relevant_unused_piece(index, &pieces, &mut used, &fringe, &L);
            row_acc.push(piece.clone());
            fringe = extract(&piece, &R);
        }
        top = extract(&left_piece, &D);
        acc.push(row_acc);
    }

    acc
}

mod image {

    use super::*;
    type T = Vec<Vec<bool>>;

    fn width(t: &T) -> usize {
        t[0].len()
    }

    fn height(t: &T) -> usize {
        t.len()
    }

    fn present_at(image: &T, pattern: &T, i0: usize, j0: usize) -> bool {
        if width(pattern) + j0 < IMAGE_SIZE && height(pattern) + i0 < IMAGE_SIZE {
            for i in 0..height(pattern) {
                for j in 0..width(pattern) {
                    if pattern[i][j] && !image[i + i0][j + j0] {
                        return false;
                    }
                }
            }
            return true;
        } else {
            return false;
        }
    }

    fn occurences(image: &T, pattern: &T) -> Vec<(usize, usize)> {
        let mut acc = vec![];
        for i in 0..IMAGE_SIZE {
            for j in 0..IMAGE_SIZE {
                if present_at(image, pattern, i, j) {
                    acc.push((i, j))
                }
            }
        }
        return acc;
    }

    fn rotate(image: &T) -> T {
        let mut result = vec![];
        for i in 0..IMAGE_SIZE {
            let mut line = vec![];
            for j in 0..IMAGE_SIZE {
                line.push(image[j][IMAGE_SIZE - i - 1])
            }
            result.push(line)
        }
        return result;
    }

    fn hflip(image: &T) -> T {
        let mut result = vec![];
        for i in 0..IMAGE_SIZE {
            let mut line = vec![];
            for j in 0..IMAGE_SIZE {
                line.push(image[i][IMAGE_SIZE - j - 1]);
            }
            result.push(line);
        }
        return result;
    }

    fn vflip(image: &T) -> T {
        let mut result = vec![];
        for i in 0..IMAGE_SIZE {
            let mut line = vec![];
            for j in 0..IMAGE_SIZE {
                line.push(image[IMAGE_SIZE - i - 1][j]);
            }
            result.push(line);
        }
        return result;
    }

    fn apply_operation(op: Operation, image: &T) -> T {
        match op {
            Operation::Rotate => rotate(image),
            Operation::Vflip => vflip(image),
            Operation::Hflip => hflip(image),
        }
    }

    pub fn find_matching_orientation(image: &T, pattern: &T) -> Option<(T, Vec<(usize, usize)>)> {
        let mut image = image.clone();

        for &op in super::OPS.iter() {
            let occurences = occurences(&image, pattern);
            if occurences.len() != 0 {
                return Some((image, occurences));
            };
            image = apply_operation(op, &image);
        }
        return None;
    }

    pub fn remove_pattern(image: &mut T, pattern: &T, i0: usize, j0: usize) {
        for i in 0..height(pattern) {
            for j in 0..width(pattern) {
                if pattern[i][j] {
                    image[i + i0][j + j0] = false
                }
            }
        }
    }
}

fn parse_image(s: &str) -> Vec<Vec<bool>> {
    let lines: Vec<_> = s.split("\n").collect();

    let mut acc = vec![];
    for line in lines.iter() {
        let line: Vec<bool> = line.chars().map(|c| c == '#').collect();
        acc.push(line)
    }
    return acc;
}

pub fn run(filename: String) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let chunks: Vec<_> = contents.split("\n\n").collect();
    let mut pieces = Vec::new();
    for chunk in chunks.iter() {
        let lines: Vec<_> = chunk.split("\n").collect();
        let id = (scan_fmt!(lines[0], "Tile {d}:", u64)).unwrap();
        let tile: Vec<bool> = lines[1..].concat().chars().map(|c| c == '#').collect();
        pieces.push((id, tile))
    }

    let index = create_index(&pieces);
    let corners = corners(&index, &pieces);
    println!(
        "{:?}",
        corners.iter().map(|piece| piece.0).collect::<Vec<_>>()
    );

    println!("{}", corners.iter().map(|piece| piece.0).product::<u64>());

    let ul_corner = orientate_ul_corner(&index, &corners[0]);
    println!("{:?}", ul_corner.0);

    let assembly = assembly(&index, &pieces, &ul_corner);
    let mut image = render(&assembly);
    println!("{} {}", image.len(), image[0].len());

    let pattern = parse_image(
        "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ",
    );

    let (mut image, occurences) = image::find_matching_orientation(&image, &pattern).unwrap();

    for (i, j) in occurences.into_iter() {
        image::remove_pattern(&mut image, &pattern, i, j)
    }

    let mut part2 = 0;
    for i in 0..IMAGE_SIZE {
        for j in 0..IMAGE_SIZE {
            if image[i][j] {
                part2 += 1
            }
        }
    }
    println!("{}", part2)
}
