use std::collections::{HashMap, HashSet, VecDeque};

const NORTH: [isize; 2] = [-1, 0];
const SOUTH: [isize; 2] = [1, 0];
const EAST: [isize; 2] = [0, 1];
const WEST: [isize; 2] = [0, -1];

type TileDirection = &'static [[isize; 2]];

const EVERY_DIRECTION: [[isize; 2]; 4] = [NORTH, SOUTH, EAST, WEST];

const DIRECTIONS: &[(char, TileDirection)] = &[
    ('S', &EVERY_DIRECTION),
    ('|', &[NORTH, SOUTH]),
    ('-', &[EAST, WEST]),
    ('L', &[NORTH, EAST]),
    ('J', &[NORTH, WEST]),
    ('7', &[SOUTH, WEST]),
    ('F', &[SOUTH, EAST]),
];

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    North,
    South,
    East,
    West,
}

impl Orientation {
    fn new_from_offset_and_tile(tile: char, offset: [isize; 2]) -> Self {
        let [x, y] = offset;

        match tile {
            '|' => {
                if x == -1 {
                    Self::North
                } else {
                    Self::South
                }
            }
            '-' => {
                if y == -1 {
                    Self::West
                } else {
                    Self::East
                }
            }
            'L' => {
                if x == 1 {
                    Self::East
                } else {
                    Self::North
                }
            }
            'J' => {
                if x == 1 {
                    Self::West
                } else {
                    Self::North
                }
            }
            '7' => {
                if x == -1 {
                    Self::West
                } else {
                    Self::South
                }
            }
            'F' => {
                if x == -1 {
                    Self::East
                } else {
                    Self::South
                }
            }
            _ => panic!("Nope {tile}"),
        }
    }

    fn get_next_orientation(&self, next_tile: char) -> Orientation {
        match self {
            Orientation::North => {
                if next_tile == 'F' {
                    return Self::East;
                } else if next_tile == '7' {
                    return Self::West;
                } else {
                    return *self;
                }
            }
            Orientation::South => {
                if next_tile == 'L' {
                    return Self::East;
                } else if next_tile == 'J' {
                    return Self::West;
                } else {
                    return *self;
                }
            }
            Orientation::East => {
                if next_tile == 'J' {
                    return Self::North;
                } else if next_tile == '7' {
                    return Self::South;
                } else {
                    return *self;
                }
            }
            Orientation::West => {
                if next_tile == 'L' {
                    return Self::North;
                } else if next_tile == 'F' {
                    return Self::South;
                } else {
                    return *self;
                }
            }
        }
    }

    // Don't care about overflow here
    fn get_left_and_right_neighbors(
        &self,
        x: usize,
        y: usize,
        tile: char,
    ) -> (Vec<[usize; 2]>, Vec<[usize; 2]>) {
        let mut lefts = vec![];
        let mut rights = vec![];
        match self {
            Orientation::North => match tile {
                'L' => {
                    lefts.push([x, y - 1]);
                }
                'J' => {
                    rights.push([x, y + 1]);
                }
                '|' => {
                    lefts.push([x, y - 1]);
                    rights.push([x, y + 1]);
                }
                _ => {
                    panic!("{self:?} | {tile}")
                }
            },
            Orientation::South => match tile {
                'F' => {
                    rights.push([x, y - 1]);
                }
                '7' => {
                    lefts.push([x, y + 1]);
                }
                '|' => {
                    lefts.push([x, y + 1]);
                    rights.push([x, y - 1]);
                }
                _ => {
                    panic!("{self:?} | {tile}")
                }
            },
            Orientation::East => match tile {
                'L' => {
                    lefts.push([x - 1, y]);
                }
                'F' => {
                    rights.push([x + 1, y]);
                }
                '-' => {
                    lefts.push([x - 1, y]);
                    rights.push([x + 1, y]);
                }
                _ => {
                    panic!("{self:?} | {tile}")
                }
            },
            Orientation::West => match tile {
                '7' => {
                    lefts.push([x + 1, y]);
                }
                'J' => {
                    rights.push([x - 1, y]);
                }
                '-' => {
                    lefts.push([x + 1, y]);
                    rights.push([x - 1, y]);
                }
                _ => {
                    panic!("{self:?} | {tile}")
                }
            },
        }

        (lefts, rights)
    }
}

