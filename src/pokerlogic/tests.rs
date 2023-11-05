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
}

#[test]
fn test_straight_wheel() {
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
            rank: Rank::Five,
            suite: Suite::Spade,
        },
    ];
    cards2.sort_by(|a, b| a.rank.value().cmp(&b.rank.value()));
    let x = straight(&mut cards2);
    assert_eq!(x.is_ok(), true);
}
#[test]
fn test_fullhouse() {
    let mut cards = vec![
        Card {
            rank: Rank::Two,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Two,
            suite: Suite::Spade,
        },
        Card {
            rank: Rank::Four,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Four,
            suite: Suite::Clubs,
        },
        Card {
            rank: Rank::Six,
            suite: Suite::Heart,
        },
        Card {
            rank: Rank::Four,
            suite: Suite::Clubs,
        },
        Card {
            rank: Rank::Eight,
            suite: Suite::Heart,
        },
    ];
    cards.sort_by(|a, b| a.rank.value().cmp(&b.rank.value()));
    let x = fullhouse(&mut cards);
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
