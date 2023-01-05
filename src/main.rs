use crate::blackjack::BlackJack;
use crate::game_common::{get_player_names, Game};
use crate::ui::*;
use crate::war::War;

pub mod blackjack;
pub mod card;
pub mod game_common;
pub mod player;
pub mod ui;
pub mod war;

fn main() {
    console_clear!();

    let game_id = prompt_options!(
        "Which game to play?",
        &[
            Option {
                name: "War",
                value: 1,
            },
            Option {
                name: "BlackJack",
                value: 2,
            },
        ]
    );

    console_clear!();
    let player_ct = prompt_headers!("How many players?").trim().parse().unwrap();
    let player_names = get_player_names(player_ct);

    let mut game: Box<dyn Game> = match game_id {
        1 => Box::new(War::new(player_names)),
        2 => Box::new(BlackJack::new(player_names)),
        _ => panic!("E_INVALID_GAME"),
    };

    console_clear!();
    header_start!();
    println!("Players:");

    for player in game.get_players() {
        println!("{}", player.name);
    }

    println!("\n{} total", player_ct);
    header_end!();
    prompt!("Press enter to continue...");

    loop {
        console_clear!();
        game.round();
    }
}
