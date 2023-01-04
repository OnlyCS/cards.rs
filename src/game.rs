use crate::player::Player;

pub trait Game {
    fn round(&mut self);
    fn get_players(&self) -> &Vec<Player>;
}