fn tile_directions(tile: &char) -> Option<TileDirection> {
    DIRECTIONS.iter().copied().find_map(|(pipe, directions)| {
        if tile == &pipe {
            Some(directions)
        } else {
            None
        }
    })
}

fn solve1() -> usize {
    let input = include_str!("../input.txt");

    let mut drawing = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (s_x, s_y) = drawing
        .iter()
        .enumerate()
        .find_map(|(idx, line)| Some((idx, line.iter().position(|&c| c == 'S')?)))
        .unwrap();

    let mut queue = VecDeque::new();

    queue.push_back(([s_x, s_y], tile_directions(&drawing[s_x][s_y]).unwrap()));
    drawing[s_x][s_y] = 'V';
    let mut depth = 0;

    while !queue.is_empty() {
        let len = queue.len();
        for _ in 0..len {
            let ([pos_x, pos_y], directions) = queue.pop_front().unwrap();
            drawing[pos_x][pos_y] = 'V';

            for [offset_x, offset_y] in directions {
                let x = (pos_x as isize + offset_x) as usize;
                let y = (pos_y as isize + offset_y) as usize;
                if let Some(line) = drawing.get_mut(x) {
                    if let Some(tile) = line.get_mut(y) {
                        if let Some(directions) = tile_directions(tile) {
                            if directions.iter().any(|[d_x, d_y]| {
                                ((x as isize + d_x) as usize) == pos_x
                                    && ((y as isize + d_y) as usize) == pos_y
                            }) {
                                if queue.iter().any(|([q_x, q_y], _)| *q_x == x && *q_y == y) {
                                    return depth + 1;
                                }
                                queue.push_back(([x, y], directions));
                            }
                        }
                    }
                }
            }
        }
        depth += 1;
    }

    panic!("Should have found a depth");
}

