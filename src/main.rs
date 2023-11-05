use crate::pokerlogic::logic::*;
use std::io::{self};
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    println!("Hello boi, this is a poker game\n");
    println!("I'm gonna shuffle the carrrds and deal them to you and my beautiful AI friend that will kick ur a**.");
    thread::sleep(Duration::from_millis(100));
    println!("jk, he is trash");
    thread::sleep(Duration::from_millis(100));
    println!("...Ok, let's proceed");
    println!("Here are your cards:");

    let mut deck = create_deck();
    deck.shuffle();

    let mut player1 = Player {
        name: "You!".to_string(),
        cards: Vec::new(),
    };

    let mut player2 = Player {
        name: "AI!".to_string(),
        cards: Vec::new(),
    };

    let players = Vec::from([&mut player1, &mut player2]);

    for player in players {
        player.cards.push(deck.deal_one());
        player.cards.push(deck.deal_one());
    }

    println!("{} {}", player1.cards[0], player2.cards[1]);

    let mut table = Table { cards: Vec::new() };

    table.cards.push(deck.deal_one());
    table.cards.push(deck.deal_one());
    table.cards.push(deck.deal_one());

    table.cards.push(deck.deal_one());

    table.cards.push(deck.deal_one());

    player1.cards.append(&mut table.cards.clone());
    player2.cards.append(&mut table.cards.clone());

    println!("");
    println!("Your cards");
    for card in player1.cards.iter() {
        print!("{} ", card);
    }
    println!("");
    let mut best = get_best(&player1);
    println!("\n");
    println!("Opponent cards");
    for card in player2.cards.iter() {
        print!("{} ", card);
    }
    println!("");
    let mut best2 = get_best(&player2);
    println!("\n");
    println!("{} {}", best.poker_hand.value(), best2.poker_hand.value());

    if best.poker_hand.value() > best2.poker_hand.value() {
        println!("You won!");
    } else if best.poker_hand.value() < best2.poker_hand.value() {
        println!("You lost!");
    } else {
        best.cards
            .sort_by(|a, b| a.rank.value().cmp(&b.rank.value()));
        best2
            .cards
            .sort_by(|a, b| a.rank.value().cmp(&b.rank.value()));

        let player1_won: bool = best
            .cards
            .clone()
            .into_iter()
            .map(|x| x.rank.value())
            .zip(best2.cards.clone().into_iter().map(|x| x.rank.value()))
            .map(|(a, b)| a > b)
            .collect::<Vec<_>>()
            .contains(&true);
        if player1_won {
            println!("You won!");
        } else {
            let player2_won: bool = best
                .cards
                .into_iter()
                .map(|x| x.rank.value())
                .zip(best2.cards.into_iter().map(|x| x.rank.value()))
                .map(|(a, b)| a < b)
                .collect::<Vec<_>>()
                .contains(&true);
            if player2_won {
                println!("You lost!");
            } else {
                player1
                    .cards
                    .sort_by(|a, b| a.rank.value().cmp(&b.rank.value()));
                player2
                    .cards
                    .sort_by(|a, b| a.rank.value().cmp(&b.rank.value()));
                let player1_won: bool = player1
                    .cards
                    .into_iter()
                    .take(5)
                    .map(|x| x.rank.value())
                    .zip(player2.cards.into_iter().take(5).map(|x| x.rank.value()))
                    .map(|(a, b)| a > b)
                    .collect::<Vec<_>>()
                    .contains(&true);
                if player1_won {
                    println!("You won!");
                } else {
                    println!("You lost!");
                }
            }
        }
    }

    //    let mut buffer = String::new();
    //    io::stdin().read_line(&mut buffer)?;

    Ok(())
}
