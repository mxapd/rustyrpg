use crate::items::{Inventory, Weapon};

pub enum Background {
    Swordsman,
    Spearman,
}
pub struct Character {
    pub name: String,

    pub background: Background,
    pub max_health: u32,
    pub health: u32,
    pub weapon_skill: f64,
    pub evasion: u32,

    pub favor: i32,
    pub weapon: Weapon,
    pub inventory: Inventory,
}

impl Character {
    pub fn new(
        name: String,
        background: Background,
        max_health: u32,
        weapon_skill: f64,
        evasion: u32,
        favor: i32,
        weapon: Weapon,
        inventory: Inventory,
    ) -> Character {
        Character {
            name,
            background,
            max_health,
            health: max_health,
            weapon_skill,
            evasion,
            favor,
            weapon,
            inventory,
        }
    }
}
