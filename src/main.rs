use card::{Deck, Hand, HandResult, Points};
use mov::Move;

mod card;
mod mov;
mod blackjack;

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

    

    return 0;
}
