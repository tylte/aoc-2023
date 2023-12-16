use std::collections::{HashSet, VecDeque};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn next_pos(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::Up => (x - 1, y),
            Direction::Down => (x + 1, y),
            Direction::Left => (x, y - 1),
            Direction::Right => (x, y + 1),
        }
    }

    fn next_direction(&self, c: char) -> Vec<Self> {
        match c {
            '.' => return vec![*self],
            '/' => match self {
                Direction::Up => vec![Self::Right],
                Direction::Down => vec![Self::Left],
                Direction::Left => vec![Self::Down],
                Direction::Right => vec![Self::Up],
            },
            '\\' => match self {
                Direction::Up => vec![Self::Left],
                Direction::Down => vec![Self::Right],
                Direction::Left => vec![Self::Up],
                Direction::Right => vec![Self::Down],
            },
            '-' => match self {
                Direction::Up | Direction::Down => vec![Self::Left, Self::Right],
                _ => vec![*self],
            },
            '|' => match self {
                Direction::Left | Direction::Right => vec![Self::Up, Self::Down],
                _ => vec![*self],
            },
            _ => unreachable!("Unknown symbol {c}"),
        }
    }
}

fn propagate_light(
    contraption: &[Vec<char>],
    start_x: usize,
    start_y: usize,
    dir: Direction,
) -> usize {
    let mut visited_with_direction: HashSet<(usize, usize, Direction)> = HashSet::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut q = VecDeque::new();

    q.push_back((start_x, start_y, dir));

    while !q.is_empty() {
        let len = q.len();

        for _ in 0..len {
            let (x, y, direction) = q.pop_front().unwrap();
            visited_with_direction.insert((x, y, direction));
            visited.insert((x, y));

            let next_char = contraption[x][y];
            for new_direction in direction.next_direction(next_char) {
                let (next_x, next_y) = new_direction.next_pos(x, y);
                if next_x < contraption.len()
                    && next_y < contraption[next_x].len()
                    && !visited_with_direction.contains(&(next_x, next_y, new_direction))
                {
                    q.push_back((next_x, next_y, new_direction));
                }
            }
        }
    }

    visited.len()
}

fn solve1() -> usize {
    let contraption = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    propagate_light(&contraption, 0, 0, Direction::Right)
}

fn solve2() -> usize {
    let contraption = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (length, width) = (contraption.len(), contraption[0].len());

    let mut max_energy = 0;
    for i in 0..length {
        max_energy = max_energy.max(propagate_light(&contraption, i, 0, Direction::Right));
        max_energy = max_energy.max(propagate_light(&contraption, i, width - 1, Direction::Left));
    }
    for i in 0..width {
        max_energy = max_energy.max(propagate_light(&contraption, 0, i, Direction::Down));
        max_energy = max_energy.max(propagate_light(&contraption, length - 1, i, Direction::Up));
    }
    max_energy
}

fn main() {
    dbg!(solve1());
    dbg!(solve2());
}
