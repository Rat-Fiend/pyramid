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
}
