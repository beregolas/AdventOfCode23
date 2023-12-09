use std::cmp::Ordering;
use std::ops::Index;

struct Card(char);

impl Card {
   fn new(card: char) -> Card {
      Card(card)
   }
}

impl PartialEq<Self> for Card {
   fn eq(&self, other: &Self) -> bool {
      self.0 == other.0 || self.0 == 'J' || other.0 == 'J'
   }
}

impl PartialOrd<Self> for Card {
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      let a = match self.0 {
         'T' => 10,
         'J' => 1,
         'Q' => 12,
         'K' => 13,
         'A' => 14,
         _ => self.0.to_digit(10).unwrap(),
      };
      let b = match other.0 {
         'T' => 10,
         'J' => 1,
         'Q' => 12,
         'K' => 13,
         'A' => 14,
         _ => other.0.to_digit(10).unwrap(),
      };
        Some(a.cmp(&b))
   }
}


#[derive(Debug, Copy, Clone)]
enum HandValues {
   FiveOfAKind = 6,
   FourOfAKind = 5,
   FullHouse = 4,
   ThreeOfAKind = 3,
   TwoPair = 2,
   OnePair = 1,
   HighCard = 0,
}

impl HandValues {
   fn new(cards: [char; 5]) -> HandValues {
      let mut values = cards;
      values.sort();
      let mut values_2 = values.map(|x| Card::new(x));
      match values_2 {
         [a, b, c, d, e] if a == b && b == c && c == d && d == e => HandValues::FiveOfAKind,
         [a, b, c, d, _] if a == b && b == c && c == d => HandValues::FourOfAKind,
         [_, b, c, d, e] if b == c && c == d && d == e => HandValues::FourOfAKind,
         [a, b, c, d, e] if a == b && b == c && d == e => HandValues::FullHouse,
         [a, b, c, d, e] if a == b && c == d && d == e => HandValues::FullHouse,
         [a, b, c, _, _] if a == b && b == c => HandValues::ThreeOfAKind,
         [_, _, c, d, e] if c == d && d == e => HandValues::ThreeOfAKind,
         [_, b, c, d, _] if c == d && d == b => HandValues::ThreeOfAKind,
         [a, b, c, d, _] if a == b && c == d => HandValues::TwoPair,
         [a, b, _, d, e] if a == b && d == e => HandValues::TwoPair,
         [_, b, c, d, e] if c == b && d == e => HandValues::TwoPair,
         [a, b, _, _, _] if a == b => HandValues::OnePair,
         [_, b, c, _, _] if b == c => HandValues::OnePair,
         [_, _, c, d, _] if c == d => HandValues::OnePair,
         [_, _, _, d, e] if d == e => HandValues::OnePair,
         _ => HandValues::HighCard,
      }
   }
}

impl Eq for HandValues {}

impl PartialEq<Self> for HandValues {
   fn eq(&self, other: &Self) -> bool {
        (*self as u8) == (*other as u8)
   }
}

impl PartialOrd<Self> for HandValues {
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
   }
}

impl Ord for HandValues {
   fn cmp(&self, other: &Self) -> Ordering {
      (*self as u8).cmp(&(*other as u8))
   }
}

#[derive(Debug, Copy, Clone)]
struct Hand {
   cards: [char; 5],
   value: HandValues,
}

impl Hand {
    fn new(cards: [char; 5]) -> Hand {
        Hand { cards, value: HandValues::new(cards) }
    }

   fn get_value(&self) -> HandValues {
      self.value
   }
}

fn cmp_card(a: char, b: char) -> Ordering {
   let a = match a {
      'T' => 10,
      'J' => 1,
      'Q' => 12,
      'K' => 13,
      'A' => 14,
      _ => a.to_digit(10).unwrap(),
   };
   let b = match b {
      'T' => 10,
      'J' => 1,
      'Q' => 12,
      'K' => 13,
      'A' => 14,
      _ => b.to_digit(10).unwrap(),
   };
   a.cmp(&b)
}

impl Index<usize> for Hand {
   type Output = char;

   fn index(&self, index: usize) -> &Self::Output {
      &self.cards[index]
   }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
   fn eq(&self, other: &Self) -> bool {
      for i in 0..5 {
         if self[i] != other[i] {
            return false;
         }
      }
      true
   }
}

impl PartialOrd<Self> for Hand {
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
   }
}

impl Ord for Hand {
   fn cmp(&self, other: &Self) -> Ordering {
      if self.value == other.value {
         for i in 0..5 {
            if self[i] != other[i] {
               return cmp_card(self[i], other[i]);
            }
         }
         Ordering::Equal
      } else {
         self.value.cmp(&other.value)
      }
   }
}

fn read_hands(input: String) -> Vec<(Hand, u32)> {
   let mut hands: Vec<(Hand, u32)> = Vec::new();
   for line in input.lines() {
      let hand = Hand::new(
         line.chars().take(5).collect::<Vec<char>>().try_into().unwrap()
      );
      let bid = line.split(" ").last().unwrap().parse::<u32>().unwrap();
      hands.push((hand, bid));
   }
   hands
}

pub(crate) fn c1(input: String) -> String {
   let mut hands: Vec<(Hand, u32)> = read_hands(input);
   hands.sort_by(|a, b| a.0.cmp(&b.0));
   let mut sum = 0;
   for (place, hand) in hands.iter().enumerate() {
      let winnings = (place+1) * hand.1 as usize;
      println!("{:?} -> {:?}", hand, winnings);
      sum += winnings;
   }

   sum.to_string()
}

pub(crate) fn c2(input: String) -> String {
   let mut hands: Vec<(Hand, u32)> = read_hands(input);
   hands.sort_by(|a, b| a.0.cmp(&b.0));
   let mut sum = 0;
   for (place, hand) in hands.iter().enumerate() {
      let winnings = (place+1) * hand.1 as usize;
      println!("{:?} -> {:?}", hand, winnings);
      sum += winnings;
   }

   sum.to_string()}


#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test1() {
      let hand = Hand::new(['2', '2', '2', '2', '2']);
      print!("{:?}", hand.get_value());
      assert_eq!(hand.get_value(), HandValues::FiveOfAKind);

   }

   #[test]
   fn test_hand_values() {
      let hand = Hand::new(['2', '2', '2', '2', '2']);
      assert_eq!(hand.get_value(), HandValues::FiveOfAKind);
      let hand = Hand::new(['2', '2', '2', '2', '3']);
      assert_eq!(hand.get_value(), HandValues::FourOfAKind);
      let hand = Hand::new(['2', '2', '2', '3', '3']);
      assert_eq!(hand.get_value(), HandValues::FullHouse);
      let hand = Hand::new(['2', '2', '2', '3', '4']);
      assert_eq!(hand.get_value(), HandValues::ThreeOfAKind);
      let hand = Hand::new(['2', '2', '3', '3', '4']);
      assert_eq!(hand.get_value(), HandValues::TwoPair);
      let hand = Hand::new(['2', '2', '3', '4', '5']);
      assert_eq!(hand.get_value(), HandValues::OnePair);
      let hand = Hand::new(['2', '3', '4', '5', '6']);
      assert_eq!(hand.get_value(), HandValues::HighCard);
   }
}