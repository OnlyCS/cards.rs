use rand::random;
use std::{error::Error};

#[derive(Clone)]
pub struct Card {
    pub suit: String,
    pub suit_id: i32,
    pub value: String,
    pub value_id: i32,
}

impl Card {
    pub fn new(suit_id: i32, value_id: i32) -> Result<Card, Box<dyn Error>> {
        let suit = match suit_id {
            0 => "Spades",
            1 => "Hearts",
            2 => "Clubs",
            3 => "Diamonds",
            _ => return Err(format!("Invalid value value, needed 0-12, got {}", suit_id).into()),
        };
        let value = match value_id {
            0 => "Ace",
            1 => "2",
            2 => "3",
            3 => "4",
            4 => "5",
            5 => "6",
            6 => "7",
            7 => "8",
            8 => "9",
            9 => "10",
            10 => "Jack",
            11 => "Queen",
            12 => "King",
            _ => return Err(format!("Invalid value value, needed 0-12, got {}", value_id).into()),
        };
        Ok(Card {
            suit: suit.to_string(),
            suit_id,
            value: value.to_string(),
            value_id,
        })
    }

    pub fn new_random() -> Result<Card, Box<dyn Error>> {
        let suit_id = random::<i32>() % 4;
        let value_id = random::<i32>() % 13;
        Card::new(suit_id, value_id)
    }

	pub fn new_deck() -> Vec<Card> {
		let mut deck = Vec::new();

		for i in 0..4 {
			for j in 0..13 {
				deck.push(Card::new(i, j).unwrap());
			}
		}

		deck
	}

	pub fn clone(&self) -> Card {
		Card {
			suit: self.suit.clone(),
			suit_id: self.suit_id,
			value: self.value.clone(),
			value_id: self.value_id,
		}
	}

	pub fn to_string(&self) -> String {
		format!("{} of {}", self.value, self.suit)
	}
}
