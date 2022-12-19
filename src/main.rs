use std::io::{self, Write};

use crate::game::Game;
use crate::ui::console_clear;

pub mod card;
pub mod game;
pub mod player;
pub mod ui;

fn main() {
    console_clear();

    let mut max_players = String::new();

    print!("How many players? ");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut max_players).unwrap();

	console_clear();

    let mut game = Game::new(max_players.trim().parse().unwrap());

    console_clear();
    println!("{}\n\nPlayers:", "-".to_string().repeat(20));

    for player in &game.players {
        println!("{}", player.name);
    }

    println!("\n{} total", max_players.trim().parse::<i32>().unwrap());
    println!("\n{}\nPress enter to continue", "-".to_string().repeat(20));

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();

    console_clear();
    game.round();
}
