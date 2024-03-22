use std::io;

struct Player {
    name: String,
    background: String,
}

fn main() {
    main_menu();
}

fn main_menu() {
    println!("1) New Game");
    println!("2) Load Game");
    println!("3) Quit");

    new_game();
}
fn new_game() {
    println!("What is your name?");

    let mut player_name = String::new();

    io::stdin()
        .read_line(&mut player_name)
        .expect("failed to get name");
}
