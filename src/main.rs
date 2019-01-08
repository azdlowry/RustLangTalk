#[derive(Debug)]
pub enum Suit {
    Diamonds,
    Hearts,
    Clubs,
    Spades,
}

#[derive(Debug)]
pub enum Rank {
    A,
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
}

#[derive(Debug)]
pub struct Card(Suit, Rank);

fn main() {
    let suit = Suit::Diamonds;
    let rank = Rank::R4;
    let card = Card(suit, rank);
    println!("{:?}", card);
}
