fn solve1() -> usize {
    let input = include_str!("../input.txt");

    let (times, distances) = input.split_once("\n").unwrap();

    let times: Vec<_> = times
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|nb| nb.parse::<usize>().unwrap())
        .collect();

    let distances: Vec<_> = distances
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|nb| nb.parse::<usize>().unwrap())
        .collect();

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| {
            (0..=time)
                .map(|possible_hold_time| possible_hold_time * (time - possible_hold_time))
                .filter(|&current_distance| current_distance > distance)
                .count()
        })
        .product::<usize>()
}

fn solve2() -> usize {
    let input = include_str!("../input.txt");

    let (times, distances) = input.split_once("\n").unwrap();

    let time = times
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    let distance = distances
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    (0..=time)
        .map(|possible_hold_time| possible_hold_time * (time - possible_hold_time))
        .filter(|&current_distance| current_distance > distance)
        .count()
}

fn main() {
    dbg!(solve1());
    dbg!(solve2());
}
