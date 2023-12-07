use std::io::stdin;

use itertools::Itertools;

fn main() {
	let cards = parse_input(stdin().lines().flatten());

	let calculate_score = |find_hand: fn(deck: &cards::Deck) -> cards::hand::Hand| {
		cards
			.iter()
			.map(|(cards, bid)| (find_hand(cards), bid))
			.sorted_by_key(|(hand, _)| *hand)
			.zip(1..)
			.map(|((_, bid), rank)| rank * bid)
			.sum::<usize>()
	};

	{
		let score = calculate_score(cards::hand::from_cards);
		println!("Part1: {score}");
	}
	{
		let score = calculate_score(cards::hand::from_cards_with_joker_wildcards);
		println!("Part2: {score}");
	}
}

mod cards {
	pub mod hand {
		use super::*;

		pub type Hand = (HandType, Deck);

		pub fn from_cards(cards: &Deck) -> Hand {
			(HandType::from_cards(cards), *cards)
		}

		pub fn from_cards_with_joker_wildcards(cards: &Deck) -> Hand {
			let mut counts = [0u8; Card::MAX_VALUE + 1];
			for card in cards {
				counts[card.0 as usize] += 1;
			}
			let jokers = counts[Card::JOKER.0 as usize];
			counts[Card::JOKER.0 as usize] = 0; // Take out the Jokers from the counts
			counts.sort();
			counts.reverse();
			counts[0] += jokers; // ...and instead use them to inflate the most occurring card count

			let with_jokers_replaced = cards.map(|c| match c {
				Card::JOKER => Card::SURROGATE_JOKER,
				_ => c,
			});

			(HandType::from_counts(&counts), with_jokers_replaced)
		}
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
	pub enum HandType {
		HighCard = 0,
		OnePair,
		TwoPair,
		ThreeOfAKind,
		FullHouse,
		FourOfAKind,
		FiveOfAKind,
	}

	impl HandType {
		pub fn from_cards(cards: &Deck) -> HandType {
			let mut counts = [0u8; Card::MAX_VALUE + 1];
			for card in cards {
				counts[card.0 as usize] += 1;
			}
			counts.sort();
			counts.reverse();

			Self::from_counts(&counts)
		}

		fn from_counts(counts: &[u8]) -> HandType {
			match counts[..] {
				[5, ..] => HandType::FiveOfAKind,
				[4, ..] => HandType::FourOfAKind,
				[3, 2, ..] => HandType::FullHouse,
				[3, ..] => HandType::ThreeOfAKind,
				[2, 2, ..] => HandType::TwoPair,
				[2, ..] => HandType::OnePair,
				_ => HandType::HighCard,
			}
		}
	}

	pub type Deck = [Card; 5];

	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
	pub struct Card(u8);

	impl Card {
		const SURROGATE_JOKER: Card = Card(0);
		const JACK: Card = Card(9);
		const JOKER: Card = Card(10);
		const QUEEN: Card = Card(11);
		const KING: Card = Card(12);
		const ACE: Card = Card(13);

		const MAX_VALUE: usize = Card::ACE.0 as usize;

		fn numbered(number: u8) -> Self {
			if !(2..=9).contains(&number) {
				panic!();
			}

			Self(number - 1)
		}
	}

	impl TryFrom<char> for Card {
		type Error = anyhow::Error;

		fn try_from(value: char) -> Result<Self, Self::Error> {
			Ok(match value {
				'2'..='9' => Self::numbered(value as u8 - b'0'),
				'T' => Self::JACK,
				'J' => Self::JOKER,
				'Q' => Self::QUEEN,
				'K' => Self::KING,
				'A' => Self::ACE,
				_ => anyhow::bail!("Invalid card {value}"),
			})
		}
	}
}

fn parse_input(lines: impl IntoIterator<Item = impl AsRef<str>>) -> Vec<(cards::Deck, usize)> {
	lines
		.into_iter()
		.map(|line| {
			let (cards, bid) = line.as_ref().split_once(' ').expect("Malformed input");
			let cards: cards::Deck = cards
				.chars()
				.map(|c| c.try_into().unwrap())
				.collect::<Vec<_>>()
				.try_into()
				.unwrap();
			let bid = bid.parse::<usize>().expect("Malformed input");
			(cards, bid)
		})
		.collect::<Vec<_>>()
}
