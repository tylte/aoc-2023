use std::collections::HashMap;

fn hash_procedure(label: &str) -> usize {
    label
        .chars()
        .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

fn solve1() -> usize {
    include_str!("../input.txt")
        .split(",")
        .map(hash_procedure)
        .sum()
}

fn solve2() -> usize {
    let mut boxes: HashMap<usize, Vec<(&str, usize)>> = HashMap::new();
    include_str!("../input.txt").split(",").for_each(|ix| {
        let ix = ix.trim();
        if ix.contains("-") {
            let label = ix.split("-").next().unwrap();

            boxes.entry(hash_procedure(label)).and_modify(|lenses| {
                if let Some(position) = lenses.iter().position(|(lab, _)| label == *lab) {
                    lenses.remove(position);
                }
            });
        } else {
            let (label, number) = ix.split_once("=").unwrap();

            let number = number.parse::<usize>().unwrap();
            let lenses = boxes.entry(hash_procedure(label)).or_insert_with(Vec::new);
            if let Some(position) = lenses.iter().position(|(lab, _)| label == *lab) {
                lenses[position] = (label, number);
            } else {
                lenses.push((label, number));
            }
        }
    });

    boxes
        .into_iter()
        .map(|(box_nb, lenses)| {
            lenses
                .into_iter()
                .enumerate()
                .map(|(idx, (_, lens))| (box_nb + 1) * (idx + 1) * lens)
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    dbg!(solve1());
    dbg!(solve2());
}
