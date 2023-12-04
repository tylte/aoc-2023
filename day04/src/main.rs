use std::collections::HashMap;

fn solve1() -> usize {
    let input = include_str!("../input.txt");

    input
        .lines()
        .map(|line| {
            let (winning, mine) = line.split_once(":").unwrap().1.split_once("|").unwrap();
            let winning_nbs = winning
                .split_whitespace()
                .map(|str_nb| str_nb.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let matching_nb = mine
                .split_whitespace()
                .map(|str_nb| str_nb.parse::<usize>().unwrap())
                .filter(|my_nb| winning_nbs.contains(my_nb))
                .count();

            if matching_nb > 0 {
                2_usize.pow(matching_nb as u32 - 1)
            } else {
                0
            }
        })
        .sum::<usize>()
}

fn solve2() -> usize {
    let input = include_str!("../input.txt");

    let mut card_to_copy: HashMap<usize, usize> = HashMap::new();
    let mut total_card = 0;

    input.lines().enumerate().for_each(|(card_nb, line)| {
        let card_numbers = card_to_copy.get(&card_nb).copied().unwrap_or(1);

        total_card += card_numbers;

        let (winning, mine) = line.split_once(":").unwrap().1.split_once("|").unwrap();
        let winning_nbs = winning
            .split_whitespace()
            .map(|str_nb| str_nb.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let matching_nb = mine
            .split_whitespace()
            .map(|str_nb| str_nb.parse::<usize>().unwrap())
            .filter(|my_nb| winning_nbs.contains(my_nb))
            .count();

        for new_card_id in (card_nb + 1)..(card_nb + 1 + matching_nb) {
            *card_to_copy.entry(new_card_id).or_insert(1) += card_numbers;
        }
    });

    total_card
}

fn main() {
    dbg!(solve1());
    dbg!(solve2());
}
