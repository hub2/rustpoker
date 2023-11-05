use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::fmt;
use strum::IntoEnumIterator;
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
    fn value(&self) -> i32 {
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
    cards: Vec<Card>,
}

pub fn create_deck() -> Deck {
    let mut cards = Vec::new();
    for rank in Rank::iter() {
        for suite in Suite::iter() {
            cards.push(Card { suite, rank });
        }
    }

    Deck { cards }
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

pub enum PokerHandEnum {
    Poker,
    Four,
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
            PokerHandEnum::Poker => 8,
            PokerHandEnum::Four => 7,
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
    poker_hand: PokerHandEnum,
    cards: Vec<Card>,
}

pub fn get_best(p: &Player) -> PokerHand {
    let mut cards = p.cards.clone();
    cards.sort_by(|a, b| a.rank.value().cmp(&b.rank.value()));
    if let Ok(ret) = poker(&mut cards) {
        println!(
            "Found poker!! {} {} {} {} {}",
            ret.cards[0], ret.cards[1], ret.cards[2], ret.cards[3], ret.cards[4]
        );
        return ret;
    }
    if let Ok(ret) = four(&mut cards) {
        println!(
            "Found four!! {} {} {} {}",
            ret.cards[0], ret.cards[1], ret.cards[2], ret.cards[3]
        );
        return ret;
    }
    if let Ok(ret) = flush(&mut cards) {
        println!(
            "Found flush!! {} {} {} {} {}",
            ret.cards[0], ret.cards[1], ret.cards[2], ret.cards[3], ret.cards[4]
        );
        return ret;
    }
    if let Ok(ret) = straight(&mut cards) {
        println!(
            "Found straight!! {} {} {} {} {}",
            ret.cards[0], ret.cards[1], ret.cards[2], ret.cards[3], ret.cards[4]
        );
        return ret;
    }
    if let Ok(ret) = three(&mut cards) {
        println!(
            "Found three!! {} {} {}",
            ret.cards[0], ret.cards[1], ret.cards[2]
        );
        return ret;
    }
    if let Ok(ret) = two_pair(&mut cards) {
        println!(
            "Found two pairs!! {} {}, {} {}",
            ret.cards[0], ret.cards[1], ret.cards[2], ret.cards[3]
        );
        return ret;
    }
    if let Ok(ret) = pair(&mut cards) {
        println!("Found pair!! {} {}", ret.cards[0], ret.cards[1]);
        return ret;
    }
    return PokerHand {
        poker_hand: PokerHandEnum::HighCard,
        cards: vec![cards.last().unwrap().clone()],
    };
}

#[derive(Debug, Clone)]
pub struct NotFoundError;

#[test]
fn test_straight() {
    let mut cards = vec![
        Card {
            rank: Rank::Two,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Three,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Four,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Five,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Six,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Seven,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Eight,
            suite: Suite::Heart,
        },
    ];
    cards.sort_by(|a, b| a.rank.value().cmp(&b.rank.value()));
    let x = straight(&mut cards);

    assert_eq!(x.is_ok(), true);
    let mut cards2 = vec![
        Card {
            rank: Rank::Ace,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Two,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Three,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Four,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Five,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Jack,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Eight,
            suite: Suite::Heart,
        },
    ];
    cards2.sort_by(|a, b| a.rank.value().cmp(&b.rank.value()));
    let x = straight(&mut cards2);
    assert_eq!(x.is_ok(), true);
}
#[test]
fn test_poker() {
    let mut cards = vec![
        Card {
            rank: Rank::Two,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Three,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Four,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Five,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Six,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Seven,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Eight,
            suite: Suite::Heart,
        },
    ];
    cards.sort_by(|a, b| a.rank.value().cmp(&b.rank.value()));
    let x = poker(&mut cards);
    assert_eq!(x.is_ok(), true);
}
pub fn poker(tmp_cards: &mut Vec<Card>) -> Result<PokerHand, NotFoundError> {
    let mut poker_hand = PokerHand {
        cards: Vec::new(),
        poker_hand: PokerHandEnum::Poker,
    };
    let mut cards = tmp_cards.clone();
    cards.dedup();

    // lets not use first three, we will get it in the loop with + 1
    let mut iter = cards.iter();
    iter.next();
    iter.next();
    iter.next();
    iter.next();

    for (i, _el) in iter.enumerate().rev() {
        let mut found_poker = true;
        for idx in 0..4 {
            println!("{}", i + idx);
            println!(
                "{} {}",
                tmp_cards[i + idx].rank.value(),
                tmp_cards[i + idx + 1].rank.value()
            );
            if !(tmp_cards[i + idx].rank.value() + 1 == tmp_cards[i + idx + 1].rank.value()
                && tmp_cards[i + idx].suite == tmp_cards[i + idx + 1].suite)
            {
                found_poker = false;
                break;
            }
        }
        if found_poker {
            for idx in 0..5 {
                poker_hand.cards.push(tmp_cards[i + idx].clone());
            }

            return Ok(poker_hand);
        }
    }

    Err(NotFoundError)
}

pub fn flush(tmp_cards: &mut Vec<Card>) -> Result<PokerHand, NotFoundError> {
    let mut poker_hand = PokerHand {
        cards: Vec::new(),
        poker_hand: PokerHandEnum::Pair,
    };
    for suite in Suite::iter() {
        let cards: Vec<Card> = tmp_cards
            .iter()
            .filter(|n| n.suite == suite)
            .cloned()
            .collect();
        let count = cards.len();
        if count >= 5 {
            poker_hand.cards = cards.iter().rev().take(5).cloned().collect();
            return Ok(poker_hand);
        }
    }

    Err(NotFoundError)
}

pub fn straight(tmp_cards: &mut Vec<Card>) -> Result<PokerHand, NotFoundError> {
    let mut poker_hand = PokerHand {
        cards: Vec::new(),
        poker_hand: PokerHandEnum::Straight,
    };
    let mut cards = tmp_cards.clone();
    cards.dedup();

    // lets not use first three, we will get it in the loop with + 1
    let mut iter = cards.iter();
    iter.next();
    iter.next();
    iter.next();
    iter.next();

    for (i, _el) in iter.enumerate().rev() {
        let mut found_straight = true;
        for idx in 0..4 {
            println!(
                "{} {}",
                tmp_cards[i + idx].rank.value(),
                tmp_cards[i + idx + 1].rank.value()
            );
            if !(tmp_cards[i + idx].rank.value() + 1 == tmp_cards[i + idx + 1].rank.value()) {
                found_straight = false;
                break;
            }
        }
        if found_straight {
            for idx in 0..5 {
                poker_hand.cards.push(tmp_cards[i + idx].clone());
            }

            return Ok(poker_hand);
        }
    }

    Err(NotFoundError)
}

pub fn four(tmp_cards: &mut Vec<Card>) -> Result<PokerHand, NotFoundError> {
    let mut poker_hand = PokerHand {
        cards: Vec::new(),
        poker_hand: PokerHandEnum::Four,
    };

    // lets not use first three, we will get it in the loop with + 1
    let mut iter = tmp_cards.iter();
    iter.next();
    iter.next();
    iter.next();

    for (i, _el) in iter.enumerate().rev() {
        if tmp_cards[i].rank.value() == tmp_cards[i + 1].rank.value()
            && tmp_cards[i + 1].rank.value() == tmp_cards[i + 2].rank.value()
            && tmp_cards[i + 2].rank.value() == tmp_cards[i + 3].rank.value()
        {
            poker_hand.cards.push(tmp_cards[i].clone());
            poker_hand.cards.push(tmp_cards[i + 1].clone());
            poker_hand.cards.push(tmp_cards[i + 2].clone());
            poker_hand.cards.push(tmp_cards[i + 3].clone());
            return Ok(poker_hand);
        }
    }

    Err(NotFoundError)
}

pub fn three(tmp_cards: &mut Vec<Card>) -> Result<PokerHand, NotFoundError> {
    let mut poker_hand = PokerHand {
        cards: Vec::new(),
        poker_hand: PokerHandEnum::Three,
    };

    // lets not use first two, we will get it in the loop with + 1
    let mut iter = tmp_cards.iter();
    iter.next();
    iter.next();

    for (i, _el) in iter.enumerate().rev() {
        if tmp_cards[i].rank.value() == tmp_cards[i + 1].rank.value()
            && tmp_cards[i + 1].rank.value() == tmp_cards[i + 2].rank.value()
        {
            poker_hand.cards.push(tmp_cards[i].clone());
            poker_hand.cards.push(tmp_cards[i + 1].clone());
            poker_hand.cards.push(tmp_cards[i + 2].clone());
            return Ok(poker_hand);
        }
    }

    Err(NotFoundError)
}
// this finds three as well, but we rely on checking order to not get bugs here
pub fn two_pair(tmp_cards: &mut Vec<Card>) -> Result<PokerHand, NotFoundError> {
    let mut poker_hand = PokerHand {
        cards: Vec::new(),
        poker_hand: PokerHandEnum::TwoPair,
    };

    // lets not use first one, we will get it in the loop with + 1
    let mut iter = tmp_cards.iter();
    iter.next();
    let mut found_one = false;

    let mut first_pair_rank = Rank::Two;

    for (i, _el) in iter.enumerate().rev() {
        if tmp_cards[i].rank.value() == tmp_cards[i + 1].rank.value() {
            if found_one && tmp_cards[i].rank.value() != first_pair_rank.value() {
                poker_hand.cards.push(tmp_cards[i].clone());
                poker_hand.cards.push(tmp_cards[i + 1].clone());
                return Ok(poker_hand);
            } else {
                poker_hand.cards.push(tmp_cards[i].clone());
                poker_hand.cards.push(tmp_cards[i + 1].clone());
                found_one = true;
                first_pair_rank = tmp_cards[i].rank
            }
        }
    }

    Err(NotFoundError)
}

pub fn pair(tmp_cards: &mut Vec<Card>) -> Result<PokerHand, NotFoundError> {
    let mut poker_hand = PokerHand {
        cards: Vec::new(),
        poker_hand: PokerHandEnum::Pair,
    };

    // lets not use first one, we will get it in the loop with + 1
    let mut iter = tmp_cards.iter();
    iter.next();

    for (i, _el) in iter.enumerate().rev() {
        if tmp_cards[i].rank.value() == tmp_cards[i + 1].rank.value() {
            poker_hand.cards.push(tmp_cards[i].clone());
            poker_hand.cards.push(tmp_cards[i + 1].clone());
            return Ok(poker_hand);
        }
    }

    Err(NotFoundError)
}
