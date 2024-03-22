use std::io;
use std::process;

struct Character {
    name: String,
    
    max_health: i32,
    weapon_skill: f32,
    
    favor: i32,
    weapon: Weapon,
}
impl Character {
    fn new(name: String, max_health: i32, weapon_skill: f32, favor: i32, weapon: Weapon) -> Character {
        Character {
            name,
            max_health,
            weapon_skill,
            favor,
            weapon,
        }
    }
    fn calc_damage(&self) -> u16{
        let calc_damage = self.weapon.damage as f32 * self.weapon_skill;
        calc_damage.round() as u16
    }
}
struct GameState{
    player: Character,
}
impl GameState{
    fn new(player: Character) -> GameState {
        GameState{
            player,
        }
    } 
}
struct Weapon{
    name: String,
    description: String,
    damage: u16,
}
impl Weapon{
    fn spear() -> Weapon {
        Weapon {
            name : "Spear".to_string(),
            description: "A long spear.".to_string(),
            damage: 10,
        }
    }
}

fn main() {
    loop {
        let game_state_option = main_menu();

        if let Some(game_state) = game_state_option {
            loop {
                let mut menu_selection = String::new();
        
                print_player_information(&game_state.player);
        
                println!("1) Fight | 2) Status");
                println!("3) Inventory | 4) Shop");
                println!("6) Quit");
                
                io::stdin()
                    .read_line(&mut menu_selection)
                    .expect("failed to get input");
        
                let menu_selection: u8 = menu_selection.trim().parse().expect("failed to parse");
        
                match menu_selection {
                    1 => println!("{}",game_state.player.calc_damage()),
                    2 => println!("2"),
                    6 => {
                        println!("Exiting to main menu..");
                        break;
                    }
                    _ => println!("other")
                }
            }
        }
    }
}

fn print_player_information(player: &Character) {
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

fn player_setup() -> Character {

    let name = set_name();
    let background = set_background();
    match background.as_ref() {
        "Spearman" => {
            let max_health = 100;
            let weapon_skill = 1.5;
            let favor = 500;
            let weapon = Weapon::spear();
            let player = Character::new(name, max_health, weapon_skill, favor, weapon);
            return player;
        }
        &_ => {
            let max_health = 100;
            let weapon_skill = 1.5;
            let favor = 500;
            let weapon = Weapon::spear();
            let player = Character::new(name, max_health, weapon_skill, favor, weapon);
            return player;
        }
    }
}
fn set_background() -> String {
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
fn set_name() -> String {
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

fn main_menu() -> Option<GameState>{
    let mut menu_selection = String::new();
    println!(r"
     _ __ _   _ ___| |_ _   _   _ __ _ __   __ _ 
    | '__| | | / __| __| | | | | '__| '_ \ / _` |
    | |  | |_| \__ \ |_| |_| | | |  | |_) | (_| |
    |_|   \__,_|___/\__|\__, | |_|  | .__/ \__, |
                        |___/       |_|    |___/ 
    _____________________________________________");
    println!("   |1. New Game");
    println!("   |2. Load Game");
    println!("   |3. Quit");

    io::stdin()
        .read_line(&mut menu_selection)
        .expect("failed to get input");

    let menu_selection: u8 = menu_selection.trim().parse().expect("failed to parse");

    match menu_selection {
        1 => {
            let game_state = new_game();
            return Some(game_state);
        },
        2 => {
            println!("not inplemented");
            None
        }
        3 => {
            quit();
            None
        }
        _ => {
            println!("unknown selection {menu_selection}");
            None
        }
    }
}
fn new_game() -> GameState {
    let game_state = GameState::new(player_setup());
    game_state
}
fn quit(){
    println!("Quitting game..");
    process::exit(0);
}