use poker::{create_deck, get_best, Card, Player, Rank, Suite, Table};
use std::io::{self, Write};
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

    player1.cards.append(&mut table.cards);
    player2.cards.append(&mut table.cards);

    println!("Your cards");
    for card in player1.cards.iter() {
        print!("{} ", card);
    }
    println!("");
    get_best(&player1);

    //    let mut buffer = String::new();
    //    io::stdin().read_line(&mut buffer)?;

    Ok(())
}
