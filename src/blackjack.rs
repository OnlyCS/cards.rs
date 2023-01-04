use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::game::Game;
use crate::player::Player;
use crate::card::Card;
use crate::ui::*;

pub struct BlackJack {
    players: Vec<Player>,
    deck: Vec<Card>
}

impl BlackJack {
    pub fn new(player_ct: i32) -> BlackJack {
        header_start();
        assert!(1 < player_ct, "Must have more than one player");
        assert!(player_ct < 5, "Must have less than five players");

        let mut deck = Card::new_deck();
        let mut players = Vec::new();

        for i in 0..player_ct {
            let name = prompt(&format!("Enter Player {}'s name: ", i + 1)).trim().to_string();
            let mut player = Player::new(
                name,
                i,
                &mut deck,
                2
            );

            player.deck[0].visible = false;

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

    fn turn(&mut self, player_id: i32) -> bool {
        self.print_player_cards();

        let player = &mut self.players[player_id as usize];

        let choice = prompt_options(&format!("{}'s turn. What do you want to do?", player.name), &[
            Option { name: "Hit".to_string(), value: 1 },
            Option { name: "Stand".to_string(), value: 2 },
        ]);
        
        header_start();
        match choice {
            1 => {
                let mut card = match self.deck.pop() {
                    Some(x) => x,
                    None => {
                        self.deck = Card::new_deck();
                        self.deck.shuffle(&mut thread_rng());
                        self.deck.pop().unwrap()
                    }
                };

                card.visible = false;
                println!("You drew a {}", card.value);
                false
            },
            2 => {
                println!("You stood. It is now the next player's turn.");
                true
            },
            _ => panic!("Invalid choice")
        }
    }
}

impl Game for BlackJack {
    fn round(&mut self) {
        let player_ids = self.players.iter().map(|x| x.id).collect::<Vec<i32>>();

        for id in player_ids {
            loop {
                header_start();
                
                let do_break = self.turn(id);

                if do_break {
                    break;
                }

                header_end();
                prompt("Press enter to continue");
            }
        }
    }

    fn get_players(&self) -> &Vec<Player> {
        &self.players
    }
}