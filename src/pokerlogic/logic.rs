use crate::pokerlogic::models::*;
use strum::IntoEnumIterator;
#[derive(Debug, Clone)]
pub struct NotFoundError;

pub fn create_deck() -> Deck {
    let mut cards = Vec::new();
    for rank in Rank::iter() {
        for suite in Suite::iter() {
            cards.push(Card { suite, rank });
        }
    }

    Deck { cards }
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
    if let Ok(ret) = fullhouse(&mut cards) {
        println!(
            "Found fullhouse!! {} {} {} {} {}",
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

pub fn poker(tmp_cards: &mut Vec<Card>) -> Result<PokerHand, NotFoundError> {
    let mut poker_hand = PokerHand {
        cards: Vec::new(),
        poker_hand: PokerHandEnum::Poker,
    };
    let mut cards = tmp_cards.clone();
    cards.dedup();

    // lets not use first three, we will get it in the loop with + 1
    let iter = cards.iter().skip(4);

    for (i, _el) in iter.enumerate().rev() {
        let mut found_poker = true;
        for idx in 0..4 {
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
pub fn fullhouse(tmp_cards: &mut Vec<Card>) -> Result<PokerHand, NotFoundError> {
    let mut poker_hand = PokerHand {
        cards: Vec::new(),
        poker_hand: PokerHandEnum::Full,
    };
    let mut cards = tmp_cards.clone();
    let mut three_result = three(&mut cards)?;

    let iter = tmp_cards.iter().skip(1);

    for (i, _el) in iter.enumerate().rev() {
        if tmp_cards[i].rank.value() == tmp_cards[i + 1].rank.value()
            && tmp_cards[i].rank.value() != three_result.cards[0].rank.value()
        {
            poker_hand.cards.push(tmp_cards[i].clone());
            poker_hand.cards.push(tmp_cards[i + 1].clone());
            poker_hand.cards.append(&mut three_result.cards);
            return Ok(poker_hand);
        }
    }

    Err(NotFoundError)
}
pub fn flush(tmp_cards: &mut Vec<Card>) -> Result<PokerHand, NotFoundError> {
    let mut poker_hand = PokerHand {
        cards: Vec::new(),
        poker_hand: PokerHandEnum::Flush,
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

    // lets not use first four, we will get it in the loop with + n
    let iter = cards.iter().skip(4);
    if cards[cards.len() - 1].rank.value() == Rank::Ace.value()
        && cards[3].rank.value() == Rank::Five.value()
    {
        for idx in 0..5 {
            poker_hand.cards.push(cards[idx].clone());
        }

        return Ok(poker_hand);
    }

    for (i, _el) in iter.enumerate().rev() {
        let mut found_straight = true;
        for idx in 0..4 {
            if !(cards[i + idx].rank.value() + 1 == cards[i + idx + 1].rank.value()) {
                found_straight = false;
                break;
            }
        }
        if found_straight {
            for idx in 0..5 {
                poker_hand.cards.push(cards[i + idx].clone());
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
    let iter = tmp_cards.iter().skip(2);

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
    let iter = tmp_cards.iter().skip(1);
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
