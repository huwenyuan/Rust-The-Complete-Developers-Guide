use std::vec;
use rand::{thread_rng, seq::SliceRandom};

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
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let cards = deck.deal(3);
    println!("Heares your hand: {:#?}", cards);
    println!("Heares your deck: {:#?}", deck);
}
