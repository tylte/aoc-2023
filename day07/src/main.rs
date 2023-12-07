use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    High,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}
impl HandType {
    fn add_joker(&self) -> Self {
        match self {
            HandType::High => HandType::OnePair, 
            HandType::OnePair => HandType::Three,
            HandType::TwoPair => HandType::FullHouse,
            HandType::Three => HandType::Four,
            HandType::FullHouse => HandType::Four,
            HandType::Four => HandType::Five,
            HandType::Five => panic!("HandType::Six"),
        }

    }
}

const CARD_VARIANT: usize = 13;

trait Card where Self: From<char> + Ord {
    fn as_usize(&self) -> usize;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CardV1 {
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



impl From<char> for CardV1 {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CardV2 {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}


impl From<char> for CardV2 {
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

impl Card for CardV1 {
    fn as_usize(&self) -> usize {
        *self as usize
    }
}


impl Card for CardV2 {
    fn as_usize(&self) -> usize {
        if matches!(self, CardV2::J) {
            usize::MAX
        } else {
            *self as usize
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Hand<T: Card> {
    cards: Vec<T>,
}

impl<T:Card> Ord for Hand<T> {
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

impl<T:Card> Hand<T> {
    fn hand_type(&self) -> HandType {
        let mut pair_nb = 0;
        let mut has_three = false;
        let mut cards_count = [0; CARD_VARIANT];
        let mut nb_joker = 0;
        for card in &self.cards {
            let card_value = card.as_usize();
            if card_value == usize::MAX {
                nb_joker +=1;
            } else {
                cards_count[card_value] += 1;
            }
        }
        let mut hand_type = HandType::High;

        for count in cards_count {
            if count == 5 || nb_joker == 5 {
                return HandType::Five;
            } else if count == 4 {
                hand_type = HandType::Four;
            } else if count == 3 {
                has_three = true;
            } else if count == 2 {
                pair_nb += 1;
            }
        }

        if has_three {
            if pair_nb == 1 {
               hand_type =  HandType::FullHouse;
            } else {
                hand_type = HandType::Three;
            }
        } else {
            if pair_nb == 2 {
                hand_type = HandType::TwoPair;
            } else if pair_nb == 1 {
                hand_type = HandType::OnePair;
            }  
        }
        
        for _ in 0..nb_joker {
            hand_type = hand_type.add_joker();
        }
        return hand_type;
    }
}

#[derive(Debug)]
struct CardsBids<T: Card> {
    cards_bids: Vec<(Hand<T>, usize)>,
}

impl<T:Card> FromStr for CardsBids<T> {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let cards_bids = input
            .lines()
            .map(|line| {
                let (raw_hand, rank) = line.split_once(" ").unwrap();

                let cards: Vec<_> = raw_hand.chars().map(T::from).collect();
                let rank: usize = rank.parse().unwrap();
                (Hand { cards }, rank)
            })
            .collect::<Vec<_>>();

        Ok(Self { cards_bids })
    }
}

impl<T:Card> CardsBids<T> {
    fn sort_hand(&mut self) {
        self.cards_bids.sort_by(|(a, _), (b, _)| a.cmp(b));
    }
}

fn solve<T:Card>() -> usize {
    let input = include_str!("../input.txt");

    let mut hands: CardsBids<T> = input.parse().unwrap();

    hands.sort_hand();

    hands
        .cards_bids
        .iter()
        .enumerate()
        .map(|(idx, (_, rank))| (idx + 1) * rank)
        .sum::<usize>()
}

fn main() {
    dbg!(solve::<CardV1>());
    dbg!(solve::<CardV2>());
}
