mod character;
mod enemy;
mod items;
mod combat;
mod game_state;
mod utility;

use std::fs;
use std::io;
use std::process;
use character::Character;
use enemy::Enemy;
use game_state::GameState;
use utility::{print_player_information, player_setup};

fn main() {
    loop {
        let game_state_option = main_menu();

        if let Some(mut game_state) = game_state_option {
            loop {
                utility::clear_console();
                
            }
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
    utility::clear_console();
    let mut player = player_setup();
    let mut game_state = GameState::new(player);
    game_state
}
fn quit(){
    println!("Quitting game..");
    process::exit(0);
}