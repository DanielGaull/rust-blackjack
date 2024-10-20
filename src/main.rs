use std::io::{stdin, stdout, Write};

use card::{Deck, Hand};
use mov::Move;

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

    let mut player_hand_pts = player_hand.calculate_points(false);
    // Used to check if dealer has blackjack
    let true_dealer_hand_pts = dealer_hand.calculate_points(false);
    let dealer_hand_pts = dealer_hand.calculate_points(true);

    if true_dealer_hand_pts.calculate_best_value() == 21 && player_hand_pts.calculate_best_value() != 21 {
        println!("Dealer blackjack!");
        return -1;
    }
    if player_hand_pts.calculate_best_value() == 21 {
        // Already checked for dealer blackjack
        println!("Blackjack!");
        return 1;
    }

    loop {
        println!("Your hand: {} ({})", player_hand.to_string(false), player_hand_pts.to_string());
        println!("Dealer's hand: {} ({})", dealer_hand.to_string(true), dealer_hand_pts.to_string());

        let mov = get_move();
        match mov {
            Move::Hit => {
                // Add another card to the player's hand
                player_hand.add_card(deck.draw_card());
                player_hand_pts = player_hand.calculate_points(false);
                let best_player_hand_value = player_hand_pts.calculate_best_value();
                if best_player_hand_value > 21 {
                    println!("Bust!");
                    return -1;
                }
            },
            Move::Stand => {

            },
            _ => (),
        }
    }
}

fn play_hand() {

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
