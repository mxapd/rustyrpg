use crate::character::Character;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub player: Character,
    pub tree_found: bool,
    pub branch_aquired: bool, 
    pub torch_aquired: bool,
}
impl GameState {
    pub fn new(player: Character) -> GameState {
        GameState { 
            player,
            tree_found: false,
            branch_aquired: false,
            torch_aquired: false,
        }
    }
}
