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
        let mut deck = Card::new_deck_blackjack();

        for (i, name) in player_names.iter().enumerate() {
            let mut player = Player::new(name.to_string(), i as i32, &mut deck, 2);
            let hand_total: i32 = player.deck.iter().map(|x| x.blackjack_cmp_val().unwrap()).sum();
        
            for card in player.deck.iter_mut() {
                if card.value_id == 14 {
                    card.value_id = match BlackJack::redef_ace(name.to_string(), hand_total-11) {
                        1 => 1,
                        11 => 14,
                        _ => panic!("E_INVALID_VALUE"),
                    };

                    card.refresh_blackjack().unwrap();
                }
            }
            player.deck[0].visible = false;

            players.push(player);
        }

        BlackJack { players, deck }
    }

    fn print_player_cards(&self, player_id: i32) {
        for player in &self.players {
            print!("{} has: ", player.name);

            if let Some(first_card) = player.deck.first() {
                let card_value = if first_card.visible || player.id == player_id {
                    &first_card.value
                } else {
                    "Unknown"
                };

                print!("{}", card_value);

                for card in player.deck.iter().skip(1) {
                    let card_value = if card.visible || player.id == player_id {
                        &card.value
                    } else {
                        "Unknown"
                    };

                    print!(", {}", card_value);
                }
            }

            println!();
        }
    }

    pub fn redef_ace(player_name: String, hand_total: i32) -> i32 {
        let value = prompt!("{}, you have an ace. Your current hand total is {}. What do you want it to be worth (1 or 11)? ",  player_name, hand_total).trim().parse().unwrap();

        assert!(value == 1 || value == 11, "E_INVALID_VALUE");
        value
    }

    fn handle_hit(&mut self, player_id: i32) -> bool {
        let player = self.players.iter_mut().find(|p| p.id == player_id).unwrap();
        let mut hand_total: i32 = player.deck.iter().map(|x| x.value_id).sum();

        let mut card = match self.deck.pop() {
            Some(x) => x,
            None => {
                self.deck = Card::new_deck();
                self.deck.shuffle(&mut thread_rng());
                self.deck.pop().unwrap()
            }
        };

        

        let card_value = if card.value_id == 14 {
            BlackJack::redef_ace(player.name.to_string(), hand_total)
        } else {
            card.value_id
        };
        card.value_id = card_value;
        card.refresh_blackjack().unwrap();

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
        self.print_player_cards(player_id);

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
