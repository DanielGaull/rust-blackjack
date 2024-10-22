use card::{Deck, Hand, HandResult};
use mov::Move;
use std::{collections::VecDeque, io::{stdin, stdout, Write}};

mod card;
mod mov;

fn main() {
    let mut deck: Deck = Deck::new(6);
    deck.shuffle();
    println!("== Blackjack ==");
    let mut credits: i32 = 0;
    loop {
        println!("You have {} credits. Place Bet:", credits);
        let credits_per: i32 = read_num();
        let credit_res = play_turn(&mut deck, credits_per);
        println!("\n\n\n");
        credits += credit_res;
    }
}

// Returns number of hands to give the player
fn play_turn(deck: &mut Deck, credits_per: i32) -> i32 {
    let mut player_hand: Hand = Hand::new();
    let mut dealer_hand: Hand = Hand::new();
    player_hand.add_card(deck.draw_card());
    dealer_hand.add_card(deck.draw_card());
    player_hand.add_card(deck.draw_card());
    dealer_hand.add_card(deck.draw_card());

    let player_hand_pts = player_hand.calculate_points(false);
    // Used to check if dealer has blackjack
    let mut true_dealer_hand_pts = dealer_hand.calculate_points(false);

    if true_dealer_hand_pts.calculate_best_value() == 21 && player_hand_pts.calculate_best_value() != 21 {
        println!("Dealer blackjack!");
        return -credits_per;
    }
    if player_hand_pts.calculate_best_value() == 21 {
        // Already checked for dealer blackjack
        println!("You got Blackjack!");
        return ((credits_per as f32) * 1.5) as i32;
    }

    let mut hands = VecDeque::<Hand>::new();
    let mut hand_results = VecDeque::<HandResult>::new();
    hands.push_front(player_hand);
    while hands.len() > 0 {
        let hand = hands.pop_front().expect("Error: no hand in the queue");
        let result = play_hand(deck, hand, &dealer_hand);
        if let HandResult::Split(num) = result {
            let mut h1 = Hand::new();
            let mut h2 = Hand::new();
            h1.add_card(num);
            h2.add_card(num);
            hands.push_back(h1);
            hands.push_back(h2);
        } else {
            hand_results.push_front(result);
        }
    }

    // Now the dealer's time to do something
    // Hit on soft 17
    while true_dealer_hand_pts.is_soft_17() {
        dealer_hand.add_card(deck.draw_card());
        true_dealer_hand_pts = dealer_hand.calculate_points(false);
    }

    // Print out dealer hand
    let mut dealer_bust = false;
    let dealer_pts = true_dealer_hand_pts.calculate_best_value();
    println!("Dealer's hand: {} ({})", dealer_hand.to_string(false), true_dealer_hand_pts.to_string());
    if dealer_pts > 21 {
        println!("Dealer bust!");
        dealer_bust = true;
    }

    let mut num_credits = 0;
    for result in hand_results {
        match result {
            HandResult::Bust => {
                num_credits -= 1;
            },
            HandResult::DoubleDown(pts) => {
                if dealer_bust || pts.calculate_best_value() > dealer_pts {
                    num_credits += 2;
                } else if !dealer_bust && pts.calculate_best_value() == dealer_pts {
                    println!("Push");
                } else {
                    num_credits -= 2;
                }
            },
            HandResult::Points(pts) => {
                if dealer_bust || pts.calculate_best_value() > dealer_pts {
                    num_credits += 1;
                } else if !dealer_bust && pts.calculate_best_value() == dealer_pts {
                    println!("Push");
                } else {
                    num_credits -= 1;
                }
            },
            // Should never get a split here
            _ => panic!("Invalid hand result!"),
        }
    }

    let change = num_credits * credits_per;

    if change >= 0 {
        println!("+{}", change);
    } else {
        println!("{}", change);
    }

    return change;
}

pub fn play_hand(
    deck: &mut Deck,
    mut hand: Hand,
    dealer_hand: &Hand,
) -> HandResult {
    let dealer_hand_pts = dealer_hand.calculate_points(true);
    loop {
        let mut player_hand_pts = hand.calculate_points(false);

        // Once a player reaches 21, their turn is over
        if player_hand_pts.calculate_best_value() == 21 {
            return HandResult::Points(player_hand_pts);
        }

        println!("This hand: {} ({})", hand.to_string(false), player_hand_pts.to_string());
        println!("Dealer's hand: {} ({})", dealer_hand.to_string(true), dealer_hand_pts.to_string());

        let mov = get_move();
        match mov {
            Move::Hit => {
                // Add another card to the player's hand
                let card = deck.draw_card();
                hand.add_card(card);
                player_hand_pts = hand.calculate_points(false);
                let best_player_hand_value = player_hand_pts.calculate_best_value();
                println!("You draw {} ({})", card, best_player_hand_value);
                if best_player_hand_value > 21 {
                    println!("Bust!");
                    return HandResult::Bust;
                }
            },
            Move::Stand => {
                return HandResult::Points(player_hand_pts);
            },
            Move::DoubleDown => {
                let card = deck.draw_card();
                hand.add_card(card);
                player_hand_pts = hand.calculate_points(false);
                let best_player_hand_value = player_hand_pts.calculate_best_value();
                println!("You draw {} ({})", card, best_player_hand_value);
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

fn read_num() -> i32 {
    loop {
        let mut input = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut input).expect("Error reading in string input");
        let res = input.trim().parse::<i32>();
        if res.is_ok() {
            return res.unwrap();
        }

        println!("Invalid!");
    }
}
