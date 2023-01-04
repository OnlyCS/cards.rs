use crate::blackjack::BlackJack;
use crate::game::Game;
use crate::ui::{console_clear, header_end, header_start, hprompt, prompt, prompt_options, Option};
use crate::war::War;

pub mod blackjack;
pub mod card;
pub mod game;
pub mod player;
pub mod ui;
pub mod war;

fn main() {
    console_clear();

    let game_id = prompt_options(
        "Which game to play?",
        &[
            Option {
                name: "War".to_string(),
                value: 1,
            },
            Option {
                name: "Blackjack".to_string(),
                value: 2,
            },
        ],
    );

    let max_players = hprompt("How many players?");

    let mut game: Box<dyn Game> = match game_id {
        1 => Box::new(War::new(max_players.trim().parse().unwrap())),
        2 => Box::new(BlackJack::new(max_players.trim().parse().unwrap())),
        _ => panic!("E_INVALID_GAME"),
    };

    header_start();
    println!("Players:");

    for player in game.get_players() {
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
