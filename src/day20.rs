use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

// In this problem, we need to fiddle with jigsaw pieces to construct a rectangular image, and find the ids of the corner pieces. We can observe that:
// - each tile that is a corner piece will have exactly two borders that are not matching any other borders;
// - each tile that is an edge piece will have one border that does not match any other border.

// So, my guess here is that we should simply: count the number of occurences of each border, extract the corner pieces based on this, and return the product of their id (the puzzle answer).

const TILE_SIZE: usize = 10;

const IMAGE_SIZE: usize = 96;

#[derive(Clone, Copy)]
pub enum Operation {
    Rotate,
    Vflip,
    Hflip,
}

mod image {

    use super::*;
    pub type T = Vec<Vec<bool>>;

    pub fn width(t: &T) -> usize {
        t[0].len()
    }

    pub fn height(t: &T) -> usize {
        t.len()
    }

    pub fn rotate(image: &T) -> T {
        assert_eq!(width(image), height(image));
        let size = width(image);
        let mut result = vec![];
        for i in 0..size {
            let mut line = vec![];
            for j in 0..size {
                line.push(image[j][size - i - 1])
            }
            result.push(line)
        }
        return result;
    }

    pub fn hflip(image: &T) -> T {
        let size = width(image);
        let mut result = vec![];
        for i in 0..height(image) {
            let mut line = vec![];
            for j in 0..size {
                line.push(image[i][size - j - 1]);
            }
            result.push(line);
        }
        return result;
    }

    pub fn vflip(image: &T) -> T {
        let size = height(image);
        let mut result = vec![];
        for i in 0..size {
            let mut line = vec![];
            for j in 0..width(image) {
                line.push(image[size - i - 1][j]);
            }
            result.push(line);
        }
        return result;
    }

    pub fn apply_operation(op: Operation, image: &T) -> T {
        match op {
            Operation::Rotate => rotate(image),
            Operation::Vflip => vflip(image),
            Operation::Hflip => hflip(image),
        }
    }

