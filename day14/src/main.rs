fn move_to_north(positions: &mut Vec<Vec<char>>, x: usize, y: usize) {
    let mut new_x = x;
    while new_x >= 1 && positions[new_x - 1][y] == '.' {
        new_x -= 1;
    }
    positions[x][y] = '.';
    positions[new_x][y] = 'O';
}

fn move_to_west(positions: &mut Vec<Vec<char>>, x: usize, y: usize) {
    let mut new_y = y;
    while new_y >= 1 && positions[x][new_y - 1] == '.' {
        new_y -= 1;
    }
    positions[x][y] = '.';
    positions[x][new_y] = 'O';
}

fn move_to_south(positions: &mut Vec<Vec<char>>, x: usize, y: usize) {
    let mut new_x = x;
    while new_x < positions.len() - 1 && positions[new_x + 1][y] == '.' {
        new_x += 1;
    }
    positions[x][y] = '.';
    positions[new_x][y] = 'O';
}

fn move_to_east(positions: &mut Vec<Vec<char>>, x: usize, y: usize) {
    let mut new_y = y;
    while new_y < positions[x].len() - 1 && positions[x][new_y + 1] == '.' {
        new_y += 1;
    }
    positions[x][y] = '.';
    positions[x][new_y] = 'O';
}

fn washing_machine_cycle(positions: &mut Vec<Vec<char>>) {
    for i in 1..positions.len() {
        for j in 0..positions[i].len() {
            if positions[i][j] == 'O' {
                move_to_north(positions, i, j);
            }
        }
    }
    for i in 0..positions.len() {
        for j in 0..positions[i].len() {
            if positions[i][j] == 'O' {
                move_to_west(positions, i, j);
            }
        }
    }
    for i in (0..positions.len()).rev() {
        for j in 0..positions[i].len() {
            if positions[i][j] == 'O' {
                move_to_south(positions, i, j);
            }
        }
    }
    for i in 0..positions.len() {
        for j in (0..positions[i].len()).rev() {
            if positions[i][j] == 'O' {
                move_to_east(positions, i, j);
            }
        }
    }
}

fn solve1() -> usize {
    let input = include_str!("../input.txt");

    let mut positions = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for i in 1..positions.len() {
        for j in 0..positions[i].len() {
            if positions[i][j] == 'O' {
                move_to_north(&mut positions, i, j);
            }
        }
    }

    positions
        .into_iter()
        .rev()
        .enumerate()
        .map(|(idx, line)| {
            line.into_iter()
                .map(|char| if char == 'O' { idx + 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn solve2() -> usize {
    let input = include_str!("../input.txt");

    let mut positions = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut results: Vec<(Vec<Vec<char>>, usize)> = vec![];

    const ITERATIONS: usize = 1_000_000_000;
    for i in 0..ITERATIONS {
        washing_machine_cycle(&mut positions);

        let result: usize = positions
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, line)| {
                line.iter()
                    .map(|&char| if char == 'O' { idx + 1 } else { 0 })
                    .sum::<usize>()
            })
            .sum();

        if let Some(same_pos_idx) = results
            .iter()
            .position(|(last_pos, _)| positions.eq(last_pos))
        {
            let cycle_len = i - same_pos_idx;
            return results[((ITERATIONS - same_pos_idx - 1) % cycle_len) + same_pos_idx].1;
        }
        results.push((positions.to_vec(), result));
    }

    positions
        .into_iter()
        .rev()
        .enumerate()
        .map(|(idx, line)| {
            line.into_iter()
                .map(|char| if char == 'O' { idx + 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    dbg!(solve1());
    dbg!(solve2());
}
