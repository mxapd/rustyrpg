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
use utility::{player_setup, print_slowly, wait_for_input};

fn main() {
    loop {
        let game_state_option = main_menu();

        if let Some(mut game_state) = game_state_option {
            loop {
                utility::clear_console();
                
                print_intro(&game_state);

                let mut branch_aquired = false;
                let mut torch_aquired = false;
                let mut tree_found = false;

                while !torch_aquired{
                    print_slowly("What will you do?\n");
                    print_slowly("[1] Inspect your supplies\n");
                    print_slowly("[2] Feel around\n");
                    print_slowly("[3] Go blindly forwards\n");
                    
                    let mut input: String = String::new();
                    io::stdin().read_line(&mut input).expect("failed to get input");
                
                    let input = input.trim().parse::<u32>();

                    match input {
                        Ok(choice) => match choice {
                            1 => {
                                print_slowly("Though darkness envelops the surroundings, your experience as a seasoned soldier and survivalist offers a glimmer of hope. Among your supplies lie tinder and flint, essential tools for igniting a fire. Yet, sparking the tinder without any firewood would be a futile endeavor.\n\n");
                                wait_for_input();
                            }
                            2 => {
                                if tree_found {
                                    print_slowly("You fumble along rugged terrain, searching for anything of use, and find a large branch perfect as firewood or a makeshift torch.\n\n");
                                    wait_for_input();   
                                    branch_aquired = true;
                                }
                                else {
                                    print_slowly("You fumble along the wet, rugged terrain, searching for anything of use, but your efforts yeild naught; only moss and wet slime meet your touch.\n\n");
                                    wait_for_input();
                                }
                            }
                            3 => {
                                print_slowly("You cautiously inch forward in the darkness, methodically exploring your surroundings with each movement. Desperately seeking something to guide your path.\n");
                                print_slowly("...\n");
                                print_slowly("As you go the dampness gradually gives way to a drier, firmer terrain, and after a few minutes, your hands meet some kind of large object. As you touch it and feel what it might be, a subtle recognition dawns upon you. The rough texture beneath your fingertips, the sturdy yet yielding presence—it's unmistakably a tree.\n\n");
                                wait_for_input(); 
                                tree_found = true;
                            }
                            _ => todo!(),
                        },
                        Err(_) => println!("Invalid input. Please enter a number."),
                    }
                    if branch_aquired {
                        print_slowly("You tear a strip of fabric from your cloak and bind it around the branch before igniting it.\n\n");
                        torch_aquired = true;
                    }
                } 

                let file_contents = match fs::read_to_string("story/intro2.txt"){
                    Ok(contents) => contents,
                    Err(e) => {
                        eprintln!("Error reading file: {}", e);
                        return;
                    } 
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
                print_slowly("What will you do?\n");
                print_slowly("[1] Aproach the camp carefully, taking care to not be noticed. You dont know what awaits ahead.\n");
                print_slowly("[2] Rush in to see if any of your friends are still alive.\n");
                print_slowly("[3] Look around for additional options\n");

                let mut input: String = String::new();
                io::stdin().read_line(&mut input).expect("failed to get input");
            
                let input = input.trim().parse::<u32>();

                match input {
                    Ok(choice) => match choice {
                        1 => {
                            print_slowly("As you carefully aproach the camp you can see a couple of four-legged beasts, their height is slightly below your shoulders, its too dark to tell what they are.");
                            print_slowly("They seem to be busy eating.. ");
                            print_slowly(".. eating the corpses of what must be your fallen brothers-in-arms.");

                        }
                        2 => {
                            print_slowly("Even if the chances that the sounds are from any of your fallen friends are low, you dont wait around and run in to see if what you hope is true.");
                            print_slowly("As you run in, you see some kind of four-legged beast charging towards you. Better get ready to fight.");
                        }
                        3 => {
                            print_slowly("You look around for anything of use, but find nothing but sticks and stones..");
                        }
                        _ => todo!(),
                    },
                    Err(_) => println!("Invalid input. Please enter a number."),
                }
            }   
        }
    }
}

fn print_intro(game_state:&GameState){
    let mut file_contents = match fs::read_to_string("story/intro1.txt"){
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        } 
    };

    file_contents = match &game_state.player.background {
        character::Background::Swordsman => file_contents.replace("sword", "sword"), // Modify based on the player's weapon name
        character::Background::Spearman => file_contents.replace("sword", "spear"), // Modify based on the player's weapon name
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
}

fn main_menu() -> Option<GameState>{
    let mut menu_selection = String::new();
    println!(r"
    ( |-| ~|~ |-| () |\| | /\ |\|   ( |-| /? () |\| | ( |_ [-                                                                                                                                                                                                                                                                                           ");
    println!("   1. New Game");
    println!("   2. Load Game");
    println!("   3. Quit");

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