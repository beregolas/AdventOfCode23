use std::cmp::Ordering;

#[derive(Debug, Hash, Copy, Clone)]
struct Card {
    value: char,
}

impl Card {
    fn new(value: char) -> Card {
        Card { value }
    }

    fn to_u8(&self) -> u8 {
        match self.value {
            'T' => 10,
            'J' => 1,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => self.value.to_digit(10).unwrap() as u8
        }
    }
}

impl Eq for Card {}

impl PartialEq<Self> for Card {
    fn eq(&self, other: &Self) -> bool {
        self.to_u8() == other.to_u8()
    }
}

impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_u8().cmp(&other.to_u8())
    }
}

#[derive(Debug, Hash, Copy, Clone)]
enum HandValue {
    HighCard = 0,
    OnePair = 1,
    TwoPairs = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

impl HandValue {
    fn to_u8(&self) -> u8 {
        match self {
            HandValue::HighCard => 0,
            HandValue::OnePair => 1,
            HandValue::TwoPairs => 2,
            HandValue::ThreeOfAKind => 3,
            HandValue::FullHouse => 4,
            HandValue::FourOfAKind => 5,
            HandValue::FiveOfAKind => 6,
        }
    }
}

fn compute_hand_value(cards: &[Card; 5]) -> HandValue {
    // count the number of each card
    let mut counts: Vec<u8> = cards.iter()
        .filter(|&card| card.value != 'J')
        .fold([0; 15], |mut counts: [u8; 15], card| {
            counts[card.to_u8() as usize] += 1;
            counts
        })
        .into_iter()
        .filter(|&count| count > 0)
        .collect();
    counts.sort();
    counts.reverse();
    let jokers: u8 = cards.iter().fold(0, |jokers, card| if card.value == 'J' { jokers + 1 } else { jokers });
    // check for five jokers, the only hand where jokers don't joke
    if jokers == 5 {
        return HandValue::FiveOfAKind;
    }
    // check for the highest count else
    match counts[..] {
        [x, ..] if x + jokers == 5 => HandValue::FiveOfAKind,
        [x, ..] if x + jokers == 4 => HandValue::FourOfAKind,
        [x, y, ..] if x + jokers == 3 && y == 2 => HandValue::FullHouse,
        [x, ..] if x + jokers == 3 => HandValue::ThreeOfAKind,
        [x, y, ..] if x + jokers == 2 && y == 2 => HandValue::TwoPairs,
        [x, ..] if x + jokers == 2 => HandValue::OnePair,
        _ => HandValue::HighCard
    }
}
#[derive(Debug, Hash, Copy, Clone)]
struct Hand {
    cards: [Card; 5],
    value: HandValue,
}


impl Hand {
    fn new(cards: [Card; 5]) -> Hand {
        // find hand value
        Hand { cards, value: compute_hand_value(&cards) }
    }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.value.to_u8() == other.value.to_u8()
            && self.cards == other.cards
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let value_cmp = self.value.to_u8().cmp(&other.value.to_u8());
        if value_cmp == Ordering::Equal {
            self.cards.cmp(&other.cards)
        } else {
            value_cmp
        }
    }
}

pub(crate) fn c1(_input: String) -> String {
    "sum".to_string()
}

pub(crate) fn c2(input: String) -> String {
    let mut hands: Vec<(Hand, u32)> = input.lines().map(|line| (
        Hand::new(line
            .split(" ")
            .next()
            .unwrap()
            .chars()
            .map(|c| Card::new(c))
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap()
        ),
        line
            .split(" ")
            .last()
            .expect("")
            .parse::<u32>()
            .unwrap()
    ))
        .collect();
    hands.sort_by(|(hand1, _), (hand2, _)| hand1.cmp(hand2));
    let mut total: u32 = 0;
    for (i, (hand, bid)) in hands.iter().enumerate() {
        total += (i as u32 + 1) * bid;
        println!("{}: {:?} {}", i + 1, hand, bid);
    }
    total.to_string()
}

