const OFFSETS: &[[isize; 2]] = &[
    [1, -1],
    [1, 0],
    [1, 1],
    [0, -1],
    [0, 1],
    [-1, -1],
    [-1, 0],
    [-1, 1],
];

fn collect_number(scheme: &mut Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut y_start = y;

    while y_start != 0 && scheme[x][y_start - 1].is_digit(10) {
        y_start -= 1;
    }
    let mut y_end = y;
    while y_end < scheme[x].len() && scheme[x][y_end].is_digit(10) {
        y_end += 1;
    }

    let mut number = 0;

    for y in y_start..y_end {
        number *= 10;
        number += scheme[x][y].to_digit(10).unwrap() as usize;
        scheme[x][y] = '.';
    }

    number
}

fn collect_numbers(scheme: &mut Vec<Vec<char>>, x: usize, y: usize) -> Vec<usize> {
    let mut ret = vec![];

    scheme[x][y] = '.';

    for [offset_x, offset_y] in OFFSETS {
        let (pos_x, pos_y) = (
            (x as isize + offset_x) as usize,
            (y as isize + offset_y) as usize,
        );

        if let Some(row) = scheme.get_mut(pos_x) {
            if let Some(c) = row.get_mut(pos_y) {
                if c.is_digit(10) {
                    let number = collect_number(scheme, pos_x, pos_y);
                    ret.push(number);
                }
            }
        }
    }
    ret
}

fn solve1() -> usize {
    let input = include_str!("../input.txt");

    let mut engine_scheme: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut sum: usize = 0;
    for x in 0..engine_scheme.len() {
        for y in 0..engine_scheme[x].len() {
            if !engine_scheme[x][y].is_digit(10) && engine_scheme[x][y] != '.' {
                let numbers = collect_numbers(&mut engine_scheme, x, y);
                sum += numbers.iter().sum::<usize>();
            }
        }
    }

    sum
}

fn solve2() -> usize {
    let input = include_str!("../input.txt");

    let mut engine_scheme: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut sum: usize = 0;
    for x in 0..engine_scheme.len() {
        for y in 0..engine_scheme[x].len() {
            if !engine_scheme[x][y].is_digit(10) && engine_scheme[x][y] != '.' {
                let was_star = engine_scheme[x][y] == '*';
                let numbers = collect_numbers(&mut engine_scheme, x, y);
                if was_star && numbers.len() == 2 {
                    sum += numbers.iter().product::<usize>();
                }
            }
        }
    }

    sum
}
fn main() {
    dbg!(solve1());
    dbg!(solve2());
}
