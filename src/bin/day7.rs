use std::{cmp::Ordering, collections::HashMap};

use aoc2023::lib;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
enum CardValue {
    Joker = 1,
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard = 1,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

type Hand = [CardValue; 5];

fn get_card_value(card: &u8) -> CardValue {
    match card {
        b'2' => CardValue::Two,
        b'3' => CardValue::Three,
        b'4' => CardValue::Four,
        b'5' => CardValue::Five,
        b'6' => CardValue::Six,
        b'7' => CardValue::Seven,
        b'8' => CardValue::Eight,
        b'9' => CardValue::Nine,
        b'T' => CardValue::Ten,
        b'J' => CardValue::Jack,
        b'Q' => CardValue::Queen,
        b'K' => CardValue::King,
        b'A' => CardValue::Ace,
        _ => panic!("Unexpected card: {}", card),
    }
}

fn get_hand_type(hand: Hand) -> HandType {
    let mut counts = HashMap::new();
    for card in hand {
        counts.insert(card, counts.get(&card).unwrap_or(&0) + 1);
    }
    match counts.len() {
        5 => HandType::HighCard,
        4 => HandType::OnePair,
        // ABCCC
        // ABBCC
        3=>
            if counts.iter().any(|(_, &v)| v == 3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        ,
        // AABBB
        // ABBBB
        2 =>
        if counts.iter().any(|(_, &v)| v == 4) {
            HandType::FourOfAKind
        } else {
            HandType::FullHouse
        },
        1 => HandType::FiveOfAKind,
        _ => panic!("Unexpected counts: {:?}", counts)
    }
}

pub fn part1() {
    let mut hands = Vec::<(Hand, HandType, i64)>::new();

    let lines = lib::read_byte_lines();
    for line in lines {
        let mut hand = [CardValue::Ace; 5];
        for (i, card) in line[0..5].iter().enumerate() {
            hand[i] = get_card_value(card);
        }
        let hand_type = get_hand_type(hand);

        let mut bid: i64 = 0;
        for byte in line[6..].iter() {
            bid = bid * 10 + (byte - b'0') as i64;
        }

        hands.push((hand, hand_type, bid));
    }

    hands.sort_by(|(hand1, hand1_type, _), (hand2, hand2_type, _)| {
        match hand1_type.cmp(hand2_type) {
            Ordering::Equal => {
                for (card1, card2) in hand1.iter().zip(hand2.iter()) {
                    if card1 < card2 {
                        return Ordering::Less;
                    } else if card1 > card2 {
                        return Ordering::Greater;
                    }
                }
                Ordering::Equal
            }
            v => v,
        }
    });

    let total: i64 = hands
        .iter()
        .enumerate()
        .map(|(index, (_, _, bid))| bid * (index + 1) as i64)
        .sum();
    println!("{}", total);
}

fn get_card_value_with_joker(card: &u8) -> CardValue {
    match card {
        b'J' => CardValue::Joker,
        b'2' => CardValue::Two,
        b'3' => CardValue::Three,
        b'4' => CardValue::Four,
        b'5' => CardValue::Five,
        b'6' => CardValue::Six,
        b'7' => CardValue::Seven,
        b'8' => CardValue::Eight,
        b'9' => CardValue::Nine,
        b'T' => CardValue::Ten,
        b'Q' => CardValue::Queen,
        b'K' => CardValue::King,
        b'A' => CardValue::Ace,
        _ => panic!("Unexpected card: {}", card),
    }
}

fn get_hand_type_with_joker(hand: Hand) -> HandType {
    let mut counts = HashMap::new();
    for card in hand {
        counts.insert(card, counts.get(&card).unwrap_or(&0) + 1);
    }

    if let Some(jack_count) = counts.get(&CardValue::Joker) {
        if let Some(best_card) = counts
            .iter()
            .filter(|(&card, _)| card != CardValue::Joker)
            .max_by_key(|(_, &count)| count)
        {
            counts.insert(*best_card.0, best_card.1 + jack_count);
            counts.remove(&CardValue::Joker);
        }
    }

    match counts.len() {
        5 => HandType::HighCard,
        4 => HandType::OnePair,
        // ABCCC
        // ABBCC
        3=>
            if counts.iter().any(|(_, &v)| v == 3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        ,
        // AABBB
        // ABBBB
        2 =>
        if counts.iter().any(|(_, &v)| v == 4) {
            HandType::FourOfAKind
        } else {
            HandType::FullHouse
        },
        1 => HandType::FiveOfAKind,
        _ => panic!("Unexpected counts: {:?}", counts)
    }
}

pub fn part2() {
    let mut hands = Vec::<(Hand, HandType, i64)>::new();

    let lines = lib::read_byte_lines();
    for line in lines {
        let mut hand = [CardValue::Ace; 5];
        for (i, card) in line[0..5].iter().enumerate() {
            hand[i] = get_card_value_with_joker(card);
        }
        let hand_type = get_hand_type_with_joker(hand);

        let mut bid: i64 = 0;
        for byte in line[6..].iter() {
            bid = bid * 10 + (byte - b'0') as i64;
        }

        hands.push((hand, hand_type, bid));
    }

    hands.sort_by(|(hand1, hand1_type, _), (hand2, hand2_type, _)| {
        match hand1_type.cmp(hand2_type) {
            Ordering::Equal => {
                for (card1, card2) in hand1.iter().zip(hand2.iter()) {
                    if card1 < card2 {
                        return Ordering::Less;
                    } else if card1 > card2 {
                        return Ordering::Greater;
                    }
                }
                Ordering::Equal
            }
            v => v,
        }
    });

    let total: i64 = hands
        .iter()
        .enumerate()
        .map(|(index, (_, _, bid))| bid * (index + 1) as i64)
        .sum();
    println!("{}", total);
}

pub fn main() {
    // part1();
    part2();
}
