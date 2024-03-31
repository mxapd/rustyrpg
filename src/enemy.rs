use crate::items::Weapon;

pub struct Enemy {
    pub name: String,

    pub max_health: u32,
    pub health: u32,
    pub weapon_skill: f64,

    pub favor: i32,
    pub weapon: Weapon,
}
impl Enemy {
    pub fn bandit() -> Enemy {
        Enemy {
            name: "Bandit".to_string(),
            max_health: 100,
            health: 100,
            weapon_skill: 1.0,
            favor: 15,
            weapon: Weapon::spear(),
        }
    }
}
