use crate::{card::{Deck, Hand, HandResult}, mov::Move};
use std::io::{stdin, stdout, Write};

struct Blackjack {
    deck: Deck,
    dealer_hand: Hand,
}

impl Blackjack {
    

    // Has the player play hit/stand and will return number of points once done
    pub fn play_hand(&mut self, hand: &mut Hand) -> HandResult {
        let dealer_hand_pts = self.dealer_hand.calculate_points(true);
        loop {
            let mut player_hand_pts = hand.calculate_points(false);
            println!("This hand: {} ({})", hand.to_string(false), player_hand_pts.to_string());
            println!("Dealer's hand: {} ({})", self.dealer_hand.to_string(true), dealer_hand_pts.to_string());

            let mov = Self::get_move();
            match mov {
                Move::Hit => {
                    // Add another card to the player's hand
                    hand.add_card(self.deck.draw_card());
                    player_hand_pts = hand.calculate_points(false);
                    let best_player_hand_value = player_hand_pts.calculate_best_value();
                    if best_player_hand_value > 21 {
                        println!("Bust!");
                        return HandResult::Bust;
                    }
                },
                Move::Stand => {
                    return HandResult::Points(player_hand_pts);
                },
                Move::DoubleDown => {
                    hand.add_card(self.deck.draw_card());
                    player_hand_pts = hand.calculate_points(false);
                    let best_player_hand_value = player_hand_pts.calculate_best_value();
                    if best_player_hand_value > 21 {
                        println!("Bust!");
                        return HandResult::Bust;
                    }
                    return HandResult::DoubleDown(player_hand_pts);
                },
                Move::Split => {
                    // Only valid if both have the same card
                    if let Some(card) = hand.split() {
                        return HandResult::Split(card);
                    }
                    println!("You can only split when your hand has 2 of the same card!");
                },
            }
        }
    }

    fn get_move() -> Move {
        loop {
            println!("H - hit, S - stand, D - double down, L - split");
            println!("Enter your move: ");
    
            let mov: char = Self::read_input();
            match mov {
                's' => return Move::Stand,
                'S' => return Move::Stand,
                'h' => return Move::Hit,
                'H' => return Move::Hit,
                'd' => return Move::DoubleDown,
                'D' => return Move::DoubleDown,
                'l' => return Move::Split,
                'L' => return Move::Split,
                _ => println!("Not a valid move!"),
            };
        }
    }

    
    fn read_input() -> char {
        let mut input = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut input).expect("Error reading in string input");
        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }
        if let Some('\r') = input.chars().next_back() {
            input.pop();
        }
        input.pop().expect("No text was entered")
    }
}
