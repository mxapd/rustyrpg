use crate::Character;
use crate::Enemy;
use std::io;
use crate:: utility;
use rand::Rng;

pub trait Combatant{
    fn calc_damage(&self) -> u32;
    fn take_damage(&mut self, damage:u32);
    fn health(&self) -> u32;
    fn name(&self) -> &str;
}

impl Combatant for Character {
    fn calc_damage(&self) -> u32{
        (self.weapon.damage as f64 * self.weapon_skill).round() as u32
    }
    
    fn take_damage(&mut self, damage: u32){
        if damage > self.health {
            self.health = 0;
        } else {
            self.health -= damage;
        }
    }

    fn health(&self) -> u32 {
        self.health
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Combatant for Enemy {
    fn calc_damage(&self) -> u32{
        (self.weapon.damage as f64 * self.weapon_skill).round() as u32
    }

    fn take_damage(&mut self, damage: u32){
        if damage > self.health {
            self.health = 0;
        } else {
            self.health -= damage;
        }
    }

    fn health(&self) -> u32 {
        self.health
    }

    fn name(&self) -> &str {
        &self.name
    }
}

pub fn attack(attacker: &mut dyn Combatant, defender: &mut dyn Combatant){
    let mut damage = rand::thread_rng().gen_range((attacker.calc_damage()/2)..=attacker.calc_damage());
    
    let crit_chance: u32 = 100;
    let crit_roll = rand::thread_rng().gen_range(0..=100);
    let mut crit_hit = false;
    if crit_roll < crit_chance{
        crit_hit = true;
        damage = damage *2;
    }

    defender.take_damage(damage);
    utility::clear_console();
    if crit_hit{
        println!("Critical hit!")
    }
    println!("{} hits {} for {} damage", attacker.name(), defender.name(), damage);

    if defender.health() == 0 {
        println!("{} is defeated!", defender.name());
    } else {
        println!("{} now has {} health left.", defender.name(), defender.health());
        println!("----------------------------------------");
    }

}

fn print_fight_information(player: &Character, enemy: &Enemy){
    println!(
        "Player:        vs         Enemy:\n\
         ----------------------------------------\n\
         Name:    {:<16} {:<16}\n\
         HP:      {:<16} {:<16}\n\
         Weapon:  {:<16} {:<16}\n\
         Skill:   {:<16.2} {:<16.2}\n\
         ----------------------------------------",
        player.name, enemy.name,
        player.health.to_string(), enemy.health.to_string(),
        player.weapon.name, enemy.weapon.name,
        player.weapon_skill, enemy.weapon_skill
    );
    
}

pub fn prefight(player: &mut Character){
    let mut bandit = Enemy::bandit();
    fight(player, &mut bandit);
}

pub fn fight(player: &mut Character, enemy: &mut Enemy){
    let mut fight_ongoing = true;

    while fight_ongoing {
        print_fight_information(player, enemy);
        
        println!("1) Attack");
        println!("5) Flee");
        
        let mut selection_string = String::new();
        io::stdin().read_line(&mut selection_string).expect("Failed to read line");
        let selection: u8 = selection_string.trim().parse().expect("Error parsing selection");
        match selection {
            1 => {
                attack(player, enemy);
                if enemy.health() == 0 {
                    println!("{} is dead.", enemy.name());
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    fight_ongoing = false;
                }
            }
            5 => {
                if attempt_flee(player.evasion) {
                    utility::clear_console();
                    println!("you get away");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    fight_ongoing = false;
                }
                else {       
                    utility::clear_console();

                    let damage = rand::thread_rng().gen_range((enemy.calc_damage()/2)..=enemy.calc_damage());
                    player.take_damage(damage);
                    println!("{} attacks you for {} damage while you fail to escape.",enemy.name(), damage);

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    fight(player, enemy);
                    utility::clear_console();
                }
            }
            _ => todo!(),
        }
    }
}

pub fn attempt_flee(evasion: u32) -> bool{
    let random_number = rand::thread_rng().gen_range(0..=100);
    random_number <= evasion 
}

