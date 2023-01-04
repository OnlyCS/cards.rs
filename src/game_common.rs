use crate::player::Player;
use crate::ui::*;

pub trait Game {
    fn round(&mut self);
    fn get_players(&self) -> &Vec<Player>;
}

pub fn get_player_names(player_ct: i32) -> Vec<String> {
    header_start();

    let mut names = Vec::new();

    for i in 0..player_ct {
        let name = prompt(&format!("Enter Player {}'s name: ", i + 1))
            .trim()
            .to_string();

        names.push(name);
    }

    console_clear();

    names
}
