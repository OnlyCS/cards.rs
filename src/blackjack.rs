use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card::Card;
use crate::game_common::Game;
use crate::player::Player;
use crate::ui::*;

pub struct BlackJack {
    players: Vec<Player>,
    deck: Vec<Card>,
}

impl BlackJack {
    pub fn new(player_names: Vec<String>) -> BlackJack {
        assert!(player_names.len() > 1, "E_NOT_ENOUGH_PLAYERS");
        assert!(player_names.len() < 5, "E_TOO_MANY_PLAYERS");

        let mut players = Vec::new();
        let mut deck = Card::new_random_deck();

        for (i, name) in player_names.iter().enumerate() {
            let mut player = Player::new(name.to_string(), i as i32, &mut deck, 2);
            player.deck[1].visible = false;

            players.push(player);
        }

        BlackJack { players, deck }
    }

    fn print_player_cards(&self) {
        for player in &self.players {
            print!("{} has:", player.name);

            for card in &player.deck {
                print!(" {}", if card.visible { &card.value } else { "Unknown" });
            }

            println!();
        }
    }

    fn handle_hit(&mut self, player_id: i32) -> bool {
        let player = self.players.iter_mut().find(|p| p.id == player_id).unwrap();

        let mut hand_total: i32 = player.deck.iter().map(|x| x.value_id).sum();

        let card = match self.deck.pop() {
            Some(x) => x,
            None => {
                self.deck = Card::new_deck();
                self.deck.shuffle(&mut thread_rng());
                self.deck.pop().unwrap()
            }
        };

        let card_value = match card.value_id {
            -1 => 1,
            0..=8 => card.value_id + 2,
            9..=11 => 10,
            12 => {
                let val = prompt!("Ace value (Current total {}, 1 or 11):", hand_total)
                    .trim()
                    .parse()
                    .unwrap();

                assert!(val == 1 || val == 11, "E_INVALID_ACE_VALUE");

                val
            }
            _ => panic!("E_INVALID_CARD_VALUE"),
        };

        hand_total += card_value;

        if hand_total > 21 {
            println!("You busted!");
            true
        } else {
            println!("Your hand total is now {}", hand_total);
            player.deck.push(card);
            false
        }
    }

    fn turn(&mut self, player_id: i32) -> bool {
        self.print_player_cards();

        let player_index = Player::player_index(&self.players, player_id).expect("E_UNKNOWN");
        let player = &self.players[player_index];

        println!();
        let choice = prompt_options!(
            format!("{}'s turn. What do you want to do?", player.name),
            &[
                Option {
                    name: "Hit",
                    value: 1,
                },
                Option {
                    name: "Stand",
                    value: 2,
                },
            ]
        );

        console_clear!();
        header_start!();
        match choice {
            1 => {
                let busted = self.handle_hit(player_id);

                if busted {
                    self.players.retain(|x| x.id != player_id);
                    return true;
                }

                false
            }
            2 => {
                println!("You stood. It is now the next player's turn.");
                true
            }
            _ => panic!("Invalid choice"),
        }
    }
}

impl Game for BlackJack {
    fn round(&mut self) {
        let player_ids = self.players.iter().map(|x| x.id).collect::<Vec<i32>>();

        for id in player_ids {
            loop {
                console_clear!();
                header_start!();

                let do_break = self.turn(id);

                header_end!();
                prompt!("Press enter to continue");

                if do_break {
                    break;
                }
            }
        }
    }

    fn get_players(&self) -> &Vec<Player> {
        &self.players
    }
}
