use core::fmt;

use crate::{card::{self, CardTools}, deck::{self, DeckTools}};
// I rally really don't want to do this one

// Node struct vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv

// // This makes it a bit difficult to know which cards are on the base and accessible, but there are ways.
struct TreeNode {
    par_left: Option<usize>,
    par_right: Option<usize>,
    child_left: Option<usize>,
    child_right: Option<usize>,
    card: Option<card::Card>
}

impl Default for TreeNode {
    fn default() -> Self {
        return TreeNode{
            par_left: None,
            par_right: None,
            child_left: None,
            child_right: None,
            card: None,
        }
    }
}




// Board struct vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv

// The parent-child scheme is as follows, index 0 is the parent of 1 and 2, 1 parents 3 and 4, 2 parents 4 and 5
// It's kinda complicated and I don't really like it but I think it's worth to be able to print easily for the moment
pub struct Board {
    pyramid: Vec<TreeNode>,
    draw_stack: deck::Deck,
    discard_stack: deck::Deck,
}

// Returns board instance with pyramid array full of initialized nodes, excluding the cards
impl Default for Board {
    fn default() -> Self {
        let mut default_board: Board = Board {
            pyramid: Vec::new(),
            draw_stack: deck::Deck::new_filled_and_shuffled(),  // Draw stack starts filled and shuffled
            discard_stack: deck::Deck::default(),   // Discard stack starts empty
        };

        // This will keep track of where we are in the pyramid
        let mut pyramid_indexer: usize = 0;
        
        // Layer 1 refers to the top of the pyramid, not the bottom
        // Using range 1-7 inlusive to count the layers as well as to represent the number of nodes in a given layer
        for layer in 1..=7 {
            
            // Loop number of times equal to the number of nodes in the layer
            for node in 0..layer {
                // Create Node and push it to Vec with default values
                default_board.pyramid.push(TreeNode::default());

                // The left and right nodes on a layer have one missing parent and need to be marked None
                // The top node is both left and right

                // Left node assignment
                if node == 0 {  // Leftmost node
                    default_board.pyramid[pyramid_indexer].par_left = None;
                } else {
                    // Left parent is equal to the current index - # of nodes on the layer, see \extra_files\Right_left_parent_assigning_scheme.png
                    default_board.pyramid[pyramid_indexer].par_left = Some(pyramid_indexer - layer);
                }
                
                // Right parent assignment
                if node == (layer - 1) {  // Rightmost node
                    default_board.pyramid[pyramid_indexer].par_right = None;
                } else {
                    // Right parent is equal to the current index - # of nodes on the layer + 1, see \extra_files\Right_left_parent_assigning_scheme.png
                    default_board.pyramid[pyramid_indexer].par_right = Some((pyramid_indexer + 1) - layer);
                }

                // Children assignment

                // Bottom layer has no children 
                if layer == 7 {
                    default_board.pyramid[pyramid_indexer].child_left = None;
                    default_board.pyramid[pyramid_indexer].child_right = None;
                }
                else {  // Not the bottom
                    // For left and right child assignment explanation see \extra_files\Right_left_child_assigning_scheme.png
                    default_board.pyramid[pyramid_indexer].child_left = Some(pyramid_indexer + layer);
                    default_board.pyramid[pyramid_indexer].child_right = Some(pyramid_indexer + layer + 1);
                }

                // Increment pyramid indexer when a node is done being edited
                pyramid_indexer += 1;
            }
        }
        // All nodes are created, fill the card spots using the fill method
        let dropped_stack = default_board.draw_stack.drop_stack(28);
        let _ = default_board.fill_pyramid(dropped_stack);

        return default_board;
    }
}

