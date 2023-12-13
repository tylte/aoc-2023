fn is_start_correct(seq: &[char], numbers: &[usize], idx: usize) -> bool {
    let mut nb_idx = 0;
    let mut streak = 0;
    let mut is_chain = false;
    for &c in seq.iter().take(idx + 1) {
        if c == '#' {
            streak += 1;
            is_chain = true;
            if let Some(nb) = numbers.get(nb_idx) {
                if streak > *nb {
                    return false;
                }
            } else {
                return false;
            }
        } else if c == '.' {
            if is_chain {
                if let Some(nb) = numbers.get(nb_idx) {
                    if streak > *nb {
                        return false;
                    }
                } else {
                    return false;
                }
                streak = 0;
                nb_idx += 1;
                is_chain = false;
            }
        }
    }

    if is_chain {
        if let Some(nb) = numbers.get(nb_idx) {
            if streak > *nb {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

fn is_correct(seq: &[char], numbers: &[usize]) -> bool {
    let mut nb_idx = 0;
    let mut streak = 0;
    let mut is_chain = false;
    for &c in seq {
        if c == '#' {
            streak += 1;
            is_chain = true;
        } else if c == '.' {
            if is_chain {
                if nb_idx >= numbers.len() || streak != numbers[nb_idx] {
                    return false;
                }
                streak = 0;
                nb_idx += 1;
                is_chain = false;
            }
        }
    }

    if is_chain {
        if nb_idx >= numbers.len() || streak != numbers[nb_idx] {
            return false;
        }
        nb_idx += 1;
    }

    nb_idx == numbers.len()
}

fn count_possiblity(seq: &Vec<char>, numbers: &[usize], seq_idx: usize) -> usize {
    if seq_idx == seq.len() {
        if is_correct(seq, numbers) {
            return 1;
        } else {
            return 0;
        }
    }

    if !is_start_correct(seq, numbers, seq_idx) {
        return 0;
    }
    let c = seq[seq_idx];
    let mut ret = 0;

    if c == '?' {
        let mut seq = seq.to_vec();
        seq[seq_idx] = '.';
        ret += count_possiblity(&seq, numbers, seq_idx + 1);
        seq[seq_idx] = '#';
        ret += count_possiblity(&seq, numbers, seq_idx + 1);
    } else {
        ret += count_possiblity(seq, numbers, seq_idx + 1);
    }

    ret
}
fn solve1() -> usize {
    let input = include_str!("../input.txt");

    input
        .lines()
        .map(|line| {
            let (seq, numbers) = line.split_once(" ").unwrap();

            let numbers: Vec<_> = numbers
                .split(",")
                .map(|c| c.parse::<usize>().unwrap())
                .collect();
            let seq: Vec<_> = seq.chars().collect();

            count_possiblity(&seq, &numbers, 0)
        })
        .sum()
}

fn solve2() -> usize {
    let input = include_str!("../input.txt");

    input
        .lines()
        .take(2)
        .map(|line| {
            let (seq, numbers) = line.split_once(" ").unwrap();

            let numbers: Vec<_> = numbers
                .split(",")
                .map(|c| c.parse::<usize>().unwrap())
                .collect();

            let numbers = numbers.repeat(5);

            let mut seq: Vec<_> = seq.chars().collect();
            seq.push('?');
            let mut seq = seq.repeat(5);
            seq.pop().unwrap();

            dbg!(count_possiblity(&seq, &numbers, 0))
        })
        .sum()
}

fn main() {
    dbg!(solve1());
    dbg!(solve2());
}
