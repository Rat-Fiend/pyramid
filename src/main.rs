use std::default;

use deck::DeckTools;

use crate::board::BoardTools;

mod deck;
mod card;
mod board;

fn main() {

    // Testing board functionality
    let mut new_board: board::Board = default::Default::default();
    println!("Board test:\n{}", new_board);

    let mut new_deck: deck::Deck = default::Default::default();
    new_deck.fill_deck();
    new_deck.shuffle();

    // Testing board fill method
    _ = new_board.fill_pyramid(new_deck.drop_stack(52-28, 52));
    println!("Filled board test:\n{}", new_board);
}
