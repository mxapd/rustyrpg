mod character;
mod enemy;
mod items;
mod combat;
mod game_state;
mod utility;

use std::time::Duration;
use std::io::Write;
use std::thread;
use std::fs;
use std::io;
use std::process;
use character::Character;
use enemy::Enemy;
use game_state::GameState;
use utility::player_setup;

fn main() {
    loop {
        let game_state_option = main_menu();

        if let Some(mut game_state) = game_state_option {
            loop {
                utility::clear_console();
                
                let mut file_contents = match fs::read_to_string("story/intro.txt"){
                    Ok(contents) => contents,
                    Err(e) => {
                        eprintln!("Error reading file: {}", e);
                        return;
                    } 
                };

                file_contents = match &game_state.player.weapon.name[..] {
                    "Sword" => file_contents.replace("sword", "sword"), // Modify based on the player's weapon name
                    "Spear" => file_contents.replace("sword", "spear"), // Modify based on the player's weapon name
                    _ => file_contents,
                };

                let paragraphs: Vec<&str> = file_contents.split("\n\n").collect(); 
                
                for paragraph in paragraphs {
                    print_slowly(paragraph);
                    println!("");
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("failed to get input");
                }

                let mut torch_aquired = false;
                let mut tree_found = false;
                while  torch_aquired == false{
                    
                    print_slowly("What will you do?\n");
                    print_slowly("[1] Inspect your supplies\n");
                    print_slowly("[2] Feel around\n");
                    print_slowly("[3] Go blindly forwards\n");
                    
                    let mut input: String = String::new();
                    io::stdin()
                    .read_line(&mut input)
                    .expect("failed to get input");
                
                    let input: u32 = input.trim().parse().expect("error");

                    match input {
                        1 => {
                            print_slowly("Though darkness envelops the surroundings, your experience as a seasoned soldier and survivalist offers a glimmer of hope. Among your supplies lie tinder and flint, essential tools for igniting a fire. Yet, sparking the tinder without any firewood would be a futile endeavor\n\n");
                            wait_for_input();
                        }
                        2 => {
                            if tree_found {
                                print_slowly("You fumble along the rugged terrain, searching for anything of use, and find a large branch perfect as firewood or a makeshift torch.\n\n");
                                wait_for_input();   
                                torch_aquired = true;
                            }
                            else {
                                print_slowly("You fumble along the rugged terrain, searching for anything of use, but your efforts yeild naught; only dust and stones meet your touch.\n\n");
                                wait_for_input();
                            }
                        }
                        3 => {
                            print_slowly("You cautiously inch forward in the darkness, methodically exploring your surroundings with each movement. Desperately seeking something to guide your path.\n");
                            print_slowly("...\n");
                            print_slowly("After a few minutes, your hands meet some large object. As you touch it and feel what it might be, a subtle recognition dawns upon you. The rough texture beneath your fingertips, the sturdy yet yielding presence—it's unmistakably a tree.\n\n");
                            wait_for_input(); 
                            tree_found = true;
                        }
                        _ => todo!(),
                    }

                }                
            }

        }
    }
}
fn print_slowly(text: &str) {
    let delay_ms = 0;
    for c in text.chars() {
        print!("{}", c);
        std::io::stdout().flush().expect("Failed to flush stdout");
        thread::sleep(Duration::from_millis(delay_ms));
    }
}
fn wait_for_input(){
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("failed to get input");
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
    utility::clear_console();
    let mut player = player_setup();
    let mut game_state = GameState::new(player);
    game_state
}
fn quit(){
    println!("Quitting game..");
    process::exit(0);
}