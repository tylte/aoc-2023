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
    let mut values: Vec<_> = lines
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
        .collect::<Vec<_>>();

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
            let mut next_values = vec![]; 
            for mapping in mappings {
                let (source, destination, length) = (mapping[1], mapping[0], mapping[2]);
                for i in 0..values.len() {
                    let [start, end] = values[i];


                    if start > source && end < source + length {
                        // Every values are translated
                        next_values.push([destination + (start - source), destination + (end - source)]);
                    }
                    else if start > source && start < source + length {
                        // Overlap with the start of the range
                        next_values.push([destination + (start - source), destination + ()]);
                    }
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