fn solve2() -> usize {
    let input = include_str!("../input.txt");

    let mut drawing = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let original_drawing = drawing.to_vec();

    let (s_x, s_y) = drawing
        .iter()
        .enumerate()
        .find_map(|(idx, line)| Some((idx, line.iter().position(|&c| c == 'S')?)))
        .unwrap();

    let mut queue = VecDeque::new();
    let starting_tag = '9';

    queue.push_back((
        [s_x, s_y],
        tile_directions(&drawing[s_x][s_y]).unwrap(),
        starting_tag,
    ));

    let mut left_tiles = vec![];
    let mut right_tiles = vec![];

    let mut tag_to_left_and_right: HashMap<char, (Vec<[usize; 2]>, Vec<[usize; 2]>)> =
        HashMap::new();

    let mut tag_to_orientation: HashMap<char, Orientation> = HashMap::new();

    let tags = 'outer: loop {
        let len = queue.len();

        for i in 0..len {
            let mut curr_tag = char::from_digit(i as u32, 10).unwrap();
            let ([pos_x, pos_y], directions, tag) = queue.pop_front().unwrap();

            if let Some(orientation) = tag_to_orientation.get(&tag) {
                let (lefts, rights) =
                    orientation.get_left_and_right_neighbors(pos_x, pos_y, drawing[pos_x][pos_y]);

                let (curr_lefts, curr_rights) = tag_to_left_and_right
                    .entry(tag)
                    .or_insert((Vec::new(), Vec::new()));

                for [left_x, left_y] in lefts {
                    if left_x < drawing.len() && left_y < drawing[left_x].len() {
                        curr_lefts.push([left_x, left_y]);

                        if tag == '1' {
                            left_tiles.push([left_x, left_y]);
                        } else {
                            right_tiles.push([left_x, left_y]);
                        }
                    }
                }
                for [right_x, right_y] in rights {
                    if right_x < drawing.len() && right_y < drawing[right_x].len() {
                        curr_rights.push([right_x, right_y]);
                        if tag == '1' {
                            right_tiles.push([right_x, right_y]);
                        } else {
                            left_tiles.push([right_x, right_y]);
                        }
                    }
                }
            }

            drawing[pos_x][pos_y] = tag;

            for [offset_x, offset_y] in directions {
                let x = (pos_x as isize + offset_x) as usize;
                let y = (pos_y as isize + offset_y) as usize;
                if let Some(line) = drawing.get_mut(x) {
                    if let Some(tile) = line.get_mut(y) {
                        if let Some(directions) = tile_directions(tile) {
                            if directions.iter().any(|[d_x, d_y]| {
                                ((x as isize + d_x) as usize) == pos_x
                                    && ((y as isize + d_y) as usize) == pos_y
                            }) {
                                if let Some((_, _, q_tag)) = queue
                                    .iter()
                                    .find(|([q_x, q_y], _, _)| *q_x == x && *q_y == y)
                                {
                                    drawing[x][y] = tag;
                                    drawing[s_x][s_y] = tag;
                                    break 'outer [tag, *q_tag];
                                }
                                let tag = if tag == starting_tag {
                                    curr_tag =
                                        char::from_digit(curr_tag.to_digit(10).unwrap() + 1, 10)
                                            .unwrap();

                                    tag_to_orientation.insert(
                                        curr_tag,
                                        Orientation::new_from_offset_and_tile(
                                            *tile,
                                            [*offset_x, *offset_y],
                                        ),
                                    );

                                    curr_tag
                                } else {
                                    tag_to_orientation.entry(tag).and_modify(|entry| {
                                        *entry = entry.get_next_orientation(*tile)
                                    });

                                    tag
                                };
                                queue.push_back(([x, y], directions, tag));
                            }
                        }
                    }
                }
            }
        }
    };

    let mut left_tiles: Vec<[usize; 2]> = vec![];
    let mut right_tiles: Vec<[usize; 2]> = vec![];

    let mut tick = true;

    for tag in tags {
        let (left, right) = tag_to_left_and_right.get(&tag).unwrap();

        if tick {
            left_tiles.extend(left);
            right_tiles.extend(right);
        } else {
            right_tiles.extend(left);
            left_tiles.extend(right);
        }
        tick = !tick;
    }

    dbg!(&tags);
    // println!("{left_tiles:?}");
    println!("len: {}", left_tiles.len());
    // println!("{right_tiles:?}");
    println!("len: {}", right_tiles.len());

    let left_tiles = left_tiles
        .into_iter()
        .filter(|[x, y]| !tags.contains(&drawing[*x][*y]))
        .collect::<HashSet<_>>();

    let right_tiles = right_tiles
        .into_iter()
        .filter(|[x, y]| !tags.contains(&drawing[*x][*y]))
        .collect::<HashSet<_>>();

    // println!("{left_tiles:?}");
    println!("len: {}", left_tiles.len());
    // println!("{right_tiles:?}");
    println!("len: {}", right_tiles.len());

    if right_tiles.len() < left_tiles.len() {
        for [x, y] in right_tiles {
            drawing[x][y] = 'x';
        }
    } else {
        for [x, y] in left_tiles {
            drawing[x][y] = 'x';
        }
    };

    for line in &drawing {
        println!("{line:?}");
    }
    let mut count = 0;

    let drawing_len = drawing.len();
    for x in 0..drawing_len {
        for y in 0..drawing[x].len() {
            let curr_char = drawing[x][y];

            if curr_char == 'x' && !tags.contains(&curr_char) {
                let mut queue = VecDeque::new();

                queue.push_back([x, y]);
                drawing[x][y] = 'V';
                count += 1;

                while !queue.is_empty() {
                    let len = queue.len();

                    for _ in 0..len {
                        let [x, y] = queue.pop_front().unwrap();

                        for [offset_x, offset_y] in EVERY_DIRECTION {
                            let x = (x as isize + offset_x) as usize;
                            let y = (y as isize + offset_y) as usize;

                            if let Some(line) = drawing.get_mut(x) {
                                if let Some(tile) = line.get_mut(y) {
                                    if *tile != 'V' && !tags.contains(tile) {
                                        queue.push_back([x, y]);
                                        *tile = 'V';

                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("============================");

    for line in &drawing {
        println!("{line:?}");
    }

    count
}

fn main() {
    dbg!(solve1());
    dbg!(solve2());
}
