pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

use rand::Rng;

#[derive(Debug, PartialEq, Eq, Copy)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club
}

#[derive(Debug, PartialEq, Eq, Copy)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=3) {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Spade,
            _ => Suit::Club,
        }
    }

    pub fn translate(value: u8) -> Suit {
		match value{
			1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
			_ => unreachable!(),
		}
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=4) {
            0 => Rank::Ace,
            1 => Rank::King,
            2 => Rank::Queen,
            3 => Rank::Jack,
            _ => Rank::Number(rng.gen_range(2..=10)),
        }
    }

    pub fn translate(value: u8) -> Rank {
		match value{
			1 => Rank::Ace,
            13 => Rank::King,
            12 => Rank::Queen,
            11 => Rank::Jack,
            v if v >= 2 && v <= 10 => Rank::Number(value),
			_ => unreachable!(),
		}
    }
}

#[derive(Debug, PartialEq, Eq, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: Card) -> bool {
	card.rank == Rank::Ace && card.suit == Suit::Spade
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
