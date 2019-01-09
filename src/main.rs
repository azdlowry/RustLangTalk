#![deny(warnings)]

use std::fmt::{self, Display};
use std::vec;

#[derive(Debug, Copy, Clone)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Copy, Clone)]
pub enum Rank {
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, Copy, Clone)]
pub struct Card(Suit, Rank);

#[derive(Debug)]
pub struct Hand(Card, Card, Card, Card, Card);

#[derive(Debug, PartialEq)]
pub enum HandRank {
    HighCard(Rank),
}

impl Hand {
    pub fn rank(&self) -> HandRank {
        HandRank::HighCard(
            vec![self.0, self.1, self.2, self.3, self.4]
                .iter()
                .map(|c| { c.1 })
                .max()
                .unwrap()
        )
    }
}

fn main() {
    let suit = Suit::Diamonds;
    let rank = Rank::R4;
    let card = Card(suit, rank);
    let hand = Hand(card, card, card, card, card);
    println!("{}", hand);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! card {
        ($r:ident, $s:ident) => (Card(Suit::$s, Rank::$r));
    }

    #[test]
    fn ranks_are_ordered() {
        assert!(Rank::R2 < Rank::R3);
        assert!(Rank::R3 < Rank::R4);
        assert!(Rank::R4 < Rank::R5);
        assert!(Rank::R5 < Rank::R6);
        assert!(Rank::R6 < Rank::R7);
        assert!(Rank::R7 < Rank::R8);
        assert!(Rank::R8 < Rank::R9);
        assert!(Rank::R9 < Rank::R10);
        assert!(Rank::R10 < Rank::J);
        assert!(Rank::J < Rank::Q);
        assert!(Rank::Q < Rank::K);
        assert!(Rank::K < Rank::A);
    }

    #[test]
    fn highcard_hands_have_rank_of_highest_card_in_hand() {
        let hand = Hand(
            card!(K, Clubs),
            card!(A, Clubs),
            card!(K, Clubs),
            card!(K, Clubs),
            card!(K, Clubs),
        );
        assert_eq!(HandRank::HighCard(Rank::A), hand.rank());

        let hand = Hand(
            card!(K, Clubs),
            card!(K, Clubs),
            card!(K, Clubs),
            card!(K, Clubs),
            card!(K, Clubs),
        );
        assert_eq!(HandRank::HighCard(Rank::K), hand.rank());

        let hand = Hand(
            card!(K, Clubs),
            card!(K, Clubs),
            card!(K, Clubs),
            card!(A, Clubs),
            card!(K, Clubs),
        );
        assert_eq!(HandRank::HighCard(Rank::A), hand.rank());
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
        })
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Rank::A => "A",
            Rank::R2 => "2",
            Rank::R3 => "3",
            Rank::R4 => "4",
            Rank::R5 => "5",
            Rank::R6 => "6",
            Rank::R7 => "7",
            Rank::R8 => "8",
            Rank::R9 => "9",
            Rank::R10 => "10",
            Rank::J => "J",
            Rank::Q => "Q",
            Rank::K => "K",
        })
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.1, self.0)
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}, {}, {}", self.0, self.1, self.2, self.3, self.4)
    }
}