    fn present_at(image: &T, pattern: &T, i0: usize, j0: usize) -> bool {
        if width(pattern) + j0 < width(image) && height(pattern) + i0 < width(image) {
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
        for i in 0..height(image) {
            for j in 0..width(image) {
                if present_at(image, pattern, i, j) {
                    acc.push((i, j))
                }
            }
        }
        return acc;
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

    /// Parse a rectangular bitmap from a string.
    pub fn parse(lines: &[&str]) -> T {
        let mut acc = vec![];
        for line in lines.iter() {
            let line: Vec<bool> = line.chars().map(|c| c == '#').collect();
            acc.push(line)
        }
        return acc;
    }
}

type Piece = (u64, image::T);
type Pos = (usize, usize);

lazy_static! {
    static ref U: Vec<Pos> = {
        let mut v = Vec::new();
        for j in 0..TILE_SIZE {
            v.push((0, j))
        }
        v
    };
    static ref L: Vec<Pos> = {
        let mut v = Vec::new();
        for i in 0..TILE_SIZE {
            v.push((i, 0))
        }
        v
    };
    static ref R: Vec<Pos> = {
        let mut v = Vec::new();
        for i in 0..TILE_SIZE {
            v.push((i, TILE_SIZE - 1))
        }
        v
    };
    static ref D: Vec<Pos> = {
        let mut v = Vec::new();
        for j in 0..TILE_SIZE {
            v.push((TILE_SIZE - 1, j))
        }
        v
    };
}

fn extract(tile: &image::T, positions: &Vec<Pos>) -> Vec<bool> {
    positions.iter().map(|(i, j)| tile[*i][*j]).collect()
}

fn extract_flip(tile: &image::T, positions: &Vec<Pos>) -> Vec<bool> {
    let mut v = extract(tile, positions);
    v.reverse();
    v
}

fn matching(tile: &image::T, positions: &Vec<Pos>, border: &Vec<bool>) -> bool {
    for (idx, (i, j)) in positions.iter().enumerate() {
        if tile[*i][*j] != border[idx] {
            return false;
        }
    }
    return true;
}

lazy_static! {
 // Iterating through OPS gives us all orientations
    static ref OPS: Vec<Operation> = {
        let mut ops = vec![];
        for _i in 0..4 {
            ops.push(Operation::Rotate)
        }
        ops.push(Operation::Vflip);
        for _i in 0..4 {
            ops.push(Operation::Rotate)
        }
        ops.push(Operation::Hflip);
        for _i in 0..4 {
            ops.push(Operation::Rotate)
        }
        return ops;
    };
}

// Rotate, flip the piece until finding a matching orientation w.r.t. the given positions / border combination.
fn find_matching_orientation(
    piece: &Piece,
    positions: &Vec<Pos>,
    border: &Vec<bool>,
) -> Option<Piece> {
    let (tile_id, mut tile) = piece.clone();
    for &op in OPS.iter() {
        if matching(&tile, positions, border) {
            return Some((tile_id, tile));
        } else {
            tile = image::apply_operation(op, &tile)
        }
    }
    return None;
}

fn borders(piece: &Piece) -> Vec<Vec<bool>> {
    vec![
        extract(&piece.1, &U),
        extract(&piece.1, &L),
        extract(&piece.1, &R),
        extract(&piece.1, &D),
        extract_flip(&piece.1, &U),
        extract_flip(&piece.1, &L),
        extract_flip(&piece.1, &R),
        extract_flip(&piece.1, &D),
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
    let (tile_id, mut tile) = piece.clone();
    while cardinal(index, &extract(&tile, &U)) != 1 && cardinal(index, &extract(&tile, &L)) != 1 {
        tile = image::rotate(&tile)
    }
    return (tile_id, tile);
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
            result.push(piece.clone())
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
    positions: &Vec<Pos>,
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
    let line = &pieces[i / TILE_SIZE];

    let mut acc = vec![];
    for piece in line.iter() {
        let (_tile_id, tile) = piece;
        for j in 0..TILE_SIZE {
            acc.push(tile[i % TILE_SIZE][j])
        }
    }
    acc
}

// Remove the borders of the tiles, i.e. indices which are 0 % TILE_SIZE or TILE_SIZE - 1 % TILE_SIZE.
fn remove_borders<T>(v: &Vec<T>) -> Vec<T>
where
    T: Clone,
{
    let mut acc = vec![];
    for (i, b) in v.iter().enumerate() {
        if !(i % TILE_SIZE == 0 || i % TILE_SIZE == TILE_SIZE - 1) {
            acc.push(b.clone())
        }
    }
    acc
}

// Render the image as a bitmap, once it has been assembled.
fn render(pieces: &Vec<Vec<Piece>>) -> Vec<Vec<bool>> {
    let number_of_tiles = pieces.len();
    let mut lines = vec![];
    for i in 0..(number_of_tiles * TILE_SIZE - 1) {
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
    let mut top = extract(&ul_corner.1, &U);

    let mut acc: Vec<Vec<Piece>> = Vec::new();
    for _row in 0..12 {
        let left_piece: Piece =
            find_and_orient_relevant_unused_piece(index, &pieces, &mut used, &top, &U);
        let mut row_acc = vec![left_piece.clone()];
        let mut fringe: Vec<bool> = extract(&left_piece.1, &R);
        for _col in 1..12 {
            let piece =
                find_and_orient_relevant_unused_piece(index, &pieces, &mut used, &fringe, &L);
            row_acc.push(piece.clone());
            fringe = extract(&piece.1, &R);
        }
        top = extract(&left_piece.1, &D);
        acc.push(row_acc);
    }

    acc
}

pub fn run(filename: String) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let chunks: Vec<_> = contents.split("\n\n").collect();
    let mut pieces: Vec<Piece> = Vec::new();
    for chunk in chunks.iter() {
        let lines: Vec<_> = chunk.split("\n").collect();
        let id = (scan_fmt!(lines[0], "Tile {d}:", u64)).unwrap();
        let tile: image::T = image::parse(&lines[1..]);
        pieces.push((id, tile))
    }

    let index = create_index(&pieces);
    let corners = corners(&index, &pieces);
    println!(
        "Corners {:?}, product {}",
        corners.iter().map(|piece| piece.0).collect::<Vec<_>>(),
        corners.iter().map(|piece| piece.0).product::<u64>()
    );

    let ul_corner = orientate_ul_corner(&index, &corners[0]);
    let assembly = assembly(&index, &pieces, &ul_corner);
    let image = render(&assembly);
    println!("Final image size: {}x{}", image.len(), image[0].len());

    let pattern = image::parse(&vec![
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ]);

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
    println!("Part 2: {}", part2)
}
