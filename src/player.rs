use std::error::Error;

use rand::{thread_rng, Rng};

use crate::card::Card;

#[derive(Clone)]
pub struct Player {
    pub deck: Vec<Card>,
    pub name: String,
    pub id: i32,
}

pub struct PlayerCard {
	pub card: Card,
	pub player_id: i32,
}

impl Player {
    pub fn new(name: String, id: i32, from_deck: &mut Vec<Card>, card_count: i32) -> Player {
        let mut deck = Vec::new();

        for _ in 0..card_count {
			let index = thread_rng().gen_range(0..from_deck.len());
            deck.push(from_deck[index].clone());
            from_deck.remove(index);
        }

        Player {
            id,
            deck,
            name: name,
        }
    }

	pub fn clone(&self) -> Player {
		Player {
			id: self.id,
			deck: self.deck.clone(),
			name: self.name.clone(),
		}
	}

	pub fn draw(&mut self) -> Result<PlayerCard, Box<dyn Error>> {
		if self.deck.len() == 0 {
			return Err("Not enough cards".into())
		}

		let card = self.deck[0].clone();
		self.deck.remove(0);

		Ok(PlayerCard { card, player_id: self.id })
	}

	pub fn draw_all(&mut self, count: i32) -> Result<Vec<PlayerCard>, Box<dyn Error>> {
		let mut cards = Vec::new();

		if self.deck.len() < count as usize {
			return Err("Not enough cards".into())
		}

		for _ in 0..count {
			let card = self.draw().unwrap();
			cards.push(card);
		}

		Ok(cards)
	}

	pub fn give(&mut self, card: Card) {
		self.deck.push(card);
	}
}
