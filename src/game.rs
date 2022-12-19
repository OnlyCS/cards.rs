use std::io::{self, Write};

use rand::{thread_rng, Rng};

use crate::card::Card;
use crate::player::{Player, PlayerCard};
use crate::ui::console_clear;

pub struct Game {
    pub players: Vec<Player>,
    pub floor: Vec<PlayerCard>,
}

impl Game {
    pub fn new(max_players: i32) -> Game {
        let mut players = Vec::new();
        let mut deck = Card::new_deck();

        let cards_per_player = (52.0 / max_players as f64).floor() as i32;
        let extra = 52 % max_players;

        for i in 0..max_players {
            let mut name = String::new();

            print!("Enter Player {}'s name: ", i + 1);

            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut name).unwrap();

            players.push(Player::new(
                name.trim().to_string(),
                i,
                &mut deck,
                cards_per_player + if i < extra { 1 } else { 0 },
            ));
        }

        Game {
            players: players.clone(),
            floor: Vec::new(),
        }
    }

    pub fn player_by_id(&mut self, id: i32) -> Player {
        self.players
            .iter()
            .find(|player| player.id == id)
            .unwrap()
            .to_owned()
    }

    pub fn remove_player(&mut self, id: i32) {
        let player = self
            .players
            .iter()
            .find(|player| player.id == id)
            .unwrap()
            .to_owned();

        let index = self.players.iter().position(|p| p.id == id).unwrap();

        println!(
            "{} ran out of cards! Cards given randomly to players",
            player.name
        );

        self.players.remove(index);
        let count = self.players.len();

        for card in player.deck {
            self.players[thread_rng().gen_range(0..count)].give(card);
        }
    }

    pub fn round(&mut self) {
        println!("{}\n", "-".to_string().repeat(20));

        let mut cmp_cards = Vec::new();

        // get each player's draw
        for player in &self.players {
            let card = match player.to_owned().draw() {
                Result::Ok(val) => val,
                Result::Err(_) => {
                    self.remove_player(player.id);
                    break;
                }
            };

            println!("{} drew a {}", player.name, card.card.to_string());
            cmp_cards.push(card);
        }

        // sort
        cmp_cards.sort_by(|a, b| b.card.value_id.cmp(&a.card.value_id));

        // war checks
        while cmp_cards[0].card.value_id == cmp_cards[1].card.value_id {
            print!("\nTie between players:");

            let mut same_ct = 0;
            let same_val = cmp_cards[0].card.value_id;

            // copy current draw to "floor"
            for card in cmp_cards {
                let player = self.player_by_id(card.player_id);

                if card.card.value_id == same_val {
                    same_ct += 1;
                    print!(" {}", player.name);
                }

                self.floor.push(card);
            }
            print!("\n");

            cmp_cards = Vec::new();

            // copy cards to new comparison and add three more to "floor"
            for i in 0..same_ct {
                let mut player = self.player_by_id(self.floor[i].player_id);

                let cards = match player.draw_all(3) {
                    Result::Ok(val) => val,
                    Result::Err(_err) => {
                        self.remove_player(player.id);
                        break;
                    }
                };

                let cmp = match player.draw() {
                    Result::Ok(val) => val,
                    Result::Err(_err) => {
                        self.remove_player(player.id);
                        break;
                    }
                };

                for i in cards {
                    self.floor.push(i);
                }

                println!("{} drew a {}", player.name, cmp.card.to_string());

                cmp_cards.push(cmp);
            }

            cmp_cards.sort_by(|a, b| b.card.value_id.cmp(&a.card.value_id));
        }

        let mut winner = self.player_by_id(cmp_cards[0].player_id);

        println!("\n{} wins this round. Cards kept:", winner.name);

        for card in cmp_cards {
            println!("{}", card.card.to_string());
            winner.give(card.card);
        }

        for card in &self.floor {
            println!("{}", card.card.to_string());
            winner.give(card.card.to_owned());
        }

        println!("\n{}", "-".to_string().repeat(20));
        print!("Press enter to continue...");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut String::new()).unwrap();

        console_clear();

        self.floor = Vec::new();
    }
}
