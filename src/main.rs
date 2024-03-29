use std::default;

use deck::DeckTools;

mod deck;
mod card;
mod board;

fn main() {

    // Testing board functionality
    let new_board: board::Board = default::Default::default();
    println!("Board test:\n{}", new_board);
}
