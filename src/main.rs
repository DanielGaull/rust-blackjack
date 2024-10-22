use card::{Deck, Hand, HandResult};
use mov::Move;
use std::{collections::VecDeque, io::{stdin, stdout, Write}};

mod card;
mod mov;

fn main() {
    let mut deck: Deck = Deck::new(6);
    deck.shuffle();
    println!("== Blackjack ==");
    let mut user_points = 10;
    println!("You have 10 hands. Each win grants you 1 hand, while a loss loses you 1 hand");
    println!("If you lose all hands, you lose");
    loop {
        let points = play_turn(&mut deck);
        user_points += points;
        if user_points <= 0 {
            println!("You lost everything");
            break;
        } else {
            if user_points == 10 {
                println!("You are even");
            } else if user_points > 10 {
                println!("You are up +{}", user_points - 10);
            } else {
                println!("You are down -{}", 10 - user_points);
            }
        }
    }
}

// Returns number of hands to give the player
fn play_turn(deck: &mut Deck) -> i32 {
    let mut player_hand: Hand = Hand::new();
    let mut dealer_hand: Hand = Hand::new();
    player_hand.add_card(deck.draw_card());
    dealer_hand.add_card(deck.draw_card());
    player_hand.add_card(deck.draw_card());
    dealer_hand.add_card(deck.draw_card());

    let player_hand_pts = player_hand.calculate_points(false);
    // Used to check if dealer has blackjack
    let true_dealer_hand_pts = dealer_hand.calculate_points(false);

    if true_dealer_hand_pts.calculate_best_value() == 21 && player_hand_pts.calculate_best_value() != 21 {
        println!("Dealer blackjack!");
        return -1;
    }
    if player_hand_pts.calculate_best_value() == 21 {
        // Already checked for dealer blackjack
        println!("Blackjack!");
        return 1;
    }

    let mut hands = VecDeque::<&mut Hand>::new();
    let mut hand_results = VecDeque::<HandResult>::new();
    hands.push_front(&mut player_hand);
    while hands.len() > 0 {
        let hand = hands.pop_front().expect("Error: no hand in the queue");
        play_hand(deck, hand, &dealer_hand, &hands, &hand_results);
    }

    return 0;
}

pub fn play_hand(
    deck: &mut Deck,
    hand: &mut Hand,
    dealer_hand: &Hand,
    hands: &VecDeque<&mut Hand>,
    hand_results: &VecDeque<HandResult>,
) -> HandResult {
    let dealer_hand_pts = dealer_hand.calculate_points(true);
    loop {
        let mut player_hand_pts = hand.calculate_points(false);
        println!("This hand: {} ({})", hand.to_string(false), player_hand_pts.to_string());
        println!("Dealer's hand: {} ({})", dealer_hand.to_string(true), dealer_hand_pts.to_string());

        let mov = get_move();
        match mov {
            Move::Hit => {
                // Add another card to the player's hand
                hand.add_card(deck.draw_card());
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
                hand.add_card(deck.draw_card());
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

        let mov: char = read_input();
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
