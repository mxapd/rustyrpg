use crate::items::Weapon;

pub struct Character {
    pub name: String,
    
    pub max_health: u32,
    pub health: u32,
    pub weapon_skill: f64,
    pub evasion: u32,

    pub favor: i32,
    pub weapon: Weapon,
}

impl Character {
    pub fn new(name: String, max_health: u32, weapon_skill: f64, evasion: u32, favor: i32, weapon: Weapon) -> Character {
        Character {
            name,
            max_health,
            health: max_health,
            weapon_skill,
            evasion,
            favor,
            weapon,
        }
    }
}