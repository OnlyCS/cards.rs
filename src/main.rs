use crate::war::War;
use crate::ui::{console_clear, hprompt, header_end, header_start, prompt};

pub mod card;
pub mod war;
pub mod player;
pub mod ui;

fn main() {
    console_clear();

    let max_players = hprompt("How many players?");
    let mut game = War::new(max_players.trim().parse().unwrap());

    header_start();
    println!("Players:");

    for player in &game.players {
        println!("{}", player.name);
    }

    println!("\n{} total", max_players.trim().parse::<i32>().unwrap());
    header_end();
    prompt("Press enter to continue...");

    console_clear();

    loop {
        game.round();
        console_clear();
    }
}
