use std::collections::HashMap;

fn solve1() -> usize {
    let input = include_str!("../input.txt");

    let (ixs, map) = input.split_once("\n\n").unwrap();

    let map = map
        .lines()
        .map(|line| {
            let (parent, childs) = line.split_once(" = ").unwrap();

            let mut childs_iter = childs.split(&['(', ',', ')', ' ']).filter(|&c| c != "");

            let left_child = childs_iter.next().unwrap();
            let right_child = childs_iter.next().unwrap();

            (parent, (left_child, right_child))
        })
        .collect::<HashMap<_, _>>();

    let mut count = 0;

    let mut pos = "AAA";
    for ix in ixs.chars().cycle() {
        let childs = map.get(pos).unwrap();

        if ix == 'L' {
            pos = childs.0;
        } else {
            pos = childs.1;
        }

        count += 1;
        if pos == "ZZZ" {
            break;
        }
    }

    count
}

fn solve2() -> usize {
    fn lcm(first: usize, second: usize) -> usize {
        first * second / gcd(first, second)
    }

    fn gcd(first: usize, second: usize) -> usize {
        let mut max = first;
        let mut min = second;
        if min > max {
            let val = max;
            max = min;
            min = val;
        }

        loop {
            let res = max % min;
            if res == 0 {
                return min;
            }

            max = min;
            min = res;
        }
    }
    let input = include_str!("../input.txt");

    let (ixs, map) = input.split_once("\n\n").unwrap();

    let map = map
        .lines()
        .map(|line| {
            let (parent, childs) = line.split_once(" = ").unwrap();

            let mut childs_iter = childs.split(&['(', ',', ')', ' ']).filter(|&c| c != "");

            let left_child = childs_iter.next().unwrap();
            let right_child = childs_iter.next().unwrap();

            (parent, (left_child, right_child))
        })
        .collect::<HashMap<_, _>>();

    let mut count = 0;

    let mut pos: Vec<_> = map
        .keys()
        .filter(|node| node.chars().skip(2).next().unwrap() == 'A')
        .collect();

    let mut pos_to_z = vec![0; pos.len()];


    for ix in ixs.chars().cycle() {
        for i in 0..pos.len() {
            let pos = &mut pos[i];
            let childs = map.get(*pos).unwrap();

            if ix == 'L' {
                *pos = &childs.0;
            } else {
                *pos = &childs.1;
            }

            if pos.chars().skip(2).next().unwrap() == 'Z' && pos_to_z[i] == 0 {
                pos_to_z[i] = count + 1;

            }
        }

        count += 1;
        if pos_to_z.iter().all(|count| *count != 0) {
            break;
        }
    }

    pos_to_z.into_iter().fold(1, |acc, x| lcm(acc,x))
}

fn main() {
    dbg!(solve1());
    dbg!(solve2());
}
