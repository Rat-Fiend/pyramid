use core::fmt;
use std::default;

use crate::card::{self, CardTools};
// I rally really don't want to do this one

// // This makes it a bit difficult to know which cards are on the base and accessible, but there are ways.
struct TreeNode {
    par_left: Option<usize>,
    par_right: Option<usize>,
    child_left: Option<usize>,
    child_right: Option<usize>,
    card: Option<card::Card>,
    is_empty: bool,
}

impl Default for TreeNode {
    fn default() -> Self {
        return TreeNode{
            par_left: None,
            par_right: None,
            child_left: None,
            child_right: None,
            card: None,
            is_empty: true,
        }
    }
}

// The parent-child scheme is as follows, index 0 is the parent of 1 and 2, 1 parents 3 and 4, 2 parents 4 and 5
// It's kinda complicated and I don't really like it but I think it's worth to be able to print easily for the moment
pub struct Board {
    pyramid: [TreeNode; 28], // TODO: I might need to implement this using a Vec becuase the array isn't well supported
    // TODO: I would do it now but I'm tired enough to start looking for hacky solutions, gotta stop
}

// Returns board instance with pyramid array full of 
impl Default for Board {
    fn default() -> Self {
        // Initialize board with default nodes
        const ARRAY_REPEAT_VALUE: TreeNode = default::Default::default();
        let mut def_board:Board = Board {
            pyramid: [ARRAY_REPEAT_VALUE; 28]
        };

        // Assign nodes with children and parent values, cards will be filled by fill_board()

        // This will keep track of where we are in the pyramid
        let mut pyramid_index: usize = 0;
        
        // Layer 1 refers to the top of the pyramid, not the bottom
        // Using range 1-7 inlusive to count the layers as well as to represent the number of nodes in a given layer
        for layer in 1..=7 {
            
            // Loop number of times equal to the number of nodes in the layer
            for node in 0..layer {
                // The left and right nodes on a layer have one missing parent and need to be marked None
                // The top node is both left and right

                // Left node assignment
                if node == 0 {  // Leftmost node
                    def_board.pyramid[pyramid_index].par_left = None;
                } else {
                    // Left parent is equal to the current index - # of nodes on the layer, see \extra_files\Right_left_parent_assigning_scheme.png
                    def_board.pyramid[pyramid_index].par_left = Some(pyramid_index - layer);
                }
                
                // Right parent assignment
                if node == (layer - 1) {  // Rightmost node
                    def_board.pyramid[pyramid_index].par_right = None;
                }
                else {
                    // Right parent is equal to the current index - # of nodes on the layer + 1, see \extra_files\Right_left_parent_assigning_scheme.png
                    def_board.pyramid[pyramid_index].par_right = Some(pyramid_index - layer + 1);
                }

                // Children assignment

                // Bottom layer has no children 
                if layer == 7 {
                    def_board.pyramid[pyramid_index].child_left = None;
                    def_board.pyramid[pyramid_index].child_right = None;
                }
                else {  // Not the bottom
                    // For left and right child assignment explanation see \extra_files\Right_left_child_assigning_scheme.png
                    def_board.pyramid[pyramid_index].child_left = Some(pyramid_index + layer);
                    def_board.pyramid[pyramid_index].child_right = Some(pyramid_index + layer + 1);
                }

                // Increment pyramid index when a card is done being edited
                pyramid_index += 1;
            }
        }
        return def_board;
    }
}

// Used as temporary display of pyramid structure
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
            match self.pyramid[0].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[1].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[2].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[3].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[4].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[5].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[6].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[7].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[8].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[9].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[10].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[11].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[12].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[13].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[14].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[15].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[16].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[17].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[18].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[19].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[20].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[21].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[22].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[23].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[24].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[25].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[26].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }, match self.pyramid[27].card.as_ref() { 
                Some(valid_card) => valid_card.clone().get_symbol(), None => 'E'.to_string()
            }
        )
    }
}

pub trait BoardTools {
    fn fill_pyramid(&mut self, input_vec:Vec<card::Card>) -> Result<String, String>;

}

// Takes a Vec of 28 cards and fills the array
impl BoardTools for Board {
    fn fill_pyramid(&mut self, input_vec:Vec<card::Card>) -> Result<String, String> {
        // Checking for the validity of the input Vec
        // It needs to be 28 long exactly
        if input_vec.len() != 28 {
            return Err("Invalid input Vec length :(".to_string());
        }
        else {
            let mut indexer: usize = 0;

            // This loops 28 times, guarenteed
            for input_card in input_vec {
                self.pyramid[indexer].card = Some(input_card);
                indexer += 1;
            }

            return Ok("Pyramid filled successfully :)".to_string());
        }
    }

}