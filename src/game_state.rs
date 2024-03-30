use crate::character::Character;
pub struct GameState{
    pub player: Character,
}
impl GameState{
    pub fn new(player: Character) -> GameState {
        GameState{
            player,
        }
    } 
}