use std::{
    collections::{BTreeMap, HashMap},
    convert::{TryFrom, TryInto},
    fmt,
    iter::FromIterator,
};

use aoc_runner_derive::*;

type Id = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum Orientation {
    None = 0,
    Rotate90 = 1,
    Rotate180 = 2,
    Rotate270 = 3,
    Flip0 = 4,
    Flip90 = 5,
    Flip180 = 6,
    Flip270 = 7,
}

impl Orientation {
    pub fn flip_180(self) -> Self {
        match self {
            Orientation::None => Orientation::Flip180,
            Orientation::Rotate90 => Orientation::Flip90,
            Orientation::Rotate180 => Orientation::Flip0,
            Orientation::Rotate270 => Orientation::Flip270,
            Orientation::Flip0 => Orientation::Rotate180,
            Orientation::Flip90 => Orientation::Rotate90,
            Orientation::Flip180 => Orientation::None,
            Orientation::Flip270 => Orientation::Rotate270,
        }
    }

    pub fn flip_270(self) -> Self {
        match self {
            Orientation::None => Orientation::Flip270,
            Orientation::Rotate90 => Orientation::Flip180,
            Orientation::Rotate180 => Orientation::Flip90,
            Orientation::Rotate270 => Orientation::Flip0,
            Orientation::Flip0 => Orientation::Rotate270,
            Orientation::Flip90 => Orientation::Rotate180,
            Orientation::Flip180 => Orientation::Rotate90,
            Orientation::Flip270 => Orientation::None,
        }
    }

    pub fn rotate_270(self) -> Self {
        match self {
            Orientation::None => Orientation::Rotate270,
            Orientation::Rotate90 => Orientation::None,
            Orientation::Rotate180 => Orientation::Rotate90,
            Orientation::Rotate270 => Orientation::Rotate180,
            Orientation::Flip0 => Orientation::Flip270,
            Orientation::Flip90 => Orientation::Flip0,
            Orientation::Flip180 => Orientation::Flip90,
            Orientation::Flip270 => Orientation::Flip180,
        }
    }
}

impl TryFrom<usize> for Orientation {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Orientation::None,
            1 => Orientation::Rotate90,
            2 => Orientation::Rotate180,
            3 => Orientation::Rotate270,
            4 => Orientation::Flip0,
            5 => Orientation::Flip90,
            6 => Orientation::Flip180,
            7 => Orientation::Flip270,
            _ => return Err(()),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Image {
    id: Id,
    data: [[bool; 10]; 10],
    edges: [u16; 8],
}

impl Image {
    pub fn new(id: Id, data: [[bool; 10]; 10]) -> Self {
        let top = data[0]
            .iter()
            .fold(0, |acc, &b| (acc << 1) | if b { 1 } else { 0 });
        let right = data
            .iter()
            .map(|a| a[9])
            .fold(0, |acc, b| (acc << 1) | if b { 1 } else { 0 });
        let bottom = data[9]
            .iter()
            .fold(0, |acc, &b| (acc << 1) | if b { 1 } else { 0 });
        let left = data
            .iter()
            .map(|a| a[0])
            .fold(0, |acc, b| (acc << 1) | if b { 1 } else { 0 });
        let top_flipped = data[0]
            .iter()
            .rev()
            .fold(0, |acc, &b| (acc << 1) | if b { 1 } else { 0 });
        let right_flipped = data
            .iter()
            .rev()
            .map(|a| a[9])
            .fold(0, |acc, b| (acc << 1) | if b { 1 } else { 0 });
        let bottom_flipped = data[9]
            .iter()
            .rev()
            .fold(0, |acc, &b| (acc << 1) | if b { 1 } else { 0 });
        let left_flipped = data
            .iter()
            .rev()
            .map(|a| a[0])
            .fold(0, |acc, b| (acc << 1) | if b { 1 } else { 0 });
        Self {
            id,
            data,
            edges: [
                top,
                left_flipped,
                bottom_flipped,
                right,
                top_flipped,
                right_flipped,
                bottom,
                left,
            ],
        }
    }

