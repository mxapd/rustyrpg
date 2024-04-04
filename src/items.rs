#[derive(Clone)]
pub enum Item{
    Weapon(Weapon),
}
#[derive(Clone)]
pub struct Weapon {
    pub name: String,
    pub description: String,
    pub damage: u16,
}
impl Weapon {
    pub fn spear() -> Weapon {
        Weapon {
            name: "Spear".to_string(),
            description: "A long spear.".to_string(),
            damage: 10,
        }
    }
    pub fn sword() -> Weapon {
        Weapon {
            name: "Longsword".to_string(),
            description: "An elaborate longsword.".to_string(),
            damage: 15,
        }
    }
    pub fn dagger() -> Weapon {
        Weapon{
            name: "Dagger".to_string(),
            description: "A sturdy dagger, useful for many things but not a very effective in combat.".to_string(),
            damage: 5,
        }
    }
}
pub struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    pub fn new(capacity: usize) -> Self {
        Inventory {
            items: Vec::with_capacity(capacity), // Use with_capacity to pre-allocate memory
        }
    }
    pub fn add_item(&mut self, item: Item) -> Result<(), &str> {
        if self.items.len() < self.items.capacity() {
            self.items.push(item);
            Ok(())
        } else {
            Err("Inventory is full")
        }
    }
}