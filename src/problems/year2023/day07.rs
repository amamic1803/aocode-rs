use crate::{Error, Solution};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::LazyLock;

day!(Day07, 2023, 7, "Camel Cards");

impl Solution for Day07 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut hands = input
            .trim()
            .lines()
            .map(|hand| {
                let mut hand = Hand::new(hand);
                hand.calc_dec_val1();
                hand
            })
            .collect_vec();

        hands.sort();

        Ok(hands
            .into_iter()
            .enumerate()
            .map(|(i, hand)| hand.bid * (i as u64 + 1))
            .sum::<u64>()
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut hands = input
            .trim()
            .lines()
            .map(|hand| {
                let mut hand = Hand::new(hand);
                hand.calc_dec_val2();
                hand
            })
            .collect_vec();

        hands.sort();

        Ok(hands
            .into_iter()
            .enumerate()
            .map(|(i, hand)| hand.bid * (i as u64 + 1))
            .sum::<u64>()
            .to_string())
    }
}

static CARD_ORDER_1: LazyLock<HashMap<char, u64>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    map.insert('2', 0);
    map.insert('3', 1);
    map.insert('4', 2);
    map.insert('5', 3);
    map.insert('6', 4);
    map.insert('7', 5);
    map.insert('8', 6);
    map.insert('9', 7);
    map.insert('T', 8);
    map.insert('J', 9);
    map.insert('Q', 10);
    map.insert('K', 11);
    map.insert('A', 12);

    map
});
static CARD_ORDER_2: LazyLock<HashMap<char, u64>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    map.insert('J', 0);
    map.insert('2', 1);
    map.insert('3', 2);
    map.insert('4', 3);
    map.insert('5', 4);
    map.insert('6', 5);
    map.insert('7', 6);
    map.insert('8', 7);
    map.insert('9', 8);
    map.insert('T', 9);
    map.insert('Q', 10);
    map.insert('K', 11);
    map.insert('A', 12);

    map
});

struct Hand<'a> {
    cards: &'a str,
    bid: u64,
    dec_val: u64,
}
impl<'a> Hand<'a> {
    fn new(hand: &'a str) -> Self {
        let (cards, bid) = hand.split_whitespace().collect_tuple().unwrap();

        Self {
            cards,
            bid: bid.parse::<u64>().unwrap(),
            dec_val: 0,
        }
    }

    fn calc_dec_val1(&mut self) {
        self.dec_val = 0;
        // we consider every card as digit in base 13
        // and add the most significant digits also in base 13 that represent the hand type
        // converting that to decimal gives us a unique value for each hand
        for (exp, card) in self.cards.chars().rev().enumerate() {
            self.dec_val += CARD_ORDER_1[&card] * 13u64.pow(exp as u32);
        }
        self.dec_val += HandTypes::from1(self.cards).value() * 13u64.pow(5);
    }

    fn calc_dec_val2(&mut self) {
        self.dec_val = 0;
        // we consider every card as digit in base 13
        // and add the most significant digits also in base 13 that represent the hand type
        // converting that to decimal gives us a unique value for each hand
        for (exp, card) in self.cards.chars().rev().enumerate() {
            self.dec_val += CARD_ORDER_2[&card] * 13u64.pow(exp as u32);
        }
        self.dec_val += HandTypes::from2(self.cards).value() * 13u64.pow(5);
    }
}
impl PartialEq<Self> for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.dec_val == other.dec_val
    }
}
impl Eq for Hand<'_> {}
impl PartialOrd<Self> for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dec_val.cmp(&other.dec_val)
    }
}

#[derive(Eq, PartialEq)]
enum HandTypes {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}
impl HandTypes {
    fn from1(cards: &str) -> Self {
        let mut card_counts = HashMap::new();

        for card in cards.chars() {
            let count = card_counts.entry(card).or_insert(0);
            *count += 1;
        }

        let mut card_counts = card_counts.values().copied().collect_vec();
        card_counts.sort();

        match card_counts.as_slice() {
            [5] => HandTypes::FiveOfAKind,
            [1, 4] => HandTypes::FourOfAKind,
            [2, 3] => HandTypes::FullHouse,
            [1, 1, 3] => HandTypes::ThreeOfAKind,
            [1, 2, 2] => HandTypes::TwoPair,
            [1, 1, 1, 2] => HandTypes::OnePair,
            [1, 1, 1, 1, 1] => HandTypes::HighCard,
            _ => panic!("Invalid hand"),
        }
    }

    fn from2(cards: &str) -> Self {
        let mut card_counts = HashMap::new();
        for card in cards.chars() {
            let count = card_counts.entry(card).or_insert(0);
            *count += 1;
        }
        let j_count = card_counts.remove_entry(&'J').unwrap_or((' ', 0)).1;

        let mut card_counts = card_counts.values().copied().collect_vec();
        if card_counts.is_empty() {
            card_counts.push(0);
        }

        let mut best_hand_type = HandTypes::HighCard;
        for i in 0..card_counts.len() {
            let mut temp_card_counts = card_counts.clone();
            temp_card_counts[i] += j_count;
            temp_card_counts.sort();
            let type_value = match temp_card_counts.as_slice() {
                [5] => HandTypes::FiveOfAKind,
                [1, 4] => HandTypes::FourOfAKind,
                [2, 3] => HandTypes::FullHouse,
                [1, 1, 3] => HandTypes::ThreeOfAKind,
                [1, 2, 2] => HandTypes::TwoPair,
                [1, 1, 1, 2] => HandTypes::OnePair,
                [1, 1, 1, 1, 1] => HandTypes::HighCard,
                _ => panic!("Invalid hand"),
            };
            if type_value > best_hand_type {
                best_hand_type = type_value;
            }
        }

        best_hand_type
    }

    /// Convert hand type to card decimal value, used for calculating decimal value of hand
    fn value(&self) -> u64 {
        match self {
            HandTypes::FiveOfAKind => 6,
            HandTypes::FourOfAKind => 5,
            HandTypes::FullHouse => 4,
            HandTypes::ThreeOfAKind => 3,
            HandTypes::TwoPair => 2,
            HandTypes::OnePair => 1,
            HandTypes::HighCard => 0,
        }
    }
}
impl PartialOrd for HandTypes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HandTypes {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}