// Used as temporary display of pyramid structure
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        // Controlling variables #############################

        // Character or string used for spacing
        let spacing_symbol: &str = " ";

        // Number of spaces between each number
        let spacing_num: usize = 3;

        // Empty card symbol
        let missing_card_string: &str = " ";

        // Derived variables #################################

        let spacing_str: &str = &spacing_symbol.repeat(spacing_num);

        // Half of spcaing num. Rounds up
        let half_spacing_num: usize = spacing_num.div_ceil(2);

        let half_spacing_str: &str = &spacing_symbol.repeat(half_spacing_num);

        // ###################################################

        // Create string to give to write!() at the end
        let mut pyramid_string: String = String::new();

        // Similar operation to default board creation

        // This will keep track of where we are in the pyramid
        let mut pyramid_indexer: usize = 0;

        // Layer 1 refers to the top of the pyramid, not the bottom
        // Using range 1-7 inlusive to count the layers as well as to represent the number of nodes in a given layer
        for layer in 1..=7 {

            // All even layers need an exrta half spacing to offset the numbers
            // If layer is even
            if (layer % 2) == 0 {
                pyramid_string.push_str(&half_spacing_str);
            }
            else { // Add a space to every odd layer except the last
                // if layer == 7 {}
                // else {
                //     pyramid_string.push_str(&spacing_symbol);
                // }
            }

            // Add spaces necessary spaces to align numbers
            pyramid_string.push_str(&spacing_str.repeat((7 - layer) / 2));
            pyramid_string.push_str(&spacing_symbol.repeat((7 - layer) / 2));


            // Add numbers and spcaing between numbers
            // Loop number of times equal to the number of nodes in the layer
            for node in 0..layer {

                // Start spacing the numbers only after the first
                if node == 0 {}
                else {
                    pyramid_string.push_str(&spacing_str);
                }

                // Add card to the string
                match &self.pyramid[pyramid_indexer].card {
                    // If there is a card, add its symbol
                    Some(valid_card) => pyramid_string.push_str(<card::Card as Clone>::clone(&valid_card).get_symbol().as_str()),
                    // If not, add the missing card symbol
                    None => pyramid_string.push_str(&missing_card_string)
                }

                // Increment indexer
                pyramid_indexer += 1;
            }

            // Adding draw and discard stacks
            if layer == 1 {
                pyramid_string.push_str(&spacing_str.repeat((7 - layer) / 2));
                pyramid_string.push_str(
                    format!("{}   {}",
                    self.draw_stack.top_card_symbol(),
                    self.discard_stack.top_card_symbol()).as_str()
                );
            }
            else {
                {}
            }
            
            // Add a newline before going to next layer of the pyramid
            pyramid_string.push('\n');
        }

        return write!(f, "{}", pyramid_string);
    }
}

pub trait BoardTools {
    fn fill_pyramid(&mut self, input_vec:Vec<card::Card>) -> Result<String, String>;
    fn match_card_pair(&mut self);
    fn kill_node(&mut self, pyramid_index: usize);
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

    // Needs to take two arguments for card locations
    // Kings are worth 13 and don't need a pair, they just leave
    // Chosen cards can come from the base of the pyramid, the top of th edraw pile, or the top of the discard pile
    fn match_card_pair(&mut self) {

    }

    // Function that takes pyramid index as input and removes the card from the node and the node from the family structure
    fn kill_node(&mut self, pyramid_index: usize) {
        // Clear card
        self.pyramid[pyramid_index].card = None;

        // Left parent forgets it's right child
        let left_parent = self.pyramid[pyramid_index].par_left;
        match left_parent {
            Some(valid_index) => self.pyramid[valid_index].child_right = None,
            None => {}
        }

        // Right parent forgets it's left child
        let right_parent = self.pyramid[pyramid_index].par_right;
        match right_parent {
            Some(valid_index) => self.pyramid[valid_index].child_left = None,
            None => {}
        }

        // Node forgets children. Shouldn't be necessary but ¯\_(ツ)_/¯
        // self.pyramid[pyramid_index].child_left = None;
        // self.pyramid[pyramid_index].child_right = None;

        // Node forgets parents. Definately isn't necessary but ¯\_(ツ)_/¯
        // self.pyramid[pyramid_index].par_left = None;
        // self.pyramid[pyramid_index].par_right = None;
    }
}