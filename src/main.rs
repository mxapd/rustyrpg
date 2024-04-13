mod character;
mod combat;
mod enemy;
mod game_state;
mod items;
mod utility;

use character::Character;
use enemy::Enemy;
use game_state::GameState;
use std::fs;
use std::io;
use std::process;
use utility::{player_setup, print_slowly, wait_for_input};

fn main() {
    loop {
        let game_state_option = main_menu();

        if let Some(mut game_state) = game_state_option {
            loop {
                utility::clear_console();

                if !game_state.torch_aquired{
                    print_intro(&game_state);
                }
                while !game_state.torch_aquired {
                    print_slowly("What will you do?\n");
                    print_slowly("[1] Inspect your supplies\n");
                    print_slowly("[2] Feel around\n");
                    print_slowly("[3] Go blindly forwards\n");

                    print_slowly("[9] Save and exit.");

                    let mut input: String = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("failed to get input");

                    let input = input.trim().parse::<u32>();

                    match input {
                        Ok(choice) => match choice {
                            1 => {
                                print_slowly("Though darkness envelops the surroundings, your experience as a seasoned soldier and survivalist offers a glimmer of hope. Among your supplies lie tinder and flint, essential tools for igniting a fire. Yet, sparking the tinder without any firewood would be a futile endeavor.\n\n");
                                wait_for_input();
                            }
                            2 => {
                                if game_state.tree_found {
                                    print_slowly("You fumble along rugged terrain, searching for anything of use, and find a large branch perfect as firewood or a makeshift torch.\n\n");
                                    wait_for_input();
                                    game_state.branch_aquired = true;
                                } else {
                                    print_slowly("You fumble along the wet, rugged terrain, searching for anything of use, but your efforts yeild naught; only moss and wet slime meet your touch.\n\n");
                                    wait_for_input();
                                }
                            }
                            3 => {
                                print_slowly("You cautiously inch forward in the darkness, methodically exploring your surroundings with each movement. Desperately seeking something to guide your path.\n");
                                print_slowly("...\n");
                                print_slowly("As you go the dampness gradually gives way to a drier, firmer terrain, and after a few minutes, your hands meet some kind of large object. As you touch it and feel what it might be, a subtle recognition dawns upon you. The rough texture beneath your fingertips, the sturdy yet yielding presence—it's unmistakably a tree.\n\n");
                                wait_for_input();
                                game_state.tree_found = true;
                            }
                            9 => {
                                save_game(&game_state);
                                quit();
                            }
                            _ => todo!(),
                        },
                        Err(_) => println!("Invalid input. Please enter a number."),
                    }
                    if game_state.branch_aquired {
                        print_slowly("You tear a strip of fabric from your cloak and bind it around the branch before igniting it.\n\n");
                        game_state.torch_aquired = true;
                    }
                }

                let file_contents = match fs::read_to_string("story/intro2.txt") {
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
                print_slowly("[9] Save and exit.");
                let mut input: String = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("failed to get input");

                let input = input.trim().parse::<u32>();

                match input {
                    Ok(choice) => match choice {
                        1 => {
                            print_slowly("As you cautiously approach the camp, the dim light reveals the outlines of two quadrupedal creatures, their size nearly matching your own. However, the darkness veils any additional features from sight.\n");
                            print_slowly("Their attention seems wholly consumed by the macabre endeavor - feasting upon the corpses of your fallen brothers-in-arms.\n");
                        }
                        2 => {
                            print_slowly("Although the likelihood that the sounds emanate from any of your fallen comrades is slim, you waste no time hesitating. With a sense of urgency, you rush forward, desperate to confirm your hopeful suspicions.\n");
                            print_slowly("As you charge forward, your ears are assaulted by a deafening screech, so piercing that it momentarily paralyzes you, forcing you to halt and shield your ears in agony.\n");
                            print_slowly("As the ringing subsides and you begin to regain your senses, you scan the surroundings for the source of the chilling noise. What you see next sends a shiver down your spine - the shadows writhe and contort, seemingly alive with movement.\n");
                            print_slowly("As your hearing gradually returns, the ground beneath you trembles with the unmistakable sound of a thousand thunderous footsteps, closing in on you with terrifying speed.\n");
                            print_slowly("The air grows thick with anticipation as the horde of unseen beasts draws closer, their hostile intent unmistakable and their numbers seemingly endless\n");
                            print_slowly("In the face of this overwhelming threat, you must make a swift decision. Will you stand your ground and prepare to face whatever horrors approach, or will you turn and flee, hoping to outrun the impending danger?\n\n");

                            print_slowly("[1] Hold your ground and ready yourself for battle.\n");
                            print_slowly("[2] Run for your life.\n");

                            let mut input: String = String::new();
                            io::stdin()
                                .read_line(&mut input)
                                .expect("failed to get input");

                            let input = input.trim().parse::<u32>();

                            match input {
                                Ok(choice) => match choice {
                                    1 => {
                                        println!(" not yet written");
                                    }
                                    2 => {
                                        println!(" not yet written");
                                    }
                                    _ => {
                                        println!("error");
                                    }
                                },
                                Err(_) => println!("error parsing"),
                            }
                        }
                        3 => {
                            print_slowly("");
                        }
                        9 => {
                            save_game(&game_state);
                            quit();
                    }
                        _ => todo!(),
                    },
                    Err(_) => println!("Invalid input. Please enter a number."),
                }
            }
        }
    }
}

fn print_intro(game_state: &GameState) {
    let mut file_contents = match fs::read_to_string("story/intro1.txt") {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    file_contents = match &game_state.player.background {
        character::Background::Swordsman => file_contents.replace("sword", "sword"), // Modify based on the player's weapon name
        character::Background::Spearman => file_contents.replace("sword", "spear"), // Modify based on the player's weapon name
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

fn main_menu() -> Option<GameState> {
    let mut menu_selection = String::new();
    println!(
        r"
    ( |-| ~|~ |-| () |\| | /\ |\|   ( |-| /? () |\| | ( |_ [-
    "
    );
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
        }
        2 => {
            let game_state = load_game();
            return Some(game_state.expect("somethign"));
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

fn serialize_game_state(game_state: &GameState) -> Result<String, serde_json::Error>{
    let game_state_json = serde_json::to_string(&game_state)?;
    println!("Game state JSON: {}", game_state_json);
    Ok(game_state_json)
}

fn save_game(game_state: &GameState){
    let game_state_json = serialize_game_state(&game_state).expect("error serializing");

    fs::write("save.txt", game_state_json).expect("Error writing file");
}

fn new_game() -> GameState {
    utility::clear_console();
    let mut player = player_setup();
    let mut game_state = GameState::new(player);
    game_state
}
fn quit() {
    println!("Quitting game..");
    process::exit(0);
}
fn load_game()-> Result<GameState,serde_json::Error>{
    let data = fs::read_to_string("save.txt").expect("error reading file");
    let mut game_state: GameState = serde_json::from_str(&data)?;
    Ok(game_state)
}