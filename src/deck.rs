use core::fmt;
use std::default;
use rand::Rng;
use crate::card::{self as card, CardTools};

#[derive(Debug)]

pub struct Deck {
    cards:Vec<card::Card>,
}

// Returns a deck instance with an empty cards Vec
// Could also default to a 52 card shuffled dec in wanted
impl Default for Deck {
    fn default() -> Self {
        return Deck {
            cards: Vec::new(),
        }
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for card in &self.cards {
            // TODO maybe implement a way to only print the name of the card when printing the whole deck
            println!("{}\n", card);
        }
        Ok(())
    }
}

pub trait DeckTools {
    fn card_count(&self) -> usize;
    fn add_stack(&mut self, stack: Vec<card::Card>);
    fn drop_stack(&mut self, drop_size: usize) -> Vec<card::Card>;
    fn fill_deck(&mut self);
    fn shuffle(&mut self);
    fn new_filled_and_shuffled() -> Self;

}
impl DeckTools for Deck {
    fn card_count(&self) -> usize {
        return self.cards.len();
    }

    fn add_stack(&mut self, mut stack: Vec<card::Card>) {
        self.cards.append(&mut stack);
    }

    fn drop_stack(&mut self, drop_size: usize) -> Vec<card::Card> {
        // Should account for the case where you try to drop more than the Vec contains
        return  self.cards.drain((self.card_count() - drop_size)..self.card_count()).collect::<Vec<card::Card>>();
    }

    fn fill_deck(&mut self) {
        let suits = ['h', 'd', 's', 'c'];

        for suit in suits {
            for val in 1..14 {
                match card::Card::create_card(val, suit) {
                    Some(x) => { self.cards.push(x); },

                    None => { println!("Please make this an error message, idk how :(\n
                        fill_deck attempted to create an invalid card, shame on you");
                        break;
                    }
                }
                
            }
        }

    }

    // Implements using Vec.swap(), there are other options though
    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        let card_count = self.card_count();

        // Using 10000 as the number of times to swap cards around, should be good enough
        // A decent number of these will try to swap to the same position which is inefficient, but whatever
        for _ in 0..10000 {
            self.cards.swap(rng.gen_range(0..card_count), rng.gen_range(0..card_count));
        }
    }

    fn new_filled_and_shuffled() -> Self {
        let mut new_deck: Deck = default::Default::default();
        new_deck.fill_deck();
        new_deck.shuffle();
        return new_deck;
    }
}