pub struct Weapon{
    pub name: String,
    pub description: String,
    pub damage: u16,
}
impl Weapon{
    pub fn spear() -> Weapon {
        Weapon {
            name : "Spear".to_string(),
            description: "A long spear.".to_string(),
            damage: 10,
        }
    }
    pub fn sword() -> Weapon {
        Weapon {
            name : "Longsword".to_string(),
            description: "An elaborate longsword.".to_string(),
            damage: 15,
        }
    }
}

// item enum, 
// bandages, healing cream?, 
// inventory struct