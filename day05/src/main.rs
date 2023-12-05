use std::collections::VecDeque;

fn solve1() -> usize {
    let input = include_str!("../input.txt");

    let mut lines = input.split("\n\n");
    let mut values: Vec<_> = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|seed| seed.parse::<usize>().unwrap())
        .collect();

    lines
        .map(|range_map| {
            range_map
                .lines()
                .skip(1)
                .map(|range_info| {
                    range_info
                        .split_whitespace()
                        .map(|nb| nb.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .for_each(|mappings| {
            let mut next_values = values.to_vec();
            for mapping in mappings {
                let (source, destination, length) = (mapping[1], mapping[0], mapping[2]);
                for i in 0..values.len() {
                    let value = values[i];
                    if value >= source && value < source + length {
                        next_values[i] = destination + (value - source);
                    }
                }
            }
            values = next_values;
        });

    values.into_iter().min().unwrap()
}

fn solve2() -> usize {
    let input = include_str!("../input.txt");

    let mut lines = input.split("\n\n");
    let mut values = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|seed| seed.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| [chunk[0], chunk[0] + chunk[1]])
        .collect::<VecDeque<_>>();

    lines
        .map(|range_map| {
            range_map
                .lines()
                .skip(1)
                .map(|range_info| {
                    range_info
                        .split_whitespace()
                        .map(|nb| nb.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .for_each(|mappings| {
            let mut next_values = VecDeque::new();

            while !values.is_empty() {
                let [start, end] = values.pop_front().unwrap();

                let next_len = next_values.len();
                for mapping in &mappings {
                    let (source, destination, length) = (mapping[1], mapping[0], mapping[2] - 1);

                    if source < start && end < source + length {
                        // Every values are translated
                        next_values.push_back([
                            destination + (start - source),
                            destination + (end - source),
                        ]);
                    } else if source < start && start < source + length {
                        // Overlap with the start of the range
                        next_values
                            .push_back([destination + (start - source), destination + length]);

                        if end > source + length + 1 {
                            values.push_back([source + length + 1, end]);
                        }
                    } else if source < end && end < source + length {
                        next_values.push_back([destination, destination + (end - source)]);

                        if start > source - 1 {
                            values.push_back([start, source - 1]);
                        }
                    } else if start < source && source + length < end {
                        if start > source - 1 {
                            values.push_back([start, source - 1]);
                        }
                        if end > source + length + 1 {
                            values.push_back([source + length + 1, end]);
                        }

                        next_values.push_back([destination, destination + length]);
                    }
                }

                let no_map_found = next_len == next_values.len();

                if no_map_found {
                    next_values.push_back([start, end]);
                }
            }

            values = next_values;
        });

    values.into_iter().map(|range| range[0]).min().unwrap()
}

fn main() {
    dbg!(solve1());
    dbg!(solve2());
}
