use std::collections::HashSet;

fn solve1() -> usize {
    let input = include_str!("../input.txt");

    let mut galaxy_id = 0;

    let mut image: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == '#' {
                        galaxy_id += 1;
                        galaxy_id
                    } else {
                        0
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut i = 0;

    let line_len = image[0].len();
    while i < image.len() {
        if image[i].iter().all(|&x| x == 0) {
            image.insert(i, vec![0; line_len]);
            i += 1;
        }
        i += 1;
    }
    let mut i = 0;
    while i < image[0].len() {
        if (0..image.len()).all(|x| image[x][i] == 0) {
            for line in image.iter_mut() {
                line.insert(i, 0);
            }
            i += 1;
        }
        i += 1;
    }

    let galaxies = image
        .iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(y, c)| if *c != 0 { Some((x, y)) } else { None })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut count = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (x_a, y_a) = galaxies[i];
            let (x_b, y_b) = galaxies[j];

            count += x_a.abs_diff(x_b) + y_a.abs_diff(y_b);
        }
    }

    count
}

fn solve2() -> usize {
    let input = include_str!("../input.txt");

    let mut galaxy_id = 0;

    let image: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == '#' {
                        galaxy_id += 1;
                        galaxy_id
                    } else {
                        0
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut added_line = HashSet::new();
    for i in 0..image.len() {
        if image[i].iter().all(|&x| x == 0) {
            added_line.insert(i);
        }
    }

    let mut added_column = HashSet::new();
    for i in 0..image[0].len() {
        if (0..image.len()).all(|x| image[x][i] == 0) {
            added_column.insert(i);
        }
    }

    let galaxies = image
        .iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(y, c)| if *c != 0 { Some((x, y)) } else { None })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    const UNIVERSE_EXPENSION: usize = 1000000 - 1;
    let mut count = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (x_a, y_a) = galaxies[i];
            let (x_b, y_b) = galaxies[j];

            let start_x = x_a.min(x_b);
            let end_x = x_a.max(x_b);

            let start_y = y_a.min(y_b);
            let end_y = y_a.max(y_b);

            for k in start_x..=end_x {
                if added_line.contains(&k) {
                    count += UNIVERSE_EXPENSION;
                }
            }

            for k in start_y..=end_y {
                if added_column.contains(&k) {
                    count += UNIVERSE_EXPENSION;
                }
            }
            count += (end_x - start_x) + (end_y - start_y);
        }
    }

    count
}

fn main() {
    dbg!(solve1());
    dbg!(solve2());
}
