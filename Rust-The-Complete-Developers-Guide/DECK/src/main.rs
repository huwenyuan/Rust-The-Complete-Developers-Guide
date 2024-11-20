use std::vec;

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    // associated function - not tied to an instance
    fn new() -> Self {
        let suits = ["Spades", "Hearts", "Diamonds"];
        let values = ["Two", "Three", "Four", "Ace"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // implicit return, no semicolon
        Deck { cards }
    }

    // method - tied to an instance
    fn shuffle(&mut self) {
        // shuffle the cards
    }
}

fn main() {
    let deck = Deck::new();
    println!("Cards: {:#?}", deck);
}
