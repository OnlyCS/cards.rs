use crate::card::Card;
use crate::game_common::Game;
use crate::player::{Player, PlayerCard};
use crate::ui::{header_end, header_start, prompt};

pub struct War {
    pub players: Vec<Player>,
}

impl War {
    pub fn new(player_names: Vec<String>) -> War {
        assert!(player_names.len() > 1, "E_NOT_ENOUGH_PLAYERS");
        assert!(player_names.len() < 5, "E_TOO_MANY_PLAYERS");

        let mut players = Vec::new();
        let mut deck = Card::new_deck();

        let cards_per_player = (52.0 / player_names.len() as f64).floor() as i32;
        let extra = 52 % player_names.len();

        for (i, name) in player_names.iter().enumerate() {
            players.push(Player::new(
                name.to_string(),
                i.try_into().unwrap(),
                &mut deck,
                cards_per_player + i32::from(i < extra),
            ));
        }

        War { players }
    }

    pub fn draw(&mut self) -> Vec<PlayerCard> {
        let players = &mut self.players;

        let mut out = Vec::new();
        let mut cards = Vec::new();

        for player in players.iter_mut() {
            let draw = match player.draw() {
                Ok(x) => x,
                Err(_) => {
                    out.push(player.id);
                    continue;
                }
            };

            println!("{} drew a {}", player.name, draw.card);
            cards.push(draw);
        }

        self.remove_all(out);

        cards
    }

    pub fn win_game(&self) {
        let winner = &self.players[0];

        println!("{} wins the game!", winner.name);
        header_end!();
        prompt!("Press enter to exit");
        std::process::exit(0);
    }

    pub fn remove(&mut self, player_id: i32) {
        let mut player: Option<Player> = None;

        for i in 0..self.players.len() {
            if self.players[i].id == player_id {
                player = Some(self.players.remove(i));
                break;
            }
        }

        println!(
            "{} has no more cards and is out of the game!",
            player.unwrap().name
        );

        if self.players.len() == 1 {
            self.win_game();
        }
    }

    pub fn remove_all(&mut self, player_ids: Vec<i32>) {
        for player_id in player_ids {
            self.remove(player_id);
        }
    }

    pub fn war(&mut self, winners: &mut Vec<i32>, floor: &mut Vec<PlayerCard>) {
        println!("\nWar!");

        let mut player_draws = Vec::new();

        for player in self.players.iter_mut() {
            if winners.contains(&player.id) {
                let mut draws = player.fdraw_all(4);
                let mut cmp: Option<PlayerCard> = None;

                if draws.is_empty() {
                    for i in (0..floor.len()).rev() {
                        if floor[i].player_id == player.id {
                            cmp = Some(floor.remove(i));
                            break;
                        }

                        if i == 0 {
                            panic!("E_UNKNOWN");
                        }
                    }
                } else {
                    cmp = Some(draws.pop().expect("E_UNKNOWN"));
                }

                let cmp_known = cmp.expect("E_UNKNOWN");

                println!(
                    "{} drew {} unknown cards and a {}",
                    player.name,
                    draws.len(),
                    cmp_known.card
                );

                player_draws.push(cmp_known);
                floor.extend(draws);
            }
        }

        player_draws.sort_by(|a, b| b.card.value_id.cmp(&a.card.value_id));

        let highest = player_draws[0].card.value_id;
        winners.clear();

        for player_draw in player_draws.iter() {
            if player_draw.card.value_id == highest {
                winners.push(player_draw.player_id);
            }
        }

        floor.extend(player_draws.drain(..).collect::<Vec<PlayerCard>>());
    }

    fn round_win(&mut self, winner: i32, floor: &mut Vec<PlayerCard>) {
        let players = &mut self.players;
        let winner = players
            .iter_mut()
            .find(|e| e.id == winner)
            .expect("E_UNKNOWN");

        println!(
            "\n{} won the round and gains {} cards!",
            winner.name,
            floor.len()
        );

        floor.iter().for_each(|e| println!("{}", e.card));

        winner
            .deck
            .extend(floor.drain(..).map(|x| x.card).collect::<Vec<Card>>());
    }
}

impl Game for War {
    fn round(&mut self) {
        header_start!();

        let mut player_draws = self.draw();

        player_draws.sort_by(|a, b| b.card.value_id.cmp(&a.card.value_id));

        let highest = player_draws[0].card.value_id;
        let mut winners = Vec::new();
        let mut floor = Vec::new();

        for player_draw in player_draws.iter() {
            if player_draw.card.value_id == highest {
                winners.push(player_draw.player_id);
            }
        }

        floor.extend(player_draws.drain(..).collect::<Vec<PlayerCard>>());

        while winners.len() > 1 {
            self.war(&mut winners, &mut floor);
        }

        self.round_win(winners[0], &mut floor);

        let sum: usize = self.players.iter().map(|x| x.deck.len()).sum();
        assert_eq!(sum, 52);

        header_end!();
        prompt!("Press enter to continue...");
    }

    fn get_players(&self) -> &Vec<Player> {
        &self.players
    }
}
