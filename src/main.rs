use std::default;

use crate::board::BoardTools;

mod deck;
mod card;
mod board;

fn main() {

    // Testing board functionality
    let mut new_board: board::Board = default::Default::default();
    println!("Board test:\n{}", new_board);

    // Test node_kill
    new_board.kill_node(0);
    new_board.kill_node(7);
    new_board.kill_node(27);
    println!("Board test:\n{}", new_board);
}
