use std::default;

use deck::DeckTools;

mod deck;
mod card;

fn main() {

    // let mut value_result = Default::default();
    // let mut suit_result = Default::default();
    // let new_card:card::Card;
    // Little loop for getting input and testing card validity
    // loop {
        // Getting user input
        // println!("Please enter the value of the card (0-13)");
        // let _ = io::stdin().read_line(&mut value_result);
        // println!("Please enter the suit of the card (h for heart, s for spade, etc.)");
        // let _ = io::stdin().read_line(&mut suit_result);
        
        // print!("Entered value and suit: {}, {}", value_result, suit_result);

        // let result: card::Card;
        // result.fill_card(value_result, suit_result);
        // match result {
        //     Some(x) => {
        //         new_card = x;
        //         println!("Your new card: {}", new_card);
        //         break;
        //     }
        //     None => println!("That is not a valid card!"),
        // }
    


    let mut new_deck: deck::Deck = default::Default::default();
    println!("Empty deck: {}\nDone.", new_deck);
    new_deck.fill_deck();
    println!("Initialized deck: {}\nDone.", new_deck);
    new_deck.shuffle();
    println!("Shuffled deck: {}\nDone.", new_deck);
}
