use core::fmt;

use crate::card;
use crate::card::CardTools;
use crate::deck;

// I am could implement the structure of the board using a sort of tree
// Each node will have a card attached to it as well as 2 children nodes.
// The odd part is that these nodes will have more than 1 parent.
// This is what implements the "one card covering 2 cards" functionality
// A card will only be uncovered and able to match if it has no children
// When a card is removed from the board, two nodes will need their children updated

// Structure heavily inspired by https://rusty-ferris.pages.dev/blog/binary-tree-sum-of-values/, thanks :)
// type TreeNodeRef = Rc<RefCell<TreeNode>>;

// struct TreeNode {
//     par_left: Option<TreeNodeRef>,
//     par_right: Option<TreeNodeRef>,
//     child_left: Option<TreeNodeRef>,
//     child_right: Option<TreeNodeRef>,
//     card: card::Card,


// }

// // Pyramid data structure described above
// pub struct Board {
//     root: TreeNode,

// }

// impl fmt::Display for Board {
//     fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
//         // This is going to create the whole board so it might get complicated

//     }
// }

// pub trait BoardTools {
//     fn fill_pyramid(&mut self, input_vec:Vec<Card>);
// }

// // 
// impl BoardTools for Board {
//     fn fill_pyramid(&mut self, input_vec:Vec<Card>) {

//     }
// }




// I rally really don't want to do this one

// I could also use a list or Vec. That way I could access any card in the tree at will which makes printing easier
// // This makes it a bit difficult to know which cards are on the base and accessible, but there are ways.
// struct TreeNode {
//     par_left: u8,
//     par_right: u8,
//     child_left: u8,
//     child_right: u8,
//     card: card::Card,
// }

// The parent-child scheme is as follows, index 0 is the parent of 1 and 2, 1 parents 3 and 4, 2 parents 4 and 5
// It's kinda complicated and I don't really like it but I think it's worth to be able to print easily for the moment
pub struct Board {
    pyramid: [Option<card::Card>; 28],
}

impl Default for Board {
    fn default() -> Self {
        const ARRAY_REPEAT_VALUE: Option<card::Card> = None;
        let def_board:Board = Board {
            pyramid: [ARRAY_REPEAT_VALUE; 28]
        };

        return def_board;
    }
}


// TODO Print carde symbol instead of full name
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, 
"              {}
             {} {}
            {} {} {}
           {} {} {} {}
          {} {} {} {} {}
         {} {} {} {} {} {}
        {} {} {} {} {} {} {}",

            // I hate this method of printing all these elements but I don't know how to make a loop for it
            // I am so sorry, but I'm sure there is a way to make a loop for it (could loop println! instead of using one huge write!)
            // The end goal is to implement a GUI which would handle the printing, so this is temporary (I'm coping)
            match self.pyramid[0].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[1].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[2].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[3].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[4].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[5].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[6].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[7].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[8].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[9].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[10].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[11].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[12].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[13].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[14].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[15].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[16].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[17].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[18].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[19].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[20].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[21].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[22].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[23].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[24].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[25].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[26].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[27].as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }
        )
    }
}