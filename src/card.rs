use core::fmt;

use rand::seq;


#[derive(Clone)]
#[derive(Debug)]
pub struct Card {
    name: String,       // Face cards will use thier normal name, capitolized
    symbol: String,     // Needs to be a string for the 10 card specifically
    value: u8,          // King == 13, Queen == 12, Jack == 11, Ace == 1, Jokers == 0
    suit: char,         // s == Spade, c == Club, d == Diamond, and h == Heart
    color: char,        // Could make this a bool e.g. False = Balck, True = Red
    ascii_art: String,  // Gotta get creative with this one, or find some online
}

impl Default for Card {
    fn default() -> Self {
        return Card {
            name: "Nameless".to_string(),
            symbol: 'z'.to_string(),
            value: u8::MAX,
            suit: 'z',
            color: 'z',
            ascii_art: "::: Nothing :::".to_string(),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Name: {}\nValue: {}\nSuit: {}\nColor: {}\nArt: {}",
                self.name, self.value, self.suit, self.color, self.ascii_art)
    }
}

// I want to make the struct method only accessible through functions becaue there are a lot of rules involved
// e.g. spades are black and hearts are red, I don't want a black heart on accident

pub trait CardTools {
    fn fill_card (&mut self, value:u8, suit:char);
    fn create_card (value:u8, suit:char) -> Option<Card>;
    fn get_name(self) -> String;
    fn get_symbol(self) -> String;
}

impl CardTools for Card {
    fn fill_card (&mut self, value:u8, suit:char) {
        self.value = value;

        match value{
            13 => { self.name = "King".to_string(); self.symbol = 'K'.to_string() },
            12 => { self.name = "Queen".to_string(); self.symbol = 'Q'.to_string() },
            11 => { self.name = "Jack".to_string(); self.symbol = 'J'.to_string() },
            10|9|8|7|6|5|4|3|2 => { self.name = value.to_string(); self.symbol = value.to_string() },
            1 => { self.name = "Ace".to_string(); self.symbol = 'A'.to_string() },
            0 => { self.name = "Joker".to_string(); self.symbol = 'X'.to_string() },
            _ => ()
        }

        self.suit = suit.to_ascii_lowercase();
        match self.suit{
            's' => {    // Spades
                self.color='b';
                self.name.push_str(" of Spades");
            }
            'c' => {    // Clubs
                self.color='b';
                self.name.push_str(" of Clubs");
            }
            'h' => {    // Hearts
                self.color='r';
                self.name.push_str(" of Hearts");
            }
            'd' => {    // Diamonds
                self.color='r';
                self.name.push_str(" of Diamonds");
            }
            _ => ()
        }
    }

    fn create_card (value:u8, suit:char) -> Option<Card> {
        let mut card:Card = Default::default();

        card.value = value;
        match value{
            13 => card.name = "King".to_string(),
            12 => card.name = "Queen".to_string(),
            11 => card.name = "Jack".to_string(),
            10|9|8|7|6|5|4|3|2 => card.name = value.to_string(),
            1 => card.name = "Ace".to_string(),
            0 => card.name = "Joker".to_string(),
            _ => return None
        }

        card.suit = suit.to_ascii_lowercase();
        match card.suit{
            's' => {    // Spades
                card.color='b';
                card.name.push_str(" of Spades");
            }
            'c' => {    // Clubs
                card.color='b';
                card.name.push_str(" of Clubs");
            }
            'h' => {    // Hearts
                card.color='r';
                card.name.push_str(" of Hearts");
            }
            'd' => {    // Diamonds
                card.color='r';
                card.name.push_str(" of Diamonds");
            }
            _ => return None
        }
        return Some(card);
    }

    fn get_name(self) -> String {
        return self.name;
    }

    fn get_symbol(self) -> String {
        return self.symbol;
    }
}