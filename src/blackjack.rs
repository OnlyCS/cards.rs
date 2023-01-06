use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card::Card;
use crate::game_common::Game;
use crate::player::Player;
use crate::ui::*;

pub struct BlackJack {
    players: Vec<Player>,
    busted_players: Vec<Player>,
    deck: Vec<Card>,
}

impl BlackJack {
    pub fn new(player_names: Vec<String>) -> BlackJack {
        assert!(player_names.len() > 1, "E_NOT_ENOUGH_PLAYERS");
        assert!(player_names.len() < 5, "E_TOO_MANY_PLAYERS");

        let mut players = Vec::new();
        let mut deck = Card::new_deck_blackjack();
        let busted_players = Vec::new();

        for (i, name) in player_names.iter().enumerate() {
            let mut player = Player::new(name.to_string(), i as i32, &mut deck, 2);
            let hand_total: i32 = player
                .deck
                .iter()
                .map(|x| x.blackjack_cmp_val().unwrap())
                .sum();

            for card in player.deck.iter_mut() {
                if card.value_id == 14 {
                    card.value_id = match BlackJack::redef_ace(name.to_string(), hand_total - 11) {
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

        BlackJack {
            players,
            deck,
            busted_players,
        }
    }

    fn print_player_cards(&self, player_id: i32) {
        for player in &self.players {
            print!("{} has: ", player.name);

            // print player.deck[i].value with proper comma and "and" logic
            for (i, card) in player.deck.iter().enumerate() {
                let mut card_value = if card.visible || player.id == player_id {
                    match card
                        .value
                        .chars()
                        .next()
                        .unwrap()
                        .to_lowercase()
                        .to_string()
                        .as_str()
                    {
                        "a" | "e" | "i" | "o" | "u" => format!("an {}", card.value),
                        _ => format!("a {}", card.value),
                    }
                } else {
                    "a face-down card".to_string()
                };

                // if i is 0, make the first letter in card_value uppercase, then set it to card_value
                if i == 0 {
                    card_value = card_value.remove(0).to_uppercase().to_string() + &card_value;
                }

                if i == player.deck.len() - 1 {
                    println!("and {}", card_value);
                } else {
                    print!("{}, ", card_value);
                }
            }
        }

        for player in &self.busted_players {
            println!("{}: busted with a total of {}", player.name, BlackJack::get_hand_total(&player.deck));
        }
    }

    pub fn redef_ace(player_name: String, hand_total: i32) -> i32 {
        let value = prompt!("{}, you have an ace. Your current hand total is {}. What do you want it to be worth (1 or 11)? ",  player_name, hand_total).trim().parse().unwrap();

        assert!(value == 1 || value == 11, "E_INVALID_VALUE");
        value
    }

    fn handle_hit(&mut self, player_id: i32) -> std::option::Option<Card> {
        let players = &mut self.players;
        let player = players.iter().find(|p| p.id == player_id).unwrap();
        let mut hand_total: i32 = BlackJack::get_hand_total(&player.deck);

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
            BlackJack::get_hand_total(&[card.clone()])
        };
        card.value_id = card_value;
        card.refresh_blackjack().unwrap();

        hand_total += card_value;

        if hand_total > 21 {
            println!(
                "You busted with a {} and a total of {}",
                card.value, hand_total
            );
            let mut player = players.remove(Player::player_index(players, player.id).unwrap());
            player.deck.push(card);
            self.busted_players.push(player);

            None
        } else {
            println!("Your hand total is now {}", hand_total);
            Some(card)
        }
    }

    fn turn(&mut self, player_id: i32) -> bool {
        self.print_player_cards(player_id);

        let player_index = Player::player_index(&self.players, player_id).expect("E_UNKNOWN");
        let player = &self.players[player_index];

        println!();
        let choice = prompt_options!(
            format!(
                "{}'s turn. Your total is {}. What do you want to do?",
                player.name,
                BlackJack::get_hand_total(&player.deck)
            ),
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
                if let Some(card) = self.handle_hit(player_id) {
                    self.players[player_index].deck.push(card);
                } else {
                    return true;
                }

                false
            }
            2 => {
                println!("You stood.");
                true
            }
            _ => panic!("Invalid choice"),
        }
    }

    fn get_hand_total(hand: &[Card]) -> i32 {
        hand.iter().map(|i| i.blackjack_cmp_val().unwrap()).sum()
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
                prompt!("Press enter to continue...");

                if do_break {
                    break;
                }
            }
        }

        let post_players = &mut self.players;

        console_clear!();
        header_start!();
        if post_players.is_empty() {
            println!("There were no winners, all of them busted");
        } else {
            let mut winners = Vec::new();

            post_players.sort_by(|a, b| {
                BlackJack::get_hand_total(&b.deck).cmp(&BlackJack::get_hand_total(&a.deck))
            });
    
            let highest = BlackJack::get_hand_total(&post_players[0].deck);
    
            for player in post_players {
                if BlackJack::get_hand_total(&player.deck) == highest {
                    winners.push(player);
                }
            }

            println!("Winners are:");
            for player in winners {
                println!(
                    "{} with a total score of {}",
                    player.name,
                    BlackJack::get_hand_total(&player.deck)
                );
            }
        }

        header_end!();
        prompt!("Press enter to exit...");
        console_clear!();

        std::process::exit(0);
    }

    fn get_players(&self) -> &Vec<Player> {
        &self.players
    }
}
