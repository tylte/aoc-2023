use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    High,
}

const CARD_VARIANT: usize = 13;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => panic!("Unknown card {value}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Hand {
    cards: Vec<Card>,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_type = self.hand_type();
        let other_type = other.hand_type();

        let cmp_type = self_type.cmp(&other_type);

        if matches!(cmp_type, Ordering::Equal) {
            self.cards
                .iter()
                .zip(other.cards.iter())
                .map(|(card_a, card_b)| card_a.cmp(&card_b))
                .filter(|cmp| !matches!(cmp, Ordering::Equal))
                .next()
                .unwrap()
        } else {
            cmp_type
        }
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut pair_nb = 0;
        let mut has_three = false;
        let mut cards_count = [0; CARD_VARIANT];
        for card in &self.cards {
            cards_count[*card as usize] += 1;
        }
        for count in cards_count {
            if count == 5 {
                return HandType::Five;
            } else if count == 4 {
                return HandType::Four;
            } else if count == 3 {
                has_three = true;
            } else if count == 2 {
                pair_nb += 1;
            }
        }
        if has_three {
            if pair_nb == 1 {
                return HandType::FullHouse;
            } else {
                return HandType::Three;
            }
        } else {
            if pair_nb == 2 {
                return HandType::TwoPair;
            } else if pair_nb == 1 {
                return HandType::OnePair;
            } else {
                return HandType::High;
            }
        }
    }
}

#[derive(Debug)]
struct CardsBids {
    cards_bids: Vec<(Hand, usize)>,
}

impl FromStr for CardsBids {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let cards_bids = input
            .lines()
            .map(|line| {
                let (raw_hand, rank) = line.split_once(" ").unwrap();

                let cards: Vec<_> = raw_hand.chars().map(Card::from).collect();
                let rank: usize = rank.parse().unwrap();
                (Hand { cards }, rank)
            })
            .collect::<Vec<_>>();

        Ok(Self { cards_bids })
    }
}

impl CardsBids {
    fn sort_hand(&mut self) {
        self.cards_bids.sort_by(|(a, _), (b, _)| a.cmp(b));
    }
}

fn solve1() -> usize {
    let input = include_str!("../input.txt");

    let mut hands: CardsBids = input.parse().unwrap();

    hands.sort_hand();


    for hand in &hands.cards_bids {
        dbg!(hand, hand.0.hand_type());
    }
    println!("{hands:?}");
    dbg!(Card::T.cmp(&Card::A));

    hands
        .cards_bids
        .iter()
        .enumerate()
        .map(|(idx, (_, rank))| (idx + 1) * rank)
        .sum::<usize>()
}

fn main() {
    dbg!(solve1());
}
