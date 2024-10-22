use std::fmt::Display;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug, EnumIter, Copy, Clone, PartialEq)]
pub enum Card {
    Ace = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Ten = 9,
    Jack = 10,
    Queen = 11,
    King = 12,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Card::Ace => "A",
            Card::Two => "2",
            Card::Three => "3",
            Card::Four => "4",
            Card::Five => "5",
            Card::Six => "6",
            Card::Seven => "7",
            Card::Eight => "8",
            Card::Nine => "9",
            Card::Ten => "10",
            Card::Jack => "J",
            Card::Queen => "Q",
            Card::King => "K",
        })
    }
}

pub struct Deck {
    cards: Vec<Card>,
    discard_pile: Vec<Card>,
}

impl Deck {
    pub fn new(num_decks: i32) -> Self {
        let mut cards = Vec::<Card>::new();
        // Duplicate cards to generate the right deck
        for card in Card::iter() {
            for _i in 0..(num_decks * 4) {
                cards.push(card);
            }
        }

        Deck {
            cards: cards,
            discard_pile: Vec::<Card>::new(),
        }
    }

    pub fn draw_card(&mut self) -> Card {
        if self.size() == 0 {
            self.restack();
            self.shuffle();
        }

        // Pull the top card
        let card = self.cards.pop().expect("Cannot pop off of empty deck");
        self.discard_pile.push(card);
        card
    }

    pub fn size(&self) -> usize {
        self.cards.len()
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    // Restores all cards from discard pile to normal pile (should reshuffle)
    pub fn restack(&mut self) {
        self.cards.append(&mut self.discard_pile);
    }

}

pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Hand {
            cards: Vec::<Card>::new(),
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    // Returns tuple of two possible sums
    pub fn calculate_points(&self, ignore_last: bool) -> Points {
        let mut sum: i32 = 0;
        let mut has_ace: bool = false;
        let mut it = (&self.cards).into_iter().peekable();

        while let Some(card) = it.next() {
            if !(it.peek().is_none() && ignore_last) {
                match card {
                    Card::Ace => {
                        has_ace = true;
                        sum += 1;
                    },
                    Card::Two => sum += 2,
                    Card::Three => sum += 3,
                    Card::Four => sum += 4,
                    Card::Five => sum += 5,
                    Card::Six => sum += 6,
                    Card::Seven => sum += 7,
                    Card::Eight => sum += 8,
                    Card::Nine => sum += 9,
                    Card::Ten => sum += 10,
                    Card::Jack => sum += 10,
                    Card::Queen => sum += 10,
                    Card::King => sum += 10,
                };
            }
        }

        if has_ace {
            Points::new2(sum, sum + 10)
        } else {
            Points::new1(sum)
        }
    }

    pub fn to_string(&self, hide_last: bool) -> String {
        let mut str = String::new();
        let mut it = (&self.cards).into_iter().peekable();
        while let Some(card) = it.next() {
            if it.peek().is_none() && hide_last {
                // On last card in the hand
                str.push_str("? ");

            } else {
                str.push_str(match card {
                    Card::Ace => "A",
                    Card::Two => "2",
                    Card::Three => "3",
                    Card::Four => "4",
                    Card::Five => "5",
                    Card::Six => "6",
                    Card::Seven => "7",
                    Card::Eight => "8",
                    Card::Nine => "9",
                    Card::Ten => "10",
                    Card::Jack => "J",
                    Card::Queen => "Q",
                    Card::King => "K",
                });
                str.push_str(" ");
            }
        }

        str
    }

    // Returns None if can't split, or the card if can split
    pub fn split(&self) -> Option<Card> {
        if self.cards.len() == 2 && self.cards[0] == self.cards[1] {
            Some(self.cards[0])
        } else {
            None
        }
    }

}

pub struct Points {
    op1: i32,
    op2: i32,
    has_op2: bool,
}

impl Points {
    pub fn new1(op1: i32) -> Self {
        Points {
            op1: op1,
            op2: -1,
            has_op2: false,
        }
    }

    pub fn new2(op1: i32, op2: i32) -> Self {
        Points {
            op1: op1,
            op2: op2,
            has_op2: true,
        }
    }

    pub fn to_string(&self) -> String {
        if self.has_op2 {
            let mut str: String = String::new();
            str.push_str(self.op1.to_string().as_str());
            str.push('/');
            str.push_str(self.op2.to_string().as_str());
            str

        } else {
            self.op1.to_string()
        }
    }

    pub fn calculate_best_value(&self) -> i32 {
        // Calculates highest, unless it goes over 21.
        // If so, uses lower value
        if !self.has_op2 {
            self.op1
        } else {
            if self.op2 > 21 {
                self.op1
            } else {
                self.op2
            }
        }
    }

    pub fn is_soft_17(&self) -> bool {
        let value = self.calculate_best_value();
        if value < 17 || (value == 17 && self.has_op2) {
            return true;
        }
        return false;
    }
}

// Result of playing a hand; either a bust, the points the player ended up with, a split, or double
pub enum HandResult {
    Bust,
    Points(Points),
    Split(Card),
    DoubleDown(Points),
}