    pub fn oriented(&self, orientation: Orientation) -> [[bool; 8]; 8] {
        let mut oriented = [[false; 8]; 8];
        for (i, row) in oriented.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                let (x, y) = match orientation {
                    Orientation::None => (i, j),
                    Orientation::Rotate90 => (7 - j, i),
                    Orientation::Rotate180 => (7 - i, 7 - j),
                    Orientation::Rotate270 => (j, 7 - i),
                    Orientation::Flip0 => (i, 7 - j),
                    Orientation::Flip90 => (7 - j, 7 - i),
                    Orientation::Flip180 => (7 - i, j),
                    Orientation::Flip270 => (j, i),
                };
                *cell = self.data[x + 1][y + 1];
            }
        }
        oriented
    }
}

pub struct Oriented {
    image: Image,
    orientation: Orientation,
}

impl<'a> fmt::Debug for Oriented {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..10 {
            for j in 0..10 {
                let (row, col) = match self.orientation {
                    Orientation::None => (i, j),
                    Orientation::Rotate90 => (9 - j, i),
                    Orientation::Rotate180 => (9 - i, 9 - j),
                    Orientation::Rotate270 => (j, 9 - i),
                    Orientation::Flip0 => (i, 9 - j),
                    Orientation::Flip90 => (9 - j, 9 - i),
                    Orientation::Flip180 => (9 - i, j),
                    Orientation::Flip270 => (j, i),
                };
                f.write_str(if self.image.data[row][col] { "#" } else { "." })?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct Images {
    id_lookup: BTreeMap<Id, Image>,
    edge_lookup: HashMap<u16, Vec<(Id, Orientation)>>,
}

impl Images {
    pub fn images(&self) -> impl Iterator<Item = &Image> {
        self.id_lookup.values()
    }

    pub fn image_by_id(&self, id: Id) -> Image {
        self.id_lookup[&id]
    }

    pub fn images_by_edge(&self, edge: u16) -> &[(Id, Orientation)] {
        &self.edge_lookup[&edge]
    }
}

impl FromIterator<Image> for Images {
    fn from_iter<T: IntoIterator<Item = Image>>(iter: T) -> Self {
        let mut id_lookup = BTreeMap::new();
        let mut edge_lookup = HashMap::new();
        for image in iter {
            let id = image.id;
            id_lookup.insert(id, image);
            for (i, edge) in image.edges.iter().enumerate() {
                let edges: &mut Vec<_> = edge_lookup.entry(*edge).or_default();
                edges.push((id, i.try_into().unwrap()));
            }
        }
        Self {
            id_lookup,
            edge_lookup,
        }
    }
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Images {
    input
        .split("\n\n")
        .map(|s| {
            let mut lines = s.lines();
            let id = lines
                .next()
                .unwrap()
                .strip_prefix("Tile ")
                .unwrap()
                .strip_suffix(":")
                .unwrap()
                .parse()
                .unwrap();
            let mut image = [[false; 10]; 10];
            for (row, line) in lines.enumerate() {
                for (col, pixel) in line.chars().enumerate() {
                    image[row][col] = match pixel {
                        '#' => true,
                        '.' => false,
                        _ => panic!("Unexpected character"),
                    };
                }
            }
            Image::new(id, image)
        })
        .collect()
}

#[aoc(day20, part1)]
pub fn day20_part1(input: &Images) -> u64 {
    let corners: Vec<_> = input
        .images()
        .filter(|image| {
            image
                .edges
                .iter()
                .flat_map(|&edge| input.images_by_edge(edge))
                .count()
                == 12
        })
        .collect();
    assert_eq!(4, corners.len());
    corners.into_iter().map(|image| image.id).product()
}

#[aoc(day20, part2)]
pub fn day20_part2(input: &Images) -> usize {
    let corner = input
        .images()
        .find(|image| {
            image
                .edges
                .iter()
                .flat_map(|&edge| input.images_by_edge(edge))
                .count()
                == 12
        })
        .unwrap();
    let edges: Vec<usize> = corner.edges[..4]
        .iter()
        .enumerate()
        .filter(|(_, &edge)| input.images_by_edge(edge).len() > 1)
        .map(|(i, _)| i)
        .collect();
    let orientation = match edges.as_slice() {
        [0, 1] => Orientation::Rotate180,
        [1, 2] => Orientation::Rotate270,
        [2, 3] => Orientation::None,
        [0, 3] => Orientation::Rotate90,
        edges => panic!("Cannot orient to {:?}", edges),
    };
    let edge_length = (input.images().count() as f64).sqrt() as usize;
    let mut puzzle = vec![vec![None; edge_length]; edge_length];
    puzzle[0][0] = Some((corner.id, orientation));
    for row in 0..edge_length {
        if row != 0 {
            let (prev_piece, pp_orientation) = puzzle[row - 1][0].unwrap();
            let edge = input.image_by_id(prev_piece).edges[pp_orientation.flip_180() as usize];
            let (piece, orientation) = input
                .images_by_edge(edge)
                .iter()
                .find(|(id, _)| *id != prev_piece)
                .unwrap();
            puzzle[row][0] = Some((*piece, *orientation));
        }
        for col in 1..edge_length {
            let (prev_piece, pp_orientation) = puzzle[row][col - 1].unwrap();
            let edge = input.image_by_id(prev_piece).edges[pp_orientation.rotate_270() as usize];
            let (piece, orientation) = input
                .images_by_edge(edge)
                .iter()
                .find(|(id, _)| *id != prev_piece)
                .unwrap();
            puzzle[row][col] = Some((*piece, orientation.flip_270()));
        }
    }
    let mut picture: Vec<Vec<_>> = Vec::new();
    for row in puzzle {
        let row: Vec<_> = row
            .into_iter()
            .map(|image| {
                let (id, orientation) = image.unwrap();
                input.image_by_id(id).oriented(orientation)
            })
            .collect();
        for i in 0..8 {
            picture.push(
                row.iter()
                    .flat_map(|data| data[i].iter().copied())
                    .collect(),
            );
        }
    }
    let picture_size = picture.len();
    let dragon: [Vec<bool>; 3] = [
        b"                  # "
            .iter()
            .map(|&byte| byte == b'#')
            .collect(),
        b"#    ##    ##    ###"
            .iter()
            .map(|&byte| byte == b'#')
            .collect(),
        b" #  #  #  #  #  #   "
            .iter()
            .map(|&byte| byte == b'#')
            .collect(),
    ];
    let dragon: Vec<_> = dragon
        .iter()
        .enumerate()
        .flat_map(|(x, row)| row.iter().enumerate().map(move |(y, &b)| (x, y, b)))
        .collect();
    let indexors: [Box<dyn Fn(usize, usize) -> (usize, usize)>; 8] = [
        Box::new(|i, j| (i, j)),
        Box::new(|i, j| (j, i)),
        Box::new(|i, j| (i, picture_size - 1 - j)),
        Box::new(|i, j| (j, picture_size - 1 - i)),
        Box::new(|i, j| (picture_size - 1 - i, j)),
        Box::new(|i, j| (picture_size - 1 - j, i)),
        Box::new(|i, j| (picture_size - 1 - i, picture_size - 1 - j)),
        Box::new(|i, j| (picture_size - 1 - j, picture_size - 1 - i)),
    ];
    for indexor in &indexors {
        let mut found_dragon = false;
        for i in 0..=picture_size - 3 {
            for j in 0..=picture_size - 20 {
                let is_dragon = dragon.iter().all(|&(x, y, b)| {
                    let (p, q) = indexor(i + x, j + y);
                    !b || picture[p][q]
                });
                if is_dragon {
                    found_dragon = true;
                    for &(x, y, b) in &dragon {
                        if b {
                            let (p, q) = indexor(i + x, j + y);
                            picture[p][q] = false;
                        }
                    }
                }
            }
        }
        if found_dragon {
            return picture.into_iter().flatten().filter(|&b| b).count();
        }
    }
    panic!("No dragons!");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
        Tile 2311:\n\
        ..##.#..#.\n\
        ##..#.....\n\
        #...##..#.\n\
        ####.#...#\n\
        ##.##.###.\n\
        ##...#.###\n\
        .#.#.#..##\n\
        ..#....#..\n\
        ###...#.#.\n\
        ..###..###\n\
        \n\
        Tile 1951:\n\
        #.##...##.\n\
        #.####...#\n\
        .....#..##\n\
        #...######\n\
        .##.#....#\n\
        .###.#####\n\
        ###.##.##.\n\
        .###....#.\n\
        ..#.#..#.#\n\
        #...##.#..\n\
        \n\
        Tile 1171:\n\
        ####...##.\n\
        #..##.#..#\n\
        ##.#..#.#.\n\
        .###.####.\n\
        ..###.####\n\
        .##....##.\n\
        .#...####.\n\
        #.##.####.\n\
        ####..#...\n\
        .....##...\n\
        \n\
        Tile 1427:\n\
        ###.##.#..\n\
        .#..#.##..\n\
        .#.##.#..#\n\
        #.#.#.##.#\n\
        ....#...##\n\
        ...##..##.\n\
        ...#.#####\n\
        .#.####.#.\n\
        ..#..###.#\n\
        ..##.#..#.\n\
        \n\
        Tile 1489:\n\
        ##.#.#....\n\
        ..##...#..\n\
        .##..##...\n\
        ..#...#...\n\
        #####...#.\n\
        #..#.#.#.#\n\
        ...#.#.#..\n\
        ##.#...##.\n\
        ..##.##.##\n\
        ###.##.#..\n\
        \n\
        Tile 2473:\n\
        #....####.\n\
        #..#.##...\n\
        #.##..#...\n\
        ######.#.#\n\
        .#...#.#.#\n\
        .#########\n\
        .###.#..#.\n\
        ########.#\n\
        ##...##.#.\n\
        ..###.#.#.\n\
        \n\
        Tile 2971:\n\
        ..#.#....#\n\
        #...###...\n\
        #.#.###...\n\
        ##.##..#..\n\
        .#####..##\n\
        .#..####.#\n\
        #..#.#..#.\n\
        ..####.###\n\
        ..#.#.###.\n\
        ...#.#.#.#\n\
        \n\
        Tile 2729:\n\
        ...#.#.#.#\n\
        ####.#....\n\
        ..#.#.....\n\
        ....#..#.#\n\
        .##..##.#.\n\
        .#.####...\n\
        ####.#.#..\n\
        ##.####...\n\
        ##..#.##..\n\
        #.##...##.\n\
        \n\
        Tile 3079:\n\
        #.#.#####.\n\
        .#..######\n\
        ..#.......\n\
        ######....\n\
        ####.#..#.\n\
        .#...#.##.\n\
        #.#####.##\n\
        ..#.###...\n\
        ..#.......\n\
        ..#.###...\n\
    ";

    #[test]
    fn parser() {
        let images = input_generator(TEST_INPUT);
        assert_eq!(
            "\
            #.#.#####.\n\
            .#..######\n\
            ..#.......\n\
            ######....\n\
            ####.#..#.\n\
            .#...#.##.\n\
            #.#####.##\n\
            ..#.###...\n\
            ..#.......\n\
            ..#.###...\n\
            ",
            format!(
                "{:?}",
                Oriented {
                    image: images.image_by_id(3079),
                    orientation: Orientation::None
                }
            )
        )
    }

    #[test]
    fn print_rotated() {
        let images = input_generator(TEST_INPUT);
        assert_eq!(
            "\
            ...#.##..#\n\
            ....###.#.\n\
            ####.###.#\n\
            ...#.##...\n\
            #.##..#.##\n\
            #.#####.##\n\
            #.##....##\n\
            ....#...##\n\
            ...###..##\n\
            ...#....#.\n\
            ",
            format!(
                "{:?}",
                Oriented {
                    image: images.image_by_id(3079),
                    orientation: Orientation::Rotate90
                }
            )
        )
    }

    #[test]
    fn dragons() {
        let images = input_generator(TEST_INPUT);
        assert_eq!(273, day20_part2(&images));
    }
}
