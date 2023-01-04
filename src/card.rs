use rand::random;
use std::{error::Error, fmt::{Display, Formatter, self}};

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
            _ => return Err(format!("Invalid value value, needed 0-12, got {}", suit_id).into())
        };
        let value = match value_id {
            0 => "2",
            1 => "3",
            2 => "4",
            3 => "5",
            4 => "6",
            5 => "7",
            6 => "8",
            7 => "9",
            8 => "10",
            9 => "Jack",
            10 => "Queen",
            11 => "King",
            12 => "Ace",
            _ => return Err(format!("Invalid value value, needed 0-12, got {}", value_id).into())
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

	pub fn copy(&self) -> Card {
		Card {
			suit: self.suit.clone(),
			suit_id: self.suit_id,
			value: self.value.clone(),
			value_id: self.value_id,
		}
	}
}


impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.value, self.suit)
    }
}