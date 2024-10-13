use std::collections::{HashMap, VecDeque};

const INPUT_TEST: &str = include_str!("../input.test.txt");
const INPUT_REAL: &str = include_str!("../input.real.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Right,
    Left,
    Top,
    Down,
}

impl Dir {
    fn dir_offset(&self) -> [isize; 2] {
        match self {
            Dir::Right => [0, 1],
            Dir::Left => [0, -1],
            Dir::Top => [-1, 0],
            Dir::Down => [1, 0],
        }
    }

    fn next_offsets(&self, can_move_forward: bool, can_turn: bool) -> Vec<([isize; 2], Self)> {
        let mut offsets = vec![];

        if can_turn {
            match self {
                Dir::Right | Dir::Left => {
                    offsets.push((Self::Down.dir_offset(), Self::Down));
                    offsets.push((Self::Top.dir_offset(), Self::Top));
                }
                Dir::Top | Dir::Down => {
                    offsets.push((Self::Right.dir_offset(), Self::Right));
                    offsets.push((Self::Left.dir_offset(), Self::Left));
                }
            }
        }
        if can_move_forward {
            offsets.push((self.dir_offset(), *self));
        }

        offsets
    }

    fn next_pos(
        &self,
        grid: &[Vec<usize>],
        x: usize,
        y: usize,
        can_move_forward: bool,
        can_turn: bool,
    ) -> Vec<([usize; 2], Self)> {
        let offsets = self.next_offsets(can_move_forward, can_turn);

        offsets
            .into_iter()
            .filter_map(|([offset_x, offset_y], dir)| {
                let x = x.checked_add_signed(offset_x)?;
                let y = y.checked_add_signed(offset_y)?;

                if x < grid.len() && y < grid[x].len() {
                    Some(([x, y], dir))
                } else {
                    None
                }
            })
            .collect()
    }
}

fn q1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut solution = usize::MAX;

    let mut visited: HashMap<([usize; 2], Dir), Vec<(usize, usize)>> = HashMap::new();

    let mut q = VecDeque::new();

    q.push_back(([1, 0], Dir::Down, 1, grid[1][0]));
    q.push_back(([0, 1], Dir::Right, 1, grid[0][1]));

    while !q.is_empty() {
        for _ in 0..q.len() {
            let ([x, y], dir, consecutive_straight, heat) = q.pop_front().unwrap();

            for ([next_x, next_y], next_dir) in
                dir.next_pos(&grid, x, y, consecutive_straight < 3, true)
            {
                let next_heat = heat + grid[next_x][next_y];
                let next_consecutive_straight = if next_dir == dir {
                    consecutive_straight + 1
                } else {
                    1
                };

                if next_heat > solution {
                    // Worse than already found path
                    continue;
                }

                let key = ([next_x, next_y], next_dir);
                let entry = visited.entry(key).or_default();

                if entry.iter().any(|(heat, consecutive)| {
                    *heat <= next_heat && *consecutive <= next_consecutive_straight
                }) {
                    continue;
                }

                entry.push((next_heat, next_consecutive_straight));

                if next_x == grid.len() - 1 && next_y == grid[next_x].len() - 1 {
                    // Could be solution
                    if next_heat < solution {
                        solution = next_heat;
                    }
                } else {
                    // Continue the road
                    q.push_back((
                        [next_x, next_y],
                        next_dir,
                        next_consecutive_straight,
                        next_heat,
                    ));
                }
            }
        }
    }

    solution
}

fn q2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut solution = usize::MAX;

    let mut visited: HashMap<([usize; 2], Dir), Vec<(usize, usize)>> = HashMap::new();

    let mut q = VecDeque::new();

    q.push_back(([1, 0], Dir::Down, 1, grid[1][0]));
    q.push_back(([0, 1], Dir::Right, 1, grid[0][1]));

    while !q.is_empty() {
        for _ in 0..q.len() {
            let ([x, y], dir, consecutive_straight, heat) = q.pop_front().unwrap();

            let can_turn = consecutive_straight >= 4;
            let can_move_forward = consecutive_straight <= 10;

            for ([next_x, next_y], next_dir) in
                dir.next_pos(&grid, x, y, can_move_forward, can_turn)
            {
                let next_heat = heat + grid[next_x][next_y];
                let next_consecutive_straight = if next_dir == dir {
                    consecutive_straight + 1
                } else {
                    1
                };

                if next_heat > solution {
                    // Worse than already found path
                    continue;
                }

                let key = ([next_x, next_y], next_dir);
                let entry = visited.entry(key).or_default();

                if entry.iter().any(|(heat, consecutive)| {
                    *heat <= next_heat && *consecutive <= next_consecutive_straight
                }) {
                    continue;
                }

                entry.push((next_heat, next_consecutive_straight));

                if next_x == grid.len() - 1 && next_y == grid[next_x].len() - 1 {
                    // Could be solution
                    if next_heat < solution {
                        solution = next_heat;
                    }
                } else {
                    // Continue the road
                    q.push_back((
                        [next_x, next_y],
                        next_dir,
                        next_consecutive_straight,
                        next_heat,
                    ));
                }
            }
        }
    }

    solution
}
fn main() {
    dbg!(q1(INPUT_TEST));
    dbg!(q1(INPUT_REAL));

    dbg!(q2(INPUT_TEST));
    dbg!(q2(INPUT_REAL));
}
