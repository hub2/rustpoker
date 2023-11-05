use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use strum_macros::EnumIter;
#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Suite {
    Heart,
    Spade,
    Clubs,
    Diamonds,
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum Rank {
    Two,
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
impl Rank {
    pub fn value(&self) -> i32 {
        match *self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}

#[derive(Clone)]
pub struct Card {
    pub suite: Suite,
    pub rank: Rank,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank.value() == other.rank.value()
    }
}

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub cards: Vec<Card>,
}

pub struct Table {
    pub cards: Vec<Card>,
}

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
    pub fn deal_one(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.suite, self.rank)
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rank::Two => write!(f, "Two"),
            Rank::Three => write!(f, "Three"),
            Rank::Four => write!(f, "Four"),
            Rank::Five => write!(f, "Five"),
            Rank::Six => write!(f, "Six"),
            Rank::Seven => write!(f, "Seven"),
            Rank::Eight => write!(f, "Eight"),
            Rank::Nine => write!(f, "Nine"),
            Rank::Ten => write!(f, "Ten"),
            Rank::Jack => write!(f, "Jack"),
            Rank::Queen => write!(f, "Queen"),
            Rank::King => write!(f, "King"),
            Rank::Ace => write!(f, "Ace"),
        }
    }
}

impl fmt::Display for Suite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suite::Heart => write!(f, "Heart"),
            Suite::Spade => write!(f, "Spade"),
            Suite::Clubs => write!(f, "Clubs"),
            Suite::Diamonds => write!(f, "Diamonds"),
        }
    }
}

//fn compare_players(p1: &Player, p2: &Player) -> &Player {}

#[derive(Clone)]
pub enum PokerHandEnum {
    Poker,
    Four,
    Flush,
    Full,
    Straight,
    Three,
    TwoPair,
    Pair,
    HighCard,
}

impl PokerHandEnum {
    pub fn value(&self) -> i32 {
        match *self {
            PokerHandEnum::Poker => 9,
            PokerHandEnum::Four => 8,
            PokerHandEnum::Flush => 7,
            PokerHandEnum::Full => 6,
            PokerHandEnum::Straight => 5,
            PokerHandEnum::Three => 4,
            PokerHandEnum::TwoPair => 3,
            PokerHandEnum::Pair => 2,
            PokerHandEnum::HighCard => 1,
        }
    }
}

pub struct PokerHand {
    pub poker_hand: PokerHandEnum,
    pub cards: Vec<Card>,
}
