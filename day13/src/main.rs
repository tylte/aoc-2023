fn maybe_solve(pattern: &[Vec<char>]) -> Option<Vec<usize>> {
    let mut ret: Option<Vec<usize>> = None;
    for i in 1..pattern[0].len() {
        if (0..pattern.len()).all(|j| {
            let mut left = pattern[j].iter().take(i).rev();
            let mut right = pattern[j].iter().skip(i);

            while let (Some(right), Some(left)) = (left.next(), right.next()) {
                if right != left {
                    return false;
                }
            }

            true
        }) {
            match &mut ret {
                Some(ret) => ret.push(i),
                None => ret = Some(vec![i]),
            }
        }
    }

    for i in 1..pattern.len() {
        if (0..pattern[0].len()).all(|j| {
            let mut left = (0..i).map(|i| pattern[i][j]).rev();
            let mut right = (i..pattern.len()).map(|i| pattern[i][j]);

            while let (Some(right), Some(left)) = (left.next(), right.next()) {
                if right != left {
                    return false;
                }
            }

            true
        }) {
            match &mut ret {
                Some(ret) => ret.push(i * 100),
                None => ret = Some(vec![i * 100]),
            }
        }
    }
    ret
}

fn solve1() -> usize {
    let input = include_str!("../input.txt");

    input
        .split("\n\n")
        .map(|pattern| {
            let pattern = pattern
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            maybe_solve(&pattern).unwrap()[0]
        })
        .sum()
}
fn solve2() -> usize {
    let input = include_str!("../input.txt");

    input
        .split("\n\n")
        .map(|pattern| {
            let mut pattern = pattern
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            let initial_value = maybe_solve(&pattern).unwrap()[0];

            for i in 0..pattern.len() {
                for j in 0..pattern[i].len() {
                    if pattern[i][j] == '.' {
                        pattern[i][j] = '#';
                    } else {
                        pattern[i][j] = '.';
                    }
                    if let Some(values) = maybe_solve(&pattern) {
                        for value in values {
                            if value != initial_value {
                                return value;
                            }
                        }
                    }
                    if pattern[i][j] == '.' {
                        pattern[i][j] = '#';
                    } else {
                        pattern[i][j] = '.';
                    }
                }
            }
            panic!("No value :(")
        })
        .sum()
}

fn main() {
    dbg!(solve1());
    dbg!(solve2());
}
