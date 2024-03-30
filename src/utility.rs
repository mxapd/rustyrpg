use std::io;
use std::process::Command;
use crate::character::Character;
use crate::items::Weapon;

pub fn print_player_information(player: &Character) {
    println!(
        "Player Information:\n\
         ---------------------\n\
         Name:    {}\n\
         Favor:   {}\n\
         HP:      {}\n\
         Weapon:  {}\n\
         Skill:   {}\n\
         ----------------------",
        player.name,
        player.favor,
        player.max_health,
        player.weapon.name,
        player.weapon_skill
    );
}

pub fn player_setup() -> Character {
    let name = set_name();
    let background = set_background();
    match background.as_ref() {
        "Spearman" => {
            let max_health = 100;
            let weapon_skill = 1.5;
            let favor = 500;
            let weapon = Weapon::spear();
            let evasion = 50;
            let player = Character::new(name, max_health, weapon_skill, evasion, favor, weapon);
            return player;
        }
        &_ => {
            let max_health = 100;
            let weapon_skill = 1.5;
            let favor = 500;
            let weapon = Weapon::spear();
            let evasion = 50;
            let player = Character::new(name, max_health, weapon_skill, evasion, favor, weapon);
            return player;
        }
    }
}
pub fn set_background() -> String {
    let mut menu_selection = String::new();
    let mut background_chosen: bool = false; 
    let mut player_background = String::new(); 

    while !background_chosen {
        println!("Choose your background:");
        println!("1) Spearman");
        
        io::stdin()
        .read_line(&mut menu_selection)
        .expect("failed to selection");
        match menu_selection.trim().parse::<u8>() {
            Ok(num) => {
                match num {
                    1 => {
                        player_background = String::from("Spearman");
                        background_chosen = true;
                    }
                    _ => {
                        player_background = String::from("Spearman");
                        background_chosen = true;
                    }
                }
            }
            Err(_) => {
                println!("Invalid input, please enter a number.")
            }
        }
        menu_selection.clear();
    }
    player_background
}
pub fn set_name() -> String {
    let mut name = String::new();
    loop {
        println!("What is your name?");
        match io::stdin().read_line(&mut name){
            Ok(_) => {
                if !name.trim().is_empty(){
                    break name.trim().to_string();
                }
            }
            Err(_) => println!("Name cannot be empty."),
        }
    }
}
pub fn clear_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "cls"])
                .status()
                .unwrap();
    } else {
        Command::new("clear")
                .status()
                .unwrap();
    }
